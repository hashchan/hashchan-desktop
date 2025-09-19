use eframe::egui;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use log::{info, error};

// Import our modules
mod gui;
mod db;
mod reth;

fn main() {
    // Set up logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    
    // Initialize database
    let db = match initialize_database() {
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
    
    // Create a shutdown flag that can be used to signal the Reth node to shut down
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    
    // Start Reth node in a separate thread with larger stack
    let reth_db = db.clone();
    let reth_shutdown = shutdown_flag.clone();
    info!("Starting Reth node in background thread");
    
    // Create a thread with a larger stack size to avoid stack overflow
    let builder = thread::Builder::new()
        .name("reth_node_thread".into())
        .stack_size(8 * 1024 * 1024); // 8 MB stack
        
    let reth_handle = builder.spawn(move || {
        info!("Reth thread started, initializing node...");
        
        // Check the shutdown flag periodically
        let check_shutdown = Arc::new(AtomicBool::new(false));
        let check_shutdown_clone = check_shutdown.clone();
        
        // Spawn a thread to check the shutdown flag
        let _shutdown_checker = thread::spawn(move || {
            while !reth_shutdown.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(100));
            }
            info!("Shutdown signal received, setting shutdown flag");
            check_shutdown_clone.store(true, Ordering::Relaxed);
        });
        
        match reth::indexer::start_reth_node(reth_db, check_shutdown) {
            Ok(_) => info!("Reth node exited normally"),
            Err(e) => error!("Reth node error: {}", e),
        }
        info!("Reth node thread terminated");
    }).unwrap_or_else(|e| {
        error!("Failed to spawn Reth thread: {}", e);
        std::process::exit(1);
    });
    
    // Give the Reth node a moment to start up
    thread::sleep(std::time::Duration::from_millis(500));
    info!("Continuing with GUI initialization...");
    
    // Set up the egui application
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([640.0, 480.0])
            .with_title("HashChan Node"),
        ..Default::default()
    };
    
    info!("Starting GUI application");
    
    // Launch the egui application
    let result = eframe::run_native(
        "HashChan Node",
        options,
        Box::new(|cc| {
            // Create app with database connection
            Ok(Box::new(MyApp::new(cc, db.clone())))
        }),
    );
    
    if let Err(e) = result {
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

// Initialize the SQLite database
fn initialize_database() -> Result<db::HashChanDB, eyre::Error> {
    // Get the data directory
    let app_data_dir = get_app_data_dir()?;
    info!("Using data directory: {:?}", app_data_dir);
    
    // Create the database connection
    let db = db::HashChanDB::from_data_dir(&app_data_dir)?;
    info!("Database connection established");
    
    // Run migrations (tables are created in the HashChanDB::new method)
    info!("Database schema initialized");
    
    Ok(db)
}

// Get the application data directory
fn get_app_data_dir() -> Result<PathBuf, eyre::Error> {
    let app_data_dir = if let Some(proj_dirs) = directories::ProjectDirs::from("com", "hashchan", "node") {
        proj_dirs.data_dir().to_path_buf()
    } else {
        // Fallback to a local directory if we can't get the project directory
        PathBuf::from("data")
    };
    
    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&app_data_dir)?;
    
    Ok(app_data_dir)
}

// Application with database connection and Reth node
struct MyApp {
    db: Arc<Mutex<db::HashChanDB>>,
    db_stats: Option<(i64, i64, i64)>, // (boards, threads, posts)
    reth_status: String,
    reth_peers: usize,
    reth_blocks: u64,
    last_update: std::time::Instant,
    node_started: bool,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>, db: Arc<Mutex<db::HashChanDB>>) -> Self {
        Self {
            db,
            db_stats: None,
            reth_status: "Starting...".to_string(),
            reth_peers: 0,
            reth_blocks: 0,
            last_update: std::time::Instant::now(),
            node_started: true,
        }
    }
    
    fn update_node_status(&mut self) {
        // In a real implementation, we would query the Reth node for status
        // For now, we'll just simulate some status updates
        
        // Simulate connecting to peers over time
        let elapsed_secs = self.last_update.elapsed().as_secs();
        
        if elapsed_secs > 10 && self.reth_peers < 1 {
            self.reth_peers = 1;
            self.reth_status = "Connected to 1 peer".to_string();
        } else if elapsed_secs > 20 && self.reth_peers < 3 {
            self.reth_peers = 3;
            self.reth_status = "Syncing blocks".to_string();
            self.reth_blocks = 100;
        } else if elapsed_secs > 30 && self.reth_blocks < 1000 {
            self.reth_peers = 5;
            self.reth_blocks = 1000;
            self.reth_status = "Synced".to_string();
        }
    }
    
    fn update_stats(&mut self) {
        if let Ok(db) = self.db.lock() {
            if let Ok(stats) = db.get_stats() {
                self.db_stats = Some(stats);
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update stats periodically
        if self.db_stats.is_none() || self.last_update.elapsed().as_secs() > 5 {
            self.update_stats();
            self.update_node_status();
        }
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("Refresh").clicked() {
                        self.update_stats();
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        // Show about dialog
                    }
                });
            });
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("HashChan Node");
            ui.label("Welcome to HashChan Node with Reth and SQLite Database");
            
            // Display Reth node status
            ui.add_space(10.0);
            ui.group(|ui| {
                ui.heading("Reth Node Status");
                ui.label(format!("Status: {}", self.reth_status));
                ui.label(format!("Connected Peers: {}", self.reth_peers));
                ui.label(format!("Synced Blocks: {}", self.reth_blocks));
                ui.label("Contract Address: 0x458c27D5a6421AfAFF435e27E870584Fe03a938F");
                
                if ui.button("Check Node Status").clicked() {
                    self.update_node_status();
                    self.last_update = std::time::Instant::now();
                }
            });
            
            // Display database stats
            ui.add_space(10.0);
            ui.group(|ui| {
                ui.heading("Database Statistics");
                if let Some((boards, threads, posts)) = self.db_stats {
                    ui.label(format!("Boards: {}", boards));
                    ui.label(format!("Threads: {}", threads));
                    ui.label(format!("Posts: {}", posts));
                } else {
                    ui.label("Loading database statistics...");
                }
                
                if ui.button("Refresh Statistics").clicked() {
                    self.update_stats();
                }
            });
        });
        
        // Request repaint every second to update status
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}