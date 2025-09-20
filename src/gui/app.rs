use eframe::egui;
use std::sync::{Arc, Mutex};
use log::{info, error};
use crate::db::HashChanDB;

/// The main application state
pub struct HashChanApp {
    // Database connection
    db: Arc<Mutex<HashChanDB>>,
    // UI state
    show_about: bool,
}

impl HashChanApp {
    /// Create a new instance of the HashChan application
    pub fn new(cc: &eframe::CreationContext<'_>, db: Arc<Mutex<HashChanDB>>) -> Self {
        // Load previous app state if any
        // let app_state: Option<Self> = cc.storage.and_then(|storage| eframe::get_value(storage, eframe::APP_KEY));
        
        Self {
            db,
            show_about: false,
        }
    }
    
    /// Initialize and run the GUI application
    pub fn init_and_run(db: Arc<Mutex<HashChanDB>>) -> Result<(), eframe::Error> {
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
        eframe::run_native(
            "HashChan Node",
            options,
            Box::new(|cc| {
                // Create app with database connection
                Ok(Box::new(HashChanApp::new(cc, db.clone())))
            }),
        )
    }
}

impl eframe::App for HashChanApp {
    /// Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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
            ui.add_space(20.0);
            
            ui.label("Welcome to HashChan Node!");
            ui.label("This is a minimal egui application to get started.");
            ui.add_space(10.0);
            
            if ui.button("Click me!").clicked() {
                // Just a demo button that does nothing yet
                println!("Button clicked!");
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
    }
}
