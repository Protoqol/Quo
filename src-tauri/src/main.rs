#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // We need a Tokio runtime to be active on the main thread for some plugins (like Aptabase)
    // that expect a reactor to be available during initialization or when calling synchronous APIs.
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    quo_debugging_client_lib::run()
}
