pub mod controllers;
pub mod router;

use crate::events::connection_established::send_connection_info_to_frontend;
use crate::server::router::router;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use hyper_util::rt::TokioIo;
use port_check::{free_local_port_in_range};
use quo_common::events::ConnectionEstablishedEvent;
use std::net::SocketAddr;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::State;
use tokio::net::TcpListener;

pub struct ServerState(pub Mutex<ConnectionEstablishedEvent>);

#[tauri::command]
pub fn get_connection_info(state: State<'_, ServerState>) -> ConnectionEstablishedEvent {
    state.0.lock().unwrap().clone()
}

/*
 * Sets up the listener to listen for quo calls.
 */
pub fn setup_server(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        // @TODO custom host
        let host = [127, 0, 0, 1];
        let port_to_use = free_local_port_in_range(7312..=7400).unwrap();

        let addr = SocketAddr::from((host, port_to_use));
        let listener = TcpListener::bind(addr).await.unwrap();

        let event_data = ConnectionEstablishedEvent {
            host: addr.ip().to_string(),
            port: addr.port(),
            success: true,
        };

        // Update global state
        {
            use tauri::Manager;
            let state = app.state::<ServerState>();
            let mut state_inner = state.0.lock().unwrap();
            *state_inner = event_data.clone();
        }

        // Send event immediately (no sleep)
        send_connection_info_to_frontend(app.clone(), event_data);

        println!(
            "Quo server listening on http://{}:{}",
            addr.ip(),
            addr.port()
        );

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let io = TokioIo::new(stream);
            let app_loop = app.clone();

            tauri::async_runtime::spawn(async move {
                let app_inner = app_loop.clone();

                // @TODO http2
                let server = http1::Builder::new().serve_connection(
                    io,
                    service_fn(move |request: Request<hyper::body::Incoming>| {
                        let app = app_inner.clone();

                        async move { router(request, app.clone()).await }
                    }),
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
