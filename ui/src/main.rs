use eframe::egui;
use log::{info, error};
use std::sync::Arc;
use tokio::sync::Mutex;

mod app;
mod db;
mod rpc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Set up logging
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
    
    info!("Starting UI service");
    
    // Configure database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@postgres:5432/hashchan".to_string());
    
    // Configure RPC connection
    let reth_api_url = std::env::var("RETH_API_URL")
        .unwrap_or_else(|_| "http://reth:8545".to_string());
    
    // Set up database client
    info!("Connecting to database");
    let db_client = match db::DbClient::new(&database_url).await {
        Ok(client) => {
            info!("Database connection established");
            Arc::new(client)
        },
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            return Err(e);
        }
    };
    
    // Set up RPC client
    let rpc_client = Arc::new(Mutex::new(rpc::RpcClient::new(&reth_api_url)));
    
    // Set up egui application
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([640.0, 480.0])
            .with_title("HashChan Node"),
        ..Default::default()
    };
    
    // Launch the egui application
    eframe::run_native(
        "HashChan Node",
        options,
        Box::new(|cc| {
            // Create app with database and RPC clients
            Box::new(app::HashChanApp::new(cc, db_client, rpc_client))
        }),
    )?;
    
    info!("UI service shutdown complete");
    Ok(())
}
