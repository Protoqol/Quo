mod events;
mod invokers;
mod server;
mod stores;

use crate::invokers::config_invokers::{get_settings, set_setting};
use crate::invokers::file_action_invokers::{
    get_available_editors, open_file, open_in_editor, show_in_explorer,
};
use crate::server::{get_connection_info, setup_server, ServerState};
use crate::stores::stores::check_config;
use quo_common::events::ConnectionEstablishedEvent;
use quo_common::QUO_CONFIG_STORE_NAME;
use std::error::Error;
use std::sync::Mutex;
use tauri::{App, Emitter};
use tauri_plugin_store::StoreExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
        check_config(app.store(QUO_CONFIG_STORE_NAME)?);
        setup_server(app.handle().clone());

        Ok(())
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(ServerState(Mutex::new(
            ConnectionEstablishedEvent::default(),
        )))
        .invoke_handler(tauri::generate_handler![
            get_connection_info,
            open_file,
            show_in_explorer,
            get_available_editors,
            open_in_editor,
            get_settings,
            set_setting
        ])
        .setup(|app| setup(app))
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                let _ = window.emit("app-exit", ());
            }
        })
        .run(tauri::generate_context!())
        .expect("Tauri to start up Quo");
}
