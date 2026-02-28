mod events;
mod server;

use std::sync::Mutex;
use tauri::Manager;
use quo_common::events::ConnectionEstablishedEvent;
use crate::server::{get_connection_info, setup_server, ServerState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ServerState(Mutex::new(ConnectionEstablishedEvent::default())))
        .invoke_handler(tauri::generate_handler![get_connection_info])
        .setup(|app| {
            setup_server(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri to start up Quo");
}
