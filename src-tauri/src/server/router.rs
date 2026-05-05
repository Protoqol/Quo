use crate::server::controllers::handle_incoming_payload;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::{Method, Request, Response, StatusCode};
use tauri::AppHandle;

pub async fn router(
    request: Request<hyper::body::Incoming>,
    app: AppHandle,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let origin = request
        .headers()
        .get("Origin")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    if request.method() == Method::OPTIONS {
        return Ok(respond("".to_string(), StatusCode::OK, origin));
    }

    match (request.method(), request.uri().path()) {
        (&Method::GET, "/") => Ok(respond(
            "Quo is listening".to_string(),
            StatusCode::OK,
            origin,
        )),
        (&Method::POST, "/payload") => handle_incoming_payload(request, app, origin).await,
        _ => Ok(respond(
            "This route does not exist for Quo".to_string(),
            StatusCode::NOT_FOUND,
            origin,
        )),
    }
}

/// Helper function for easier response handling.
pub fn respond(
    s: String,
    status_code: StatusCode,
    origin: Option<String>,
) -> Response<BoxBody<Bytes, hyper::Error>> {
    let mut builder = Response::builder()
        .status(status_code)
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Max-Age", "86400")
        .header("Vary", "Origin");

    if let Some(origin_val) = origin {
        if is_localhost(&origin_val) {
            builder = builder.header("Access-Control-Allow-Origin", origin_val);
        }
    }

    builder.body(full(s)).expect("Valid response body")
}

fn is_localhost(origin: &str) -> bool {
    origin == "http://localhost"
        || origin.starts_with("http://localhost:")
        || origin == "https://localhost"
        || origin.starts_with("https://localhost:")
        || origin == "http://127.0.0.1"
        || origin.starts_with("http://127.0.0.1:")
        || origin == "https://127.0.0.1"
        || origin.starts_with("https://127.0.0.1:")
        || origin == "http://[::1]"
        || origin.starts_with("http://[::1]:")
        || origin == "https://[::1]"
        || origin.starts_with("https://[::1]:")
}

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
