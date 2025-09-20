use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use log::{info, error};

// Import our modules
mod gui;
mod db;
mod reth;

fn main() {
    
    // Create a shutdown flag that can be used to signal the Reth node to shut down
    let shutdown_flag = Arc::new(AtomicBool::new(false));

    // Set up logging
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
    
    // Initialize database
    let db = match db::initialize() {
        Ok(db) => {
            info!("Database initialized successfully");
            db
        },
        Err(e) => {
            error!("Failed to initialize database: {}", e);
            std::process::exit(1);
        }
    };
    
    // Wrap database in Arc<Mutex<>> for thread-safe access
    let db = Arc::new(Mutex::new(db));
    
    // Start Reth node in a separate thread
    info!("Starting Reth node in background thread");
    let reth_handle = reth::start_reth_thread(db.clone(), shutdown_flag.clone());
    
    // Give the Reth node a moment to start up
    thread::sleep(std::time::Duration::from_millis(500));
    info!("Continuing with GUI initialization...");
    
    // Initialize and run the GUI application
    if let Err(e) = gui::init_and_run(db.clone()) {
        error!("Error running application: {}", e);
        std::process::exit(1);
    }
    
    // Signal the Reth node to shut down
    info!("GUI application closed, signaling Reth node to shut down...");
    shutdown_flag.store(true, Ordering::Relaxed);
    
    // Wait for the Reth node to shut down with a timeout
    let shutdown_timeout = Duration::from_secs(5);
    let start_time = std::time::Instant::now();
    
    let join_result = reth_handle.join();
    
    if start_time.elapsed() > shutdown_timeout {
        info!("Reth node shutdown took longer than expected, but completed");
    }
    
    if let Err(e) = join_result {
        error!("Error joining Reth thread: {:?}", e);
    } else {
        info!("Reth node shut down successfully");
    }
}


