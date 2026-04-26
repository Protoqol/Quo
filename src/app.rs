use crate::atoms::{provide_toast_context, ToastType, Toaster};
use crate::components::{DumpGroup, DumpItem};
use crate::components::SideBar;
use crate::toast;
use codee::string::JsonSerdeCodec;
use leptos::ev;
use leptos::html;
use leptos::serde_json;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::storage::use_local_storage;
use quo_common::events::ConnectionEstablishedEvent;
use quo_common::payloads::IncomingQuoPayload;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> JsValue;
}

/// Full setting DTO — shared between Taskbar, Sidebar, and App.
#[derive(Clone, Debug, Deserialize)]
pub struct SettingDto {
    pub id: String,
    pub category: String,
    pub label: String,
    pub description: String,
    pub show_in_sidebar: bool,
    pub value: serde_json::Value,
}

/// Shared reactive settings that both `App`, `Taskbar`, and `Sidebar` read/write.
#[derive(Clone, Copy)]
pub struct AppSettings {
    /// Master list of all settings — single source of truth.
    pub all_settings: RwSignal<Vec<SettingDto>>,
    pub auto_group: RwSignal<bool>,
    pub long_file_path: RwSignal<bool>,
    pub auto_expand: RwSignal<bool>,
}

impl AppSettings {
    pub fn new() -> Self {
        Self {
            all_settings: RwSignal::new(vec![]),
            auto_group: RwSignal::new(true),
            long_file_path: RwSignal::new(false),
            auto_expand: RwSignal::new(true),
        }
    }
}

/// One entry in the grouped view-model: either a single payload or a
/// group of payloads sharing the same `grouping_hash`.
#[derive(Clone)]
enum DumpEntry {
    Single(IncomingQuoPayload),
    Group(Vec<IncomingQuoPayload>),
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

    // Consume settings from the shared context provided by main.rs
    let settings = use_context::<AppSettings>().expect("AppSettings context missing");
    let auto_group = settings.auto_group;
    let long_file_path = settings.long_file_path;
    let auto_expand = settings.auto_expand;
    let all_settings = settings.all_settings;

    // Load all settings from the Tauri store once on mount and populate the
    // shared signal so Taskbar and Sidebar can both read from it.
    Effect::new(move |_| {
        spawn_local(async move {
            let result = invoke("get_settings", JsValue::NULL).await;
            if let Ok(all) = serde_wasm_bindgen::from_value::<Vec<SettingDto>>(result) {
                // Sync the two dedicated signals first
                if let Some(s) = all.iter().find(|s| s.id == "auto-group-dumps") {
                    if let Some(v) = s.value.as_bool() { auto_group.set(v); }
                }
                if let Some(s) = all.iter().find(|s| s.id == "long-file-path") {
                    if let Some(v) = s.value.as_bool() { long_file_path.set(v); }
                }
                if let Some(s) = all.iter().find(|s| s.id == "auto-expand") {
                    if let Some(v) = s.value.as_bool() { auto_expand.set(v); }
                }
                all_settings.set(all);
            }
        });
    });

    // Compute view-model: grouped by hash if auto-group is on, or always
    // grouped by hash if they are consecutive (for the vertical line).
    let dump_entries = move || {
        let mut sorted = payloads.get().clone();
        sorted.sort_by(|a, b| b.meta.time_epoch_ms.cmp(&a.meta.time_epoch_ms));

        // Group consecutive payloads that share the same `grouping_hash`.
        let mut entries: Vec<DumpEntry> = Vec::new();
        for payload in sorted {
            let hash = payload.meta.variable.grouping_hash.as_deref().unwrap_or("").to_string();
            let merged = if let Some(DumpEntry::Group(ref mut group)) = entries.last_mut() {
                let group_hash = group.first()
                    .and_then(|p| p.meta.variable.grouping_hash.as_deref())
                    .unwrap_or("");
                if !hash.is_empty() && group_hash == hash {
                    group.push(payload.clone());
                    true
                } else {
                    false
                }
            } else {
                false
            };
            if !merged {
                entries.push(DumpEntry::Group(vec![payload]));
            }
        }

        // Unwrap single-item groups back to Single.
        entries
            .into_iter()
            .map(|e| match e {
                DumpEntry::Group(mut v) if v.len() == 1 => DumpEntry::Single(v.remove(0)),
                other => other,
            })
            .collect::<Vec<_>>()
    };

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
                                <path d="M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748Z" />
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
                                            <p class="text-white">Waiting for incoming payloads...</p>
                                            <span class="text-xs text-slate-400 mt-2">
                                                Dumps from your application will appear here automatically.
                                            </span>
                                        </div>
                                    </div>
                                }
                            }
                        >
                            <For
                                each=dump_entries
                                key=|entry| match entry {
                                    DumpEntry::Single(p) => p.meta.uid.clone(),
                                    DumpEntry::Group(g) => g
                                        .iter()
                                        .map(|p| p.meta.uid.as_str())
                                        .collect::<Vec<_>>()
                                        .join(","),
                                }
                                children=move |entry| {
                                    match entry {
                                        DumpEntry::Single(payload) => view! {
                                            <DumpItem
                                                dump=payload
                                                on_delete=Callback::new(delete_payload)
                                                long_file_path=Signal::from(long_file_path)
                                                auto_expand=Signal::from(auto_expand)
                                            />
                                        }.into_any(),
                                        DumpEntry::Group(dumps) => view! {
                                            <DumpGroup
                                                dumps=dumps
                                                on_delete=Callback::new(delete_payload)
                                                long_file_path=Signal::from(long_file_path)
                                                auto_group=Signal::from(auto_group)
                                                auto_expand=Signal::from(auto_expand)
                                            />
                                        }.into_any(),
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
