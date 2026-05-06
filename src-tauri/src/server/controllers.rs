use crate::events::clear_entries_received::send_clear_entries_request_to_frontend;
use crate::events::payload_received::send_incoming_payload_to_frontend;
use crate::server::router::respond;
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use hyper::body::Bytes;
use hyper::{Request, Response, StatusCode};
use quo_common::payloads::IncomingQuoPayload;
use tauri::AppHandle;

/// Parses request, emits payload-received event, sends payload to frontend.
pub async fn handle_incoming_payload(
    req: Request<hyper::body::Incoming>,
    app: AppHandle,
    origin: Option<String>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let body_bytes = match req.collect().await {
        Ok(b) => b.to_bytes(),
        Err(e) => {
            return Ok(respond(
                format!("Unparseable payload: {}", e.to_string()),
                StatusCode::BAD_REQUEST,
                origin,
            ));
        }
    };

    if body_bytes.is_empty() {
        return Ok(respond(
            "Empty payload".to_string(),
            StatusCode::BAD_REQUEST,
            origin,
        ));
    }

    let body_str: IncomingQuoPayload = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(e) => {
            return Ok(respond(
                format!(
                    "Could not parse payload into Quo Payload: {}",
                    e.to_string()
                ),
                StatusCode::BAD_REQUEST,
                origin,
            ));
        }
    };

    send_incoming_payload_to_frontend(app, body_str);

    Ok(respond("OK".to_string(), StatusCode::OK, origin))
}

/// Ability to clear all payloads via the API.
/// Useful in case of a UI freeze
pub async fn handle_clear_entries(
    _req: Request<hyper::body::Incoming>,
    app: AppHandle,
    origin: Option<String>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    send_clear_entries_request_to_frontend(app);
    Ok(respond("OK".to_string(), StatusCode::OK, origin))
}
