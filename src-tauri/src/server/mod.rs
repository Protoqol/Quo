pub mod controllers;
pub mod router;

use crate::events::connection_established::send_connection_info_to_frontend;
use crate::server::router::router;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use hyper_util::rt::TokioIo;
use port_check::{free_local_port_in_range, is_port_reachable};
use quo_common::events::ConnectionEstablishedEvent;
use std::net::SocketAddr;
use tauri::AppHandle;
use tokio::net::TcpListener;

/*
 * Sets up the listener to listen for quo calls.
 */
pub fn setup_server(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let port_to_use = free_local_port_in_range(7312..=7400).unwrap();
        let host = [127, 0, 0, 1];

        // @TODO custom host
        let addr = SocketAddr::from((host, port_to_use));
        let listener = TcpListener::bind(addr).await.unwrap();

        println!(
            "Quo server listening on http://{}:{}",
            addr.ip(),
            addr.port()
        );

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let io = TokioIo::new(stream);
            let app = app.clone();

            tauri::async_runtime::spawn(async move {
                let app_copy = app.clone();

                // @TODO http2
                let server = http1::Builder::new().serve_connection(
                    io,
                    service_fn(move |request: Request<hyper::body::Incoming>| {
                        let app = app.clone();

                        async move { router(request, app.clone()).await }
                    }),
                );

                send_connection_info_to_frontend(
                    app_copy.clone(),
                    ConnectionEstablishedEvent {
                        host: "127.0.0.1".to_string(),
                        port: port_to_use,
                        success: true,
                    },
                );

                // Wait for the connection to finish
                match server.await {
                    Ok(_) => {
                        println!(
                            "Server connection ended successfully on {}:{}",
                            addr.ip(),
                            addr.port()
                        );
                    }
                    Err(e) => println!("Server connection error: {}", e.to_string()),
                }
            });
        }
    });
}
