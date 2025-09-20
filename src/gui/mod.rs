mod app;
mod views;
mod widgets;

pub use app::HashChanApp;

/// Initialize and run the GUI application
pub fn init_and_run(db: std::sync::Arc<std::sync::Mutex<crate::db::HashChanDB>>) -> Result<(), eframe::Error> {
    app::HashChanApp::init_and_run(db)
}
