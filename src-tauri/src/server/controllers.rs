use crate::events::payload_received::send_incoming_payload_to_frontend;
use crate::server::router::{full, respond};
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use hyper::body::Bytes;
use hyper::{Request, Response, StatusCode};
use quo_common::payloads::IncomingQuoPayload;
use tauri::AppHandle;

/*
 * Parses request, and emits payload received event to frontend.
 */
pub async fn handle_incoming_payload(
    req: Request<hyper::body::Incoming>,
    app: AppHandle,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let body_bytes = match req.collect().await {
        Ok(b) => b.to_bytes(),
        Err(e) => {
            return Ok(respond(
                format!("Unparseable payload: {}", e.to_string()),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    if body_bytes.is_empty() {
        return Ok(respond(
            "Empty payload".to_string(),
            StatusCode::BAD_REQUEST,
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
            ));
        }
    };

    send_incoming_payload_to_frontend(app, body_str);

    println!("Payload received and sent to frontend");

    Ok(Response::new(full("OK")))
}
