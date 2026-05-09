use crate::atoms::ToastType;
use crate::components::LanguageIcon;
use crate::toast;
use crate::utils::analytics::track_event;
use crate::utils::settings::{AppSettings, SettingDto};
use codee::string::JsonSerdeCodec;
use gloo_timers::callback::Timeout;
use itertools::Itertools;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::serde_json;
use leptos::task::spawn_local;
use leptos_use::storage::use_local_storage;
use leptos_use::{use_clipboard, UseClipboardReturn};
use quo_common::payloads::{IncomingQuoPayload, QuoPayloadLanguage};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn SideBar(
    #[prop(into)] server_host: Signal<String>,
    #[prop(into)] server_port: Signal<String>,
    #[prop(into)] selected_group: Signal<Option<String>>,
    set_selected_group: WriteSignal<Option<String>>,
    #[prop(optional)] on_clear: Option<Callback<()>>,
) -> impl IntoView {
    let UseClipboardReturn {
        is_supported,
        copy: copy_fn,
        ..
    } = use_clipboard();

    let (clear_button_txt, set_clear_button_txt) = signal("Clear entries".to_string());
    let (clear_button_disabled, set_clear_button_disabled) = signal(false);
    let (payloads, set_payloads, _) =
        use_local_storage::<Vec<IncomingQuoPayload>, JsonSerdeCodec>("payloads");

    // Delete all dumps from local storage.
    let clear_dump_entries = move |_ev: MouseEvent| {
        if !payloads.get().is_empty() {
            set_clear_button_disabled.set(true);
            set_clear_button_txt.set("Clearing...".to_string());

            if let Some(on_clear) = on_clear {
                on_clear.run(());
            }

            set_payloads.set(vec![]);
            track_event("dumps_cleared", None);

            let timeout = Timeout::new(3_000, move || {
                set_clear_button_disabled.set(false);
                set_clear_button_txt.set("Clear entries".to_string());
            });

            timeout.forget();
        } else {
            set_clear_button_txt.set("Nothing to delete".to_string());

            track_event("no_dumps_cleared", None);

            let timeout = Timeout::new(3_000, move || {
                set_clear_button_txt.set("Clear entries".to_string());
            });

            timeout.forget();
        }
    };

    let app_settings = use_context::<AppSettings>().expect("AppSettings context missing");
    let all_settings = app_settings.all_settings;

    // Derive the sidebar subset reactively from the shared signal (no separate fetch needed).
    let sidebar_settings = move || -> Vec<SettingDto> {
        all_settings
            .get()
            .into_iter()
            .filter(|s| s.show_in_sidebar)
            .collect()
    };

    let copy_address = move |server_host: String, server_port: String, is_supported: bool| {
        if !is_supported {
            toast!("Clipboard is not available for writing", ToastType::Error);
            return;
        }

        copy_fn(format!("{}:{}", server_host, server_port).as_str());
        toast!("Copied to clipboard", ToastType::Success);

        track_event("sidebar_address_copied", Some(serde_json::json!({
            "address": format!("{}:{}", server_host, server_port)
        })));
    };

    // @TODO optimise lists
    view! {
        <div class="quo-sidebar">
            <div class="quo-sidebar-header">
                <div class="quo-logo-container">
                    <div
                        oncontextmenu=move || false
                        class="quo-logo" />
                    <span class="quo-logo-text">"QUO"</span>
                </div>
                <a
                    title="Visit protoqol.nl"
                    href="https://protoqol.nl?referer=quo-app"
                    target="_blank"
                    class="quo-protoqol-link"
                >
                    Protoqol
                </a>
            </div>
            <nav class="quo-nav">
                <div id="quo-tabs-container" class="quo-origin-tabs">
                    <h2
                        class="quo-sidebar-groups-title"
                        on:click=move |_| set_selected_group.set(None)
                    >
                        Groups
                        <small class="quo-sidebar-groups-subtitle">
                            Click to filter
                        </small>
                    </h2>
                    <hr class="quo-sidebar-separator" />
                    <For
                        each=move || {
                            let mut sorted_payloads = payloads.get().clone();
                            sorted_payloads.sort_by(|a, b| a.meta.origin.cmp(&b.meta.origin));
                            sorted_payloads
                                .into_iter()
                                .chunk_by(|a| a.meta.origin.clone())
                                .into_iter()
                                .map(|(key, group)| (key, group.collect::<Vec<_>>()))
                                .collect::<Vec<_>>()
                        }
                        key=|(group, _items)| group.clone()
                        children=move |(group, items): (String, Vec<IncomingQuoPayload>)| {
                            let language: QuoPayloadLanguage = match items.first() {
                                Some(payload) => payload.language.clone(),
                                None => QuoPayloadLanguage::Unknown,
                            };
                            let group_for_click = group.clone();
                            let group_for_style = group.clone();

                            view! {
                                <div
                                    class=move || format!("quo-sidebar-group-item {}",
                                        if selected_group.get() == Some(group_for_style.clone()) { "active" } else { "inactive" }
                                    )
                                    on:click={
                                        let group = group_for_click.clone();
                                        move |_| {
                                            if selected_group.get_untracked() == Some(group.clone()) {
                                                set_selected_group.set(None);
                                            } else {
                                                set_selected_group.set(Some(group.clone()));
                                                track_event("sidebar_group_selected", Some(serde_json::json!({
                                                    "group": group
                                                })));
                                            }
                                        }
                                    }
                                >
                                    <span class="quo-sidebar-group-label">
                                        <LanguageIcon lang=language class="mt-[4px]".to_string() />
                                        <p>{format!("{}", group)}</p>
                                    </span>
                                    <p class="quo-sidebar-group-count">{format!("{}", items.len())}</p>
                                </div>
                            }
                        }
                    />
                </div>
            </nav>
            <div
                title="Copy Quo address"
                class="quo-sidebar-address-container"
                on:click=move |_| copy_address(
                    server_host.get(),
                    server_port.get(),
                    is_supported.get(),
                )
            >
                <div class="quo-sidebar-address">
                    <pre>
                        {move || {
                            let host = server_host.get();
                            let port = server_port.get();
                            if host.is_empty() || port.is_empty() || port == "0" {
                                "Waiting for Quo server...".to_string()
                            } else {
                                format!("http://{}:{}", host, port)
                            }
                        }}
                    </pre>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                    >
                        <path
                            fill="currentColor"
                            d="M15.24 2h-3.894c-1.764 0-3.162 0-4.255.148c-1.126.152-2.037.472-2.755 1.193c-.719.721-1.038 1.636-1.189 2.766C3 7.205 3 8.608 3 10.379v5.838c0 1.508.92 2.8 2.227 3.342c-.067-.91-.067-2.185-.067-3.247v-5.01c0-1.281 0-2.386.118-3.27c.127-.948.413-1.856 1.147-2.593s1.639-1.024 2.583-1.152c.88-.118 1.98-.118 3.257-.118h3.07c1.276 0 2.374 0 3.255.118A3.6 3.6 0 0 0 15.24 2"
                        />
                        <path
                            fill="currentColor"
                            d="M6.6 11.397c0-2.726 0-4.089.844-4.936c.843-.847 2.2-.847 4.916-.847h2.88c2.715 0 4.073 0 4.917.847S21 8.671 21 11.397v4.82c0 2.726 0 4.089-.843 4.936c-.844.847-2.202.847-4.917.847h-2.88c-2.715 0-4.073 0-4.916-.847c-.844-.847-.844-2.21-.844-4.936z"
                        />
                    </svg>
                </div>
            </div>
            <div class="quo-sidebar-footer">
                <div class="settings-container">
                    <div class="settings-header">
                        <span>
                            Settings
                        </span>
                    </div>
                    <For
                        each=sidebar_settings
                        key=|setting| setting.id.clone()
                        children=move |setting| {
                            let stored_id = StoredValue::new(setting.id.clone());
                            // Derive checked state from the shared all_settings signal.
                            let checked = Memo::new(move |_| {
                                all_settings
                                    .get()
                                    .into_iter()
                                    .find(|s| s.id == stored_id.get_value())
                                    .and_then(|s| s.value.as_bool())
                                    .unwrap_or(false)
                            });
                            view! {
                                <label
                                    class="setting-label group"
                                    title=setting.description.clone()
                                >
                                    <span>
                                        {setting.label.clone()}
                                    </span>
                                    <button
                                        class=move || format!(
                                            "setting-toggle {}",
                                            if checked.get() { "checked" } else { "unchecked" }
                                        )
                                        on:click=move |_| {
                                            let new_val = !checked.get_untracked();
                                            let id = stored_id.get_value();

                                            all_settings.update(|list| {
                                                if let Some(s) = list.iter_mut().find(|s| s.id == id) {
                                                    s.value = serde_json::json!(new_val);
                                                }
                                            });

                                            track_event("sidebar_setting_changed", Some(serde_json::json!({
                                                "id": id,
                                                "value": new_val
                                            })));

                                            match id.as_str() {
                                                "auto-group-dumps" => app_settings.auto_group.set(new_val),
                                                "long-file-path"   => app_settings.long_file_path.set(new_val),
                                                "auto-expand"      => app_settings.auto_expand.set(new_val),
                                                "truncate-large-var-types" => app_settings.truncate_large_var_types.set(new_val),
                                                _ => {}
                                            }
                                            // Persist to the Tauri store
                                            spawn_local(async move {
                                                let args = serde_wasm_bindgen::to_value(
                                                    &serde_json::json!({ "id": id, "value": new_val })
                                                ).unwrap();
                                                invoke("set_setting", args).await;
                                            });
                                        }
                                    >
                                        <span class=move || format!(
                                            "toggle-dot {}",
                                            if checked.get() { "translated" } else { "initial" }
                                        ) />
                                    </button>
                                </label>
                            }
                        }
                    />
                </div>
                <button
                    on:click=clear_dump_entries
                    type="button"
                    title="Clear all entries"
                    class="quo-btn-clear cursor-pointer"
                    disabled=clear_button_disabled
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="16"
                        height="16"
                        class="w-4 h-4"
                    >
                        <path fill="none" d="M0 0h24v24H0z" />
                        <path d="M17 6h5v2h-2v13a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V8H2V6h5V3a1 1 0 0 1 1-1h8a1 1 0 0 1 1 1v3zm1 2H6v12h12V8zm-4.586 6l1.768 1.768-1.414 1.414L12 15.414l-1.768 1.768-1.414-1.414L10.586 14l-1.768-1.768 1.414-1.414L12 12.586l1.768-1.768 1.414 1.414L13.414 14zM9 4v2h6V4H9z" />
                    </svg>
                    <span>{clear_button_txt}</span>
                </button>
            </div>
        </div>
    }
}
