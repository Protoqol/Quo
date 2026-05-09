use quo_common::payloads::IncomingQuoPayload;
use quo_common::QUO_CONFIG_STORE_NAME;
use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::{NotificationExt};
use tauri_plugin_store::StoreExt;

/*
 * Payload received event `payload-received`.
 */
#[tauri::command]
pub fn send_incoming_payload_to_frontend(app: AppHandle, data: IncomingQuoPayload) {
    let _ = app.emit("payload-received", &data);

    if let Ok(store) = app.store(QUO_CONFIG_STORE_NAME) {
        let enabled = store
            .get("notifications")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if enabled {
            tauri::async_runtime::spawn(async move {
                app.notification()
                    .builder()
                    .title("Quo")
                    .body(format!(
                        "Received a new payload from {}",
                        &data.meta.origin
                    ))
                    .show()
                    .unwrap();
            });
        }
    }
}
