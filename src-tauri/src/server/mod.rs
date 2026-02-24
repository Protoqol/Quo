use crate::events::payload_received::send_to_frontend;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use quo_common::payloads::IncomingQuoPayload;
use std::net::SocketAddr;
use tauri::AppHandle;
use tokio::net::TcpListener;

/*
 * Parses request, and emits payload received event to frontend.
 */
async fn handle_request(
    req: Request<hyper::body::Incoming>,
    app: AppHandle,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let body_bytes = match req.collect().await {
        Ok(b) => b.to_bytes(),
        Err(e) => {
            eprintln!("Error collecting body: {:?}", e);
            return Ok(Response::builder()
                .status(400)
                .body(Full::new(Bytes::from("Error collecting body")))
                .unwrap());
        }
    };

    let body_str: IncomingQuoPayload = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing payload: {:?}", e);
            return Ok(Response::builder()
                .status(400)
                .body(Full::new(Bytes::from(format!("Error parsing payload: {}", e))))
                .unwrap());
        }
    };
    // println!("Received payload (decoded): {}", body_str.get_raw_payload());

    send_to_frontend(app, body_str);

    Ok(Response::new(Full::new(Bytes::from("OK"))))
}

/*
 * Sets up the listener to listen on port 7312.
 */
pub fn setup_server(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], 7312));
        let listener = TcpListener::bind(addr).await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let io = TokioIo::new(stream);
            let app = app.clone();

            tauri::async_runtime::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(
                        io,
                        service_fn(move |request: Request<hyper::body::Incoming>| {
                            let app = app.clone();

                            async move { handle_request(request, app).await }
                        }),
                    )
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    });
}
