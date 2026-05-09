use tauri::{AppHandle, Emitter};

/*
 * Clear entries request event `clear-entries`.
 */
#[tauri::command]
pub fn send_clear_entries_request_to_frontend(app: AppHandle) {
    let _ = app.emit("clear-entries", None::<&str>);
}
