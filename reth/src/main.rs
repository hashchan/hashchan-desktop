use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use log::{info, error};
use tokio::signal;

mod indexer;
mod db;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Set up logging
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
    
    info!("Starting Reth service");
    
    // Create a shutdown flag
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let shutdown_flag_clone = shutdown_flag.clone();
    
    // Handle shutdown signals
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                info!("Shutdown signal received");
                shutdown_flag_clone.store(true, Ordering::Relaxed);
            },
            Err(err) => {
                error!("Error setting up signal handler: {}", err);
            }
        }
    });
    
    // Set up database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@postgres:5432/hashchan".to_string());
    
    info!("Connecting to database");
    let db_client = match db::DbClient::new(&database_url).await {
        Ok(client) => {
            info!("Database connection established");
            client
        },
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            return Err(e);
        }
    };
    
    // Verify database schema
    if let Err(e) = db_client.verify_schema().await {
        error!("Failed to verify database schema: {}", e);
        return Err(e);
    }
    
    // Start the Reth node in a separate thread
    info!("Starting Reth node thread");
    let db_client = Arc::new(std::sync::Mutex::new(db_client));
    let _reth_thread = indexer::start_reth_thread(db_client, shutdown_flag.clone());
    
    // Wait for shutdown signal
    while !shutdown_flag.load(Ordering::Relaxed) {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    
    // Wait for any remaining tasks to complete
    tokio::time::sleep(Duration::from_millis(500)).await;
    info!("Reth service shutdown complete");
    
    Ok(())
}
