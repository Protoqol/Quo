use quo_common::payloads::IncomingQuoPayload;
use quo_common::QUO_CONFIG_STORE_NAME;
use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::{NotificationExt};
use tauri_plugin_store::StoreExt;

/*
 * Clear entries request event `clear-entries`.
 */
#[tauri::command]
pub fn send_clear_entries_request_to_frontend(app: AppHandle) {
    let _ = app.emit("clear-entries", None::<&str>);
}
