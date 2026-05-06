use crate::atoms::{provide_toast_context, ToastType, Toaster};
use crate::layout::SideBar;
use crate::components::{DumpGroup, DumpItem};
use crate::modals::DiffModal;
use crate::toast;
use crate::utils::analytics::track_event;
use crate::utils::formatter::format_by_language;
use crate::utils::settings::{AppSettings, SettingDto};
use codee::string::JsonSerdeCodec;
use gloo_net::http::Request;
use leptos::ev;
use leptos::html;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::serde_json;
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

    track_event("app_started", Some(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION")
    })));

    let search_input_ref = NodeRef::<html::Input>::new();

    let (payloads, set_payloads, _) =
        use_local_storage::<Vec<IncomingQuoPayload>, JsonSerdeCodec>("payloads");

    let (search_query, set_search_query) = signal(String::new());
    let (selected_group, set_selected_group) = signal::<Option<String>>(None);

    let (expand_all_command, set_expand_all_command) = signal(0usize);
    let (collapse_all_command, set_collapse_all_command) = signal(0usize);

    let (server_host, set_server_host, _) =
        use_local_storage::<String, JsonSerdeCodec>("server_host");
    let (server_port, set_server_port, _) =
        use_local_storage::<String, JsonSerdeCodec>("server_port");

    let settings = use_context::<AppSettings>().expect("AppSettings context missing");
    let auto_group = settings.auto_group;
    let long_file_path = settings.long_file_path;
    let auto_expand = settings.auto_expand;
    let truncate_large_var_types = settings.truncate_large_var_types;
    let all_settings = settings.all_settings;
    let update_available = settings.update_available;
    let latest_version_signal = settings.latest_version;

    let (is_all_expanded, set_is_all_expanded) = signal(auto_expand.get_untracked());

    let (is_diff_mode, set_is_diff_mode) = signal(false);
    let (selected_payload_uids, set_selected_payload_uids) = signal::<Vec<String>>(vec![]);
    let (show_diff_modal, set_show_diff_modal) = signal(false);

    Effect::new(move |_| {
        spawn_local(async move {
            if let Ok(response) = Request::get("https://api.github.com/repos/Protoqol/Quo/releases/latest")
                .header("User-Agent", "Quo-Client")
                .send()
                .await {
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    if let Some(tag_name) = json["tag_name"].as_str() {
                        let latest_version = tag_name.replace("quo-v", "");
                        let current_version = env!("CARGO_PKG_VERSION");
                        
                        let is_update = |curr: &str, lat: &str| {
                            let c: Vec<u32> = curr.split('.').filter_map(|s| s.parse().ok()).collect();
                            let l: Vec<u32> = lat.split('.').filter_map(|s| s.parse().ok()).collect();
                            for i in 0..3 {
                                let cv = c.get(i).unwrap_or(&0);
                                let lv = l.get(i).unwrap_or(&0);
                                if lv > cv { return true; }
                                if lv < cv { return false; }
                            }
                            false
                        };

                        if is_update(current_version, &latest_version) {
                            latest_version_signal.set(Some(latest_version.clone()));
                            update_available.set(true);

                            track_event("update_detected", Some(serde_json::json!({
                                "current_version": current_version,
                                "latest_version": latest_version
                            })));
                        }
                    }
                }
            }
        });
    });

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
                    if let Some(v) = s.value.as_bool() {
                        auto_group.set(v);
                    }
                }
                if let Some(s) = all.iter().find(|s| s.id == "long-file-path") {
                    if let Some(v) = s.value.as_bool() {
                        long_file_path.set(v);
                    }
                }
                if let Some(s) = all.iter().find(|s| s.id == "auto-expand") {
                    if let Some(v) = s.value.as_bool() {
                        auto_expand.set(v);
                    }
                }
                if let Some(s) = all.iter().find(|s| s.id == "truncate-large-var-types") {
                    if let Some(v) = s.value.as_bool() {
                        truncate_large_var_types.set(v);
                    }
                }
                all_settings.set(all);
            }
        });
    });

    let filtered_payloads = move || {
        let query = search_query.get().to_lowercase();
        let group_filter = selected_group.get();
        let mut all = payloads.get().clone();

        if let Some(group) = group_filter {
            all.retain(|p| p.meta.origin == group);
        }

        if !query.is_empty() {
            all.retain(|p| {
                p.meta.variable.var_type.to_lowercase().contains(&query)
                    || p.meta.variable.name.to_lowercase().contains(&query)
                    || p.meta.variable.value.to_lowercase().contains(&query)
                    || p.meta.origin.to_lowercase().contains(&query)
                    || p.meta
                        .variable
                        .memory_address
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
            let hash = payload
                .meta
                .variable
                .grouping_hash
                .as_deref()
                .unwrap_or("")
                .to_string();
            let merged = if let Some(DumpEntry::Group(ref mut group)) = entries.last_mut() {
                let group_hash = group
                    .first()
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
            track_event("dump_deleted", None);
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
            DumpEntry::Single(payload) => format_by_language(payload, truncate).lines().count() > 6,
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
                    current.insert(0, event.payload.clone());
                    set_payloads.set(current);

                    track_event("payload_received", Some(serde_json::json!({
                        "language": event.payload.language,
                        "origin": event.payload.meta.origin
                    })));
                }
                Err(_e) => {
                    // @TODO error handle
                    eprintln!("Could not store incoming payload");
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

                    if success {
                        set_server_host.set(host.clone());
                        set_server_port.set(port.to_string());
                        console_log("Connection established");

                        track_event("connection_established", Some(serde_json::json!({
                            "host": host,
                            "port": port
                        })));
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

        let handle_app_exit = Closure::wrap(Box::new(move |_obj: JsValue| {
            //
        }) as Box<dyn FnMut(JsValue)>);

        let handle_clear_entries_event = Closure::wrap(Box::new(move |_obj: JsValue| {
            set_payloads.set(vec![]);
            set_selected_payload_uids.set(vec![]);
            set_is_diff_mode.set(false);
            set_selected_group.set(None);
            track_event("dumps_cleared_via_event", None);

            let _ = window().location().reload();
        }) as Box<dyn FnMut(JsValue)>);

        spawn_local(async move {
            listen("payload-received", &handle_payload_received_event).await;
            listen("connection-established", &handle_connection_established).await;
            listen("app-exit", &handle_app_exit).await;
            listen("clear-entries", &handle_clear_entries_event).await;

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

                    if success {
                        set_server_host.set(host);
                        set_server_port.set(port.to_string());
                        console_log("Initial connection info loaded")
                    }
                }
            }

            handle_payload_received_event.forget();
            handle_connection_established.forget();
            handle_app_exit.forget();
            handle_clear_entries_event.forget();
        });
    });

    view! {
        <div class="quo-layout">
            <Toaster />
            <SideBar
                server_host
                server_port
                selected_group
                set_selected_group
                on_clear=Callback::new(move |_| {
                    set_selected_payload_uids.set(vec![]);
                    set_is_diff_mode.set(false);
                    set_selected_group.set(None);
                })
            />
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
                                    let new_query = event_target_value(&ev);
                                    set_search_query.set(new_query.clone());

                                    if !new_query.is_empty() {
                                        track_event("search_performed", Some(serde_json::json!({
                                            "query_length": new_query.len()
                                        })));
                                    }
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
                   <Show when=move || is_diff_mode.get() && selected_payload_uids.get().len() == 2>
                       <div class="absolute bottom-8 right-4 z-[70]">
                           <button
                               class="bg-accent hover:bg-accent/75 text-slate-950 px-4 py-2 rounded-full font-bold shadow-xl flex items-center gap-2 border-0 border-slate-400 hover:border-1 transition-all"
                               on:click=move |_| {
                                   set_show_diff_modal.set(true);
                                   track_event("diff_modal_opened", None);
                               }
                           >
                               "Diff selected payloads"
                               <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                                   <path d="M16 3h5v5M4 20L21 3M21 16v5h-5M15 15l6 6M4 4l5 5"></path>
                               </svg>
                           </button>
                       </div>
                   </Show>
                    <div id="quo" class="relative">
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
                                                    track_event("collapse_all_clicked", None);
                                                } else {
                                                    set_expand_all_command.update(|v| *v += 1);
                                                    set_is_all_expanded.set(true);
                                                    track_event("expand_all_clicked", None);
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
                                            let new_state = !is_diff_mode.get_untracked();
                                            set_is_diff_mode.set(new_state);
                                            if !new_state {
                                                set_selected_payload_uids.set(vec![]);
                                            }

                                            track_event("diff_mode_toggled", Some(serde_json::json!({
                                                "enabled": new_state
                                            })));
                                        }
                                        title="Compare 2 payloads with eachother"
                                    >
                                        "Diff payloads"
                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 256 256">
                                            <path fill="currentColor" d="M112 154a6 6 0 0 0-6 6v33.52l-41.07-41.08a9.93 9.93 0 0 1-2.93-7.07v-52a30 30 0 1 0-12 0v52a21.88 21.88 0 0 0 6.44 15.56L97.52 202H64a6 6 0 0 0 0 12h48a6 6 0 0 0 6-6v-48a6 6 0 0 0-6-6M38 64a18 18 0 1 1 18 18a18 18 0 0 1-18-18m168 98.6v-52a21.88 21.88 0 0 0-6.44-15.56L158.48 54H192a6 6 0 0 0 0-12h-48a6 6 0 0 0-6 6v48a6 6 0 0 0 12 0V62.48l41.07 41.08a9.93 9.93 0 0 1 2.93 7.07v52a30 30 0 1 0 12 0Zm-6 47.4a18 18 0 1 1 18-18a18 18 0 0 1-18 18"/>
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
                                            <img draggable="false"
                                                oncontextmenu=move || false
                                                src="/public/assets/icons/boat-animation.apng"
                                                class="w-32 select-none"
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
                                                    selected_group=selected_group
                                                    set_selected_group=set_selected_group
                                                    expand_all_command=expand_all_command
                                                    collapse_all_command=collapse_all_command
                                                    is_selectable=is_diff_mode
                                                    selection_index=Signal::derive(move || selected_payload_uids.get().iter().position(|u| u == &uid_for_select))
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
                                                selected_group=selected_group
                                                set_selected_group=set_selected_group
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

        <DiffModal
            show=show_diff_modal
            selected_uids=selected_payload_uids
            payloads=payloads
            on_close=Callback::new(move |_| set_show_diff_modal.set(false))
        />
    }
}
