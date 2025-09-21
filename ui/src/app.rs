use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;
use poll_promise::Promise;
use log::{info, error};
use serde_json::Value;

use crate::db::{DbClient, Board, Thread, Post};
use crate::rpc::{RpcClient, RpcError};

/// The main application state
pub struct HashChanApp {
    // Clients
    db_client: Arc<DbClient>,
    rpc_client: Arc<Mutex<RpcClient>>,
    
    // UI state
    show_about: bool,
    boards: Option<Promise<Result<Vec<Board>, String>>>,
    selected_board: Option<i64>,
    threads: Option<Promise<Result<Vec<Thread>, String>>>,
    selected_thread: Option<Vec<u8>>,
    posts: Option<Promise<Result<Vec<Post>, String>>>,
    
    // Node status
    node_status: Option<Promise<Result<NodeStatus, String>>>,
}

struct NodeStatus {
    block_number: u64,
    peer_count: u64,
    syncing: Value,
}

impl HashChanApp {
    /// Create a new instance of the HashChan application
    pub fn new(cc: &eframe::CreationContext<'_>, db_client: Arc<DbClient>, rpc_client: Arc<Mutex<RpcClient>>) -> Self {
        // Load previous app state if any
        // let app_state: Option<Self> = cc.storage.and_then(|storage| eframe::get_value(storage, eframe::APP_KEY));
        
        Self {
            db_client,
            rpc_client,
            show_about: false,
            boards: None,
            selected_board: None,
            threads: None,
            selected_thread: None,
            posts: None,
            node_status: None,
        }
    }
    
    fn load_boards(&mut self) {
        let db_client = self.db_client.clone();
        self.boards = Some(Promise::spawn_async(async move {
            match db_client.get_boards().await {
                Ok(boards) => Ok(boards),
                Err(e) => Err(format!("Error loading boards: {}", e)),
            }
        }));
    }
    
    fn load_threads(&mut self, board_id: i64) {
        let db_client = self.db_client.clone();
        self.threads = Some(Promise::spawn_async(async move {
            match db_client.get_threads(board_id).await {
                Ok(threads) => Ok(threads),
                Err(e) => Err(format!("Error loading threads: {}", e)),
            }
        }));
    }
    
    fn load_posts(&mut self, thread_id: Vec<u8>) {
        let db_client = self.db_client.clone();
        let thread_id_clone = thread_id.clone();
        self.posts = Some(Promise::spawn_async(async move {
            match db_client.get_posts(&thread_id_clone).await {
                Ok(posts) => Ok(posts),
                Err(e) => Err(format!("Error loading posts: {}", e)),
            }
        }));
    }
    
    fn update_node_status(&mut self) {
        let rpc_client = self.rpc_client.clone();
        self.node_status = Some(Promise::spawn_async(async move {
            let mut client = rpc_client.lock().await;
            
            let block_number = match client.eth_block_number().await {
                Ok(n) => n,
                Err(e) => return Err(format!("Failed to get block number: {}", e)),
            };
            
            let peer_count = match client.net_peer_count().await {
                Ok(n) => n,
                Err(e) => return Err(format!("Failed to get peer count: {}", e)),
            };
            
            let syncing = match client.eth_syncing().await {
                Ok(s) => s,
                Err(e) => return Err(format!("Failed to get sync status: {}", e)),
            };
            
            Ok(NodeStatus {
                block_number,
                peer_count,
                syncing,
            })
        }));
    }
}

impl eframe::App for HashChanApp {
    /// Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load boards if not already loaded
        if self.boards.is_none() {
            self.load_boards();
        }
        
        // Update node status periodically
        if self.node_status.is_none() {
            self.update_node_status();
        }
        
        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("Refresh").clicked() {
                        if let Some(board_id) = self.selected_board {
                            self.load_threads(board_id);
                        } else {
                            self.load_boards();
                        }
                        self.update_node_status();
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                    }
                });
            });
        });
        
        // Main central area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("HashChan Node");
            
            // Display node status
            ui.group(|ui| {
                ui.heading("Node Status");
                if let Some(status_promise) = &self.node_status {
                    match status_promise.ready() {
                        Some(Ok(status)) => {
                            ui.label(format!("Block Number: {}", status.block_number));
                            ui.label(format!("Connected Peers: {}", status.peer_count));
                            
                            let sync_status = if status.syncing.is_boolean() && !status.syncing.as_bool().unwrap_or(false) {
                                "Synced".to_string()
                            } else {
                                "Syncing...".to_string()
                            };
                            ui.label(format!("Sync Status: {}", sync_status));
                        },
                        Some(Err(err)) => {
                            ui.label(format!("Error: {}", err));
                        },
                        None => {
                            ui.spinner();
                            ui.label("Loading node status...");
                        },
                    }
                } else {
                    ui.spinner();
                    ui.label("Loading node status...");
                }
                
                if ui.button("Refresh Status").clicked() {
                    self.update_node_status();
                }
            });
            
            ui.add_space(20.0);
            
            // Display boards
            if let Some(boards_promise) = &self.boards {
                match boards_promise.ready() {
                    Some(Ok(boards)) => {
                        ui.heading("Boards");
                        ui.add_space(10.0);
                        
                        for board in boards {
                            if ui.button(&format!("/{}/: {}", board.symbol, board.name)).clicked() {
                                self.selected_board = Some(board.board_id);
                                self.load_threads(board.board_id);
                            }
                        }
                    },
                    Some(Err(err)) => {
                        ui.label(format!("Error: {}", err));
                    },
                    None => {
                        ui.spinner();
                        ui.label("Loading boards...");
                    },
                }
            }
            
            // Display threads if a board is selected
            if let Some(threads_promise) = &self.threads {
                match threads_promise.ready() {
                    Some(Ok(threads)) => {
                        ui.add_space(20.0);
                        ui.heading("Threads");
                        ui.add_space(10.0);
                        
                        for thread in threads {
                            if ui.button(&thread.title).clicked() {
                                self.selected_thread = Some(thread.thread_id.clone());
                                self.load_posts(thread.thread_id.clone());
                            }
                        }
                    },
                    Some(Err(err)) => {
                        ui.label(format!("Error: {}", err));
                    },
                    None => {
                        ui.spinner();
                        ui.label("Loading threads...");
                    },
                }
            }
            
            // Display posts if a thread is selected
            if let Some(posts_promise) = &self.posts {
                match posts_promise.ready() {
                    Some(Ok(posts)) => {
                        ui.add_space(20.0);
                        ui.heading("Posts");
                        ui.add_space(10.0);
                        
                        for post in posts {
                            ui.group(|ui| {
                                let creator_hex = hex::encode(&post.creator);
                                ui.label(format!("From: 0x{}...", &creator_hex[0..8]));
                                if let Some(content) = &post.content {
                                    ui.label(content);
                                }
                                if let Some(img_url) = &post.img_url {
                                    ui.label(format!("Image: {}", img_url));
                                }
                            });
                        }
                    },
                    Some(Err(err)) => {
                        ui.label(format!("Error: {}", err));
                    },
                    None => {
                        ui.spinner();
                        ui.label("Loading posts...");
                    },
                }
            }
        });
        
        // About dialog
        if self.show_about {
            egui::Window::new("About HashChan Node")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("HashChan Node");
                    ui.label("Version: 0.1.0");
                    ui.label("A specialized ultra-light Ethereum node that indexes HashChan3 events");
                    ui.add_space(10.0);
                    if ui.button("Close").clicked() {
                        self.show_about = false;
                    }
                });
        }
        
        // Request repaint to keep UI responsive
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}