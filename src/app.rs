use crate::atoms::{provide_toast_context, ToastType, Toaster};
use crate::components::{DumpGroup, DumpItem};
use crate::components::SideBar;
use crate::toast;
use crate::utils::formatter::format_by_language;
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
    pub truncate_large_var_types: RwSignal<bool>,
}

impl AppSettings {
    pub fn new() -> Self {
        Self {
            all_settings: RwSignal::new(vec![]),
            auto_group: RwSignal::new(true),
            long_file_path: RwSignal::new(false),
            auto_expand: RwSignal::new(true),
            truncate_large_var_types: RwSignal::new(false),
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

    let (search_query, set_search_query) = signal(String::new());

    let (expand_all_command, set_expand_all_command) = signal(0usize);
    let (collapse_all_command, set_collapse_all_command) = signal(0usize);

    let (server_host, set_server_host, _) =
        use_local_storage::<String, JsonSerdeCodec>("server_host");
    let (server_port, set_server_port, _) =
        use_local_storage::<String, JsonSerdeCodec>("server_port");

    // Consume settings from the shared context provided by main.rs
    let settings = use_context::<AppSettings>().expect("AppSettings context missing");
    let auto_group = settings.auto_group;
    let long_file_path = settings.long_file_path;
    let auto_expand = settings.auto_expand;
    let truncate_large_var_types = settings.truncate_large_var_types;
    let all_settings = settings.all_settings;

    let (is_all_expanded, set_is_all_expanded) = signal(auto_expand.get_untracked());

    let (is_diff_mode, set_is_diff_mode) = signal(false);
    let (selected_payload_uids, set_selected_payload_uids) = signal::<Vec<String>>(vec![]);
    let (diff_result, set_diff_result) = signal::<Option<String>>(None);
    let (show_diff_modal, set_show_diff_modal) = signal(false);

    let toggle_payload_selection = move |uid: String| {
        set_selected_payload_uids.update(|uids| {
            if let Some(pos) = uids.iter().position(|u| u == &uid) {
                uids.remove(pos);
            } else if uids.len() < 2 {
                uids.push(uid);
            } else {
                toast!(
                    "You can only select up to 2 payloads to diff.",
                    ToastType::Warning
                );
            }
        });
    };

    let perform_diff = move |_| {
        let uids = selected_payload_uids.get();
        if uids.len() != 2 {
            return;
        }

        let all_payloads = payloads.get();
        let p1 = all_payloads.iter().find(|p| p.meta.uid == uids[0]);
        let p2 = all_payloads.iter().find(|p| p.meta.uid == uids[1]);

        if let (Some(p1), Some(p2)) = (p1, p2) {
            let s1 = format_by_language(p1, false);
            let s2 = format_by_language(p2, false);

            spawn_local(async move {
                let diff = invoke(
                    "get_diff_for_snippets",
                    serde_wasm_bindgen::to_value(&serde_json::json!({ "first": s1, "second": s2 }))
                        .unwrap(),
                )
                .await;
                if let Ok(diff_str) = serde_wasm_bindgen::from_value::<String>(diff) {
                    set_diff_result.set(Some(diff_str));
                    set_show_diff_modal.set(true);
                }
            });
        }
    };

    Effect::new(move |_| {
        set_is_all_expanded.set(auto_expand.get());
    });

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
                if let Some(s) = all.iter().find(|s| s.id == "truncate-large-var-types") {
                    if let Some(v) = s.value.as_bool() { truncate_large_var_types.set(v); }
                }
                all_settings.set(all);
            }
        });
    });

    let filtered_payloads = move || {
        let query = search_query.get().to_lowercase();
        let mut all = payloads.get().clone();
        if !query.is_empty() {
            all.retain(|p| {
                p.meta.variable.var_type.to_lowercase().contains(&query)
                    || p.meta.variable.name.to_lowercase().contains(&query)
                    || p.meta.variable.value.to_lowercase().contains(&query)
                    || p.meta.origin.to_lowercase().contains(&query)
                    || p.meta.variable.memory_address
                        .as_ref()
                        .map(|addr| addr.to_lowercase().contains(&query))
                        .unwrap_or(false)
            });
        }
        all
    };

    // Compute view-model: grouped by hash if auto-group is on, or always
    // grouped by hash if they are consecutive (for the vertical line).
    let dump_entries = move || {
        let mut sorted = filtered_payloads();
        sorted.sort_by(|a, b| {
            b.meta
                .time_epoch_ms
                .cmp(&a.meta.time_epoch_ms)
                .then_with(|| b.meta.id.cmp(&a.meta.id))
        });

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

        // Within each group, sort by ID descending.
        for entry in &mut entries {
            if let DumpEntry::Group(ref mut group) = entry {
                group.sort_by(|a, b| b.meta.id.cmp(&a.meta.id));
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

    let has_expandable_items = Memo::new(move |_| {
        let entries = dump_entries();
        let auto_group_enabled = auto_group.get();
        let truncate = truncate_large_var_types.get();

        entries.iter().any(|entry| match entry {
            DumpEntry::Single(payload) => {
                format_by_language(payload, truncate).lines().count() > 6
            }
            DumpEntry::Group(payloads) => {
                auto_group_enabled
                    || payloads
                        .iter()
                        .any(|p| format_by_language(p, truncate).lines().count() > 6)
            }
        })
    });

    let search_results_count = move || {
        let query = search_query.get();

        if query.is_empty() {
            return String::new();
        }

        let count = filtered_payloads().len();

        if count == 1 {
            "1 result".to_string()
        } else {
            format!("{} results", count)
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
                                autocomplete="off"
                                autocapitalize="none"
                                spellcheck="false"
                                on:input=move |ev| {
                                    set_search_query.set(event_target_value(&ev));
                                }
                                prop:value=search_query
                            />
                            <Show when=move || !search_query.get().is_empty()>
                                <button
                                    class="clear-button"
                                    on:click=move |_| {
                                        set_search_query.set(String::new());
                                        if let Some(input) = search_input_ref.get() {
                                            let _ = input.focus();
                                        }
                                    }
                                >
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 24 24"
                                        width="16"
                                        height="16"
                                        fill="currentColor"
                                    >
                                        <path d="M12 10.586L16.95 5.63605L18.364 7.05026L13.414 12L18.364 16.9498L16.95 18.364L12 13.414L7.05026 18.364L5.63605 16.95L10.586 12L5.63605 7.05026L7.05026 5.63605L12 10.586Z" />
                                    </svg>
                                </button>
                            </Show>
                        </label>
                    </div>

                </header>
                <div class="quo-body">
                    <div id="quo" class="relative">
                        <Show when=move || is_diff_mode.get() && selected_payload_uids.get().len() == 2>
                            <div class="absolute top-4 left-4 z-[70] animate-bounce-in">
                                <button
                                    class="bg-accent text-slate-950 px-4 py-2 rounded-full font-bold shadow-xl flex items-center gap-2 hover:scale-110 transition-all"
                                    on:click=perform_diff
                                >
                                    "Diff selected payloads"
                                    <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M16 3h5v5M4 20L21 3M21 16v5h-5M15 15l6 6M4 4l5 5"></path>
                                    </svg>
                                </button>
                            </div>
                        </Show>
                        <div class="flex items-center justify-between -mt-4 -mb-2">
                            <Show
                                when=move || !search_results_count().is_empty()
                                fallback=|| {
                                    view! {
                                        <span id="searchResult" class="opacity-0">-</span>
                                    }
                                }
                                >
                                <span id="searchResult" class="text-slate-600 text-xs font-bold uppercase tracking-wider">{search_results_count}</span>
                            </Show>

                            <Show when=move || !payloads.get().is_empty()>
                                <div class="flex items-center gap-x-2">
                                    <Show when=move || has_expandable_items.get()>
                                        <button
                                            class="text-xs font-bold uppercase tracking-wider text-slate-500 hover:text-accent transition-colors flex items-center gap-1.5 p-2 rounded-lg hover:bg-slate-800"
                                            on:click=move |_| {
                                                if is_all_expanded.get() {
                                                    set_collapse_all_command.update(|v| *v += 1);
                                                    set_is_all_expanded.set(false);
                                                } else {
                                                    set_expand_all_command.update(|v| *v += 1);
                                                    set_is_all_expanded.set(true);
                                                }
                                            }
                                            title=move || if is_all_expanded.get() { "Collapse all" } else { "Expand all" }
                                        >
                                            <Show
                                                when=move || is_all_expanded.get()
                                                fallback=move || view! {
                                                    "Expand all"
                                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
                                                        <path d="M12 13.172l4.95-4.95 1.414 1.414L12 16 5.636 9.636 7.05 8.222z"/>
                                                        <path d="M12 18.172l4.95-4.95 1.414 1.414L12 21 5.636 14.636 7.05 13.222z"/>
                                                    </svg>
                                                }
                                            >
                                                "Collapse all"
                                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
                                                    <path d="M12 10.828l-4.95 4.95-1.414-1.414L12 8l6.364 6.364-1.414 1.414z"/>
                                                    <path d="M12 5.828l-4.95 4.95-1.414-1.414L12 3l6.364 6.364-1.414 1.414z"/>
                                                </svg>
                                            </Show>
                                        </button>
                                    </Show>
                                    <button
                                        class=move || format!(
                                            "text-xs font-bold uppercase tracking-wider transition-colors flex items-center gap-1.5 p-2 rounded-lg {}",
                                            if is_diff_mode.get() { "text-accent bg-slate-800" } else { "text-slate-500 hover:text-accent hover:bg-slate-800" }
                                        )
                                        on:click=move |_| {
                                            set_is_diff_mode.update(|v| *v = !*v);
                                            if !is_diff_mode.get() {
                                                set_selected_payload_uids.set(vec![]);
                                            }
                                        }
                                        title="Diff payloads"
                                    >
                                        "Diff payloads"
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
                                            <path d="M16 3h5v5M4 20L21 3M21 16v5h-5M15 15l6 6M4 4l5 5"></path>
                                        </svg>
                                    </button>
                                </div>
                            </Show>
                        </div>
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
                                        DumpEntry::Single(payload) => {
                                            let uid_for_select = payload.meta.uid.clone();
                                            let uid_for_on_select = payload.meta.uid.clone();
                                            view! {
                                                <DumpItem
                                                    dump=payload
                                                    on_delete=Callback::new(delete_payload)
                                                    long_file_path=Signal::from(long_file_path)
                                                    auto_expand=Signal::from(auto_expand)
                                                    truncate_large_var_types=Signal::from(truncate_large_var_types)
                                                    expand_all_command=expand_all_command
                                                    collapse_all_command=collapse_all_command
                                                    is_selectable=is_diff_mode
                                                    is_selected=Signal::derive(move || selected_payload_uids.get().contains(&uid_for_select))
                                                    on_select=Callback::new(move |_| toggle_payload_selection(uid_for_on_select.clone()))
                                                />
                                            }.into_any()
                                        },
                                        DumpEntry::Group(dumps) => view! {
                                            <DumpGroup
                                                dumps=dumps
                                                on_delete=Callback::new(delete_payload)
                                                long_file_path=Signal::from(long_file_path)
                                                auto_group=Signal::from(auto_group)
                                                auto_expand=Signal::from(auto_expand)
                                                truncate_large_var_types=Signal::from(truncate_large_var_types)
                                                expand_all_command=expand_all_command
                                                collapse_all_command=collapse_all_command
                                                is_selectable=is_diff_mode
                                                selected_uids=selected_payload_uids
                                                on_select=Callback::new(toggle_payload_selection)
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

        <Show when=move || show_diff_modal.get()>
            <div class="fixed inset-0 bg-slate-950/80 backdrop-blur-sm z-[100] flex items-center justify-center p-4">
                <div class="bg-slate-900 border border-slate-800 rounded-xl shadow-2xl w-full max-w-4xl max-h-[80vh] flex flex-col">
                    <div class="p-4 border-b border-slate-800 flex items-center justify-between">
                        <h3 class="text-lg font-bold text-slate-200">"Payload Diff"</h3>
                        <button
                            class="text-slate-500 hover:text-slate-300"
                            on:click=move |_| set_show_diff_modal.set(false)
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                    <div class="p-4 overflow-auto font-mono text-sm whitespace-pre-wrap">
                        {move || {
                            let diff = diff_result.get().unwrap_or_default();
                            let lines: Vec<String> = diff.lines().map(|s| s.to_string()).collect();
                            lines.into_iter().map(|line| {
                                let color = if line.starts_with('+') {
                                    "text-green-400 bg-green-400/10"
                                } else if line.starts_with('-') {
                                    "text-red-400 bg-red-400/10"
                                } else {
                                    "text-slate-400"
                                };
                                view! {
                                    <div class=format!("px-2 py-0.5 rounded {}", color)>{line}</div>
                                }
                            }).collect_view()
                        }}
                    </div>
                    <div class="p-4 border-t border-slate-800 flex justify-end">
                        <button
                            class="bg-slate-800 hover:bg-slate-700 text-slate-200 px-4 py-2 rounded font-bold transition-colors"
                            on:click=move |_| set_show_diff_modal.set(false)
                        >
                            "Close"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
