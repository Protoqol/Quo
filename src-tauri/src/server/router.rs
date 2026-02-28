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
    match (request.method(), request.uri().path()) {
        (&Method::GET, "/") => Ok(respond("Quo is listening".to_string(), StatusCode::OK)),
        (&Method::POST, "/payload") => handle_incoming_payload(request, app).await,
        _ => Ok(respond(
            "This route does not exist for Quo".to_string(),
            StatusCode::NOT_FOUND,
        )),
    }
}

/// Helper function for easier response handling.
pub fn respond(s: String, status_code: StatusCode) -> Response<BoxBody<Bytes, hyper::Error>> {
    Response::builder()
        .status(status_code)
        .body(full(s))
        .expect("Valid response body")
}

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
