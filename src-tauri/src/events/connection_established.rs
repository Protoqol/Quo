use quo_common::events::ConnectionEstablishedEvent;
use tauri::{AppHandle, Emitter};
/*
 * Server connection established, `connection-established` event.
 */
#[tauri::command]
pub fn send_connection_info_to_frontend(app: AppHandle, data: ConnectionEstablishedEvent) {
    println!("Sending connection-established event");
    let _ = app.emit("connection-established", &data);
}
