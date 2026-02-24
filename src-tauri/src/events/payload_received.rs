use quo_common::payloads::IncomingQuoPayload;
use tauri::{AppHandle, Emitter};

/*
 * Payload received event `payload-received`.
 */
#[tauri::command]
pub fn send_to_frontend(app: AppHandle, data: IncomingQuoPayload) {
    app.emit("payload-received", &data).unwrap();
}
