use crate::atoms::{provide_toast_context, ToastType, Toaster};
use crate::components::DumpItem;
use crate::components::SideBar;
use crate::toast;
use codee::string::JsonSerdeCodec;
use leptos::ev;
use leptos::html;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::storage::use_local_storage;
use quo_common::events::ConnectionEstablishedEvent;
use quo_common::payloads::IncomingQuoPayload;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    provide_toast_context();

    let search_input_ref = NodeRef::<html::Input>::new();

    let (payloads, set_payloads, _) =
        use_local_storage::<Vec<IncomingQuoPayload>, JsonSerdeCodec>("payloads");

    let (server_host, set_server_host, _) =
        use_local_storage::<String, JsonSerdeCodec>("server_host");
    let (server_port, set_server_port, _) =
        use_local_storage::<String, JsonSerdeCodec>("server_port");

    let delete_payload = move |uid: String| {
        let backup = payloads.get_untracked();
        let mut current = payloads.get_untracked();
        current.retain(|p| p.meta.uid != uid);
        let to_compare = current.clone();
        set_payloads.set(current);

        if (backup.len() - 1) == to_compare.len() {
            toast!("Dump was deleted", ToastType::Success)
        } else {
            // @TODO check why
            toast!("Dump not deleted", ToastType::Error)
        }
    };

    window_event_listener(ev::keydown, move |ev| {
        if ev.key() == "/" {
            if let Some(search_input) = search_input_ref.get() {
                ev.prevent_default();
                let _ = search_input.focus();
            }
        }
    });

    Effect::new(move |_| {
        let handle_payload_received_event = Closure::wrap(Box::new(move |event_obj: JsValue| {
            #[derive(serde::Deserialize)]
            struct TauriEvent<T> {
                payload: T,
            }

            match serde_wasm_bindgen::from_value::<TauriEvent<IncomingQuoPayload>>(event_obj) {
                Ok(event) => {
                    println!("{}", event.payload.meta.sender_origin);
                    let mut current = payloads.get_untracked();
                    current.insert(0, event.payload);
                    set_payloads.set(current);
                }
                Err(_e) => {
                    // @TODO error handle
                    println!("Could not store incoming payload");
                }
            };
        }) as Box<dyn FnMut(JsValue)>);

        let handle_connection_established = Closure::wrap(Box::new(move |event_obj: JsValue| {
            #[derive(serde::Deserialize)]
            struct TauriEvent<T> {
                payload: T,
            }

            match serde_wasm_bindgen::from_value::<TauriEvent<ConnectionEstablishedEvent>>(
                event_obj,
            ) {
                Ok(event) => {
                    let ConnectionEstablishedEvent {
                        host,
                        port,
                        success,
                    } = event.payload;

                    set_server_host.set(host);
                    set_server_port.set(port.to_string());

                    if success {
                        console_log("Connection established")
                    } else {
                        console_log("Connection NOT established")
                    }
                }
                Err(_e) => {
                    // @TODO error handle
                    println!("Could not handle event `connection-established`");
                }
            };
        }) as Box<dyn FnMut(JsValue)>);

        // When app is closed remove server_host & server_port from localstorage to prevent being out-of-date
        let handle_app_exit = Closure::wrap(Box::new(move |_obj: JsValue| {
            let _ = window()
                .local_storage()
                .unwrap()
                .unwrap()
                .remove_item("server_host");
            let _ = window()
                .local_storage()
                .unwrap()
                .unwrap()
                .remove_item("server_port");
        }) as Box<dyn FnMut(JsValue)>);

        spawn_local(async move {
            listen("payload-received", &handle_payload_received_event).await;
            listen("connection-established", &handle_connection_established).await;
            listen("app-exit", &handle_app_exit).await;

            // Fetch initial connection info after listeners are set up
            let connection_info = invoke("get_connection_info", JsValue::NULL).await;
            if !connection_info.is_null() && !connection_info.is_undefined() {
                if let Ok(event) =
                    serde_wasm_bindgen::from_value::<ConnectionEstablishedEvent>(connection_info)
                {
                    let ConnectionEstablishedEvent {
                        host,
                        port,
                        success,
                    } = event;

                    set_server_host.set(host);
                    set_server_port.set(port.to_string());

                    if success {
                        console_log("Initial connection info loaded")
                    }
                }
            }

            handle_payload_received_event.forget();
            handle_connection_established.forget();
            handle_app_exit.forget();
        });
    });

    view! {
        <div class="quo-layout">
            <Toaster />
            <SideBar server_host server_port />
            <main class="quo-main">
                <header class="quo-main-header">
                    <div class="input-container">
                        <label for="search">
                            <svg
                                class="search-icon"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                width="16"
                                height="16"
                                fill="currentColor"
                            >
                                <path d="M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748Z"></path>
                            </svg>
                            <input
                                type="text"
                                id="search"
                                node_ref=search_input_ref
                                placeholder="Search payloads... (Press '/' to focus)"
                            />
                        </label>
                        <span id="searchResult"></span>
                    </div>
                </header>
                <div class="quo-body">
                    <div id="quo">
                        <Show
                            when=move || !payloads.get().is_empty()
                            fallback=|| {
                                view! {
                                    <div id="quoNoRequestsMessage">
                                        <div class="empty-state">
                                            <img
                                                src="/public/assets/icons/boat-animation.apng"
                                                class="w-32"
                                            />
                                            <p>Waiting for incoming payloads...</p>
                                            <span class="text-xs text-slate-400 mt-2">
                                                Dumps from your application will appear here automatically.
                                            </span>
                                        </div>
                                    </div>
                                }
                            }
                        >
                            <For
                                each=move || {
                                    let mut sorted_payloads = payloads.get().clone();
                                    sorted_payloads
                                        .sort_by(|a, b| {
                                            b.meta.time_epoch_ms.cmp(&a.meta.time_epoch_ms)
                                        });
                                    sorted_payloads.into_iter()
                                }
                                key=|payload| payload.meta.uid.clone()
                                children=move |payload: IncomingQuoPayload| {
                                    view! {
                                        <DumpItem
                                            dump=payload
                                            on_delete=Callback::new(delete_payload)
                                        />
                                    }
                                }
                            />
                        </Show>
                    </div>
                </div>
            </main>
        </div>
    }
}
