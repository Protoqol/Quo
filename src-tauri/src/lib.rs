mod events;
mod server;

use crate::server::{get_connection_info, setup_server, ServerState};
use quo_common::events::ConnectionEstablishedEvent;
use std::sync::Mutex;
use tauri::{Emitter};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ServerState(Mutex::new(
            ConnectionEstablishedEvent::default(),
        )))
        .invoke_handler(tauri::generate_handler![get_connection_info])
        .setup(|app| {
            setup_server(app.handle().clone());
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                let _ = window.emit("app-exit", ());
            }
        })
        .run(tauri::generate_context!())
        .expect("Tauri to start up Quo");
}
