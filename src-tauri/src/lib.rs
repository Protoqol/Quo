mod events;
mod server;

use tauri::Manager;
use crate::server::setup_server;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            setup_server(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri to start up Quo");
}
