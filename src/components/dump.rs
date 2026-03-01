use crate::components::LanguageIcon;
use crate::utils::formatter::format_by_language;
use chrono::prelude::*;
use chrono::Locale;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{html, serde_json};
use leptos_use::on_click_outside;
use quo_common::payloads::IncomingQuoPayload;
use std::string::ToString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightElement(el: &leptos::web_sys::Element);

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// @TODO configurable formats
const DATETIME_FORMAT: &str = "%_d %b %H:%M:%S.%3f";
const TIME_FORMAT: &str = "%H:%M:%S.%3f";

// @TODO configurable locale
const DEFAULT_LOCALE: Locale = Locale::nl_NL;

#[component]
pub fn DumpItem(dump: IncomingQuoPayload, on_delete: Callback<String>) -> impl IntoView {
    let code_ref = NodeRef::<html::Code>::new();
    let dropdown_ref = NodeRef::<html::Div>::new();

    let (show_dropdown, set_show_dropdown) = signal(false);
    let (available_editors, set_available_editors) = signal::<Vec<serde_json::Value>>(vec![]);
    let sender_origin = StoredValue::new(dump.meta.sender_origin.clone());

    let delete_uid = dump.meta.uid.clone();
    let delete_self = move || {
        on_delete.run(delete_uid.clone());
    };

    let open_default = StoredValue::new(move || {
        let path = sender_origin.get_value();
        spawn_local(async move {
            invoke(
                "open_file",
                serde_wasm_bindgen::to_value(&serde_json::json!({ "path": path })).unwrap(),
            )
            .await;
        });
        set_show_dropdown.set(false);
    });

    let open_in_editor = StoredValue::new(move |cmd: String| {
        let path = sender_origin.get_value();
        spawn_local(async move {
            invoke(
                "open_in_editor",
                serde_wasm_bindgen::to_value(&serde_json::json!({ "cmd": cmd, "path": path }))
                    .unwrap(),
            )
            .await;
        });
        set_show_dropdown.set(false);
    });

    let show_in_explorer = StoredValue::new(move || {
        let path = sender_origin.get_value();
        spawn_local(async move {
            invoke(
                "show_in_explorer",
                serde_wasm_bindgen::to_value(&serde_json::json!({ "path": path })).unwrap(),
            )
            .await;
        });
        set_show_dropdown.set(false);
    });

    // Close dropdown when clicking outside
    let _ = on_click_outside(dropdown_ref, move |_| set_show_dropdown.set(false));

    //
    // Functions
    //

    /// POC code formatting for larger objects
    fn format_code(dump: &IncomingQuoPayload) -> String {
        format_by_language(&dump)
    }

    /// Pretty datetime formatting
    fn datetime_format(epoch: i64) -> String {
        let now: DateTime<Local> = Local::now();

        // Include date if not today
        if let Some(chrono_dt) = DateTime::from_timestamp_millis(epoch) {
            if chrono_dt.date_naive() == now.date_naive() {
                return chrono_dt
                    .format_localized(TIME_FORMAT, DEFAULT_LOCALE)
                    .to_string();
            }

            return chrono_dt
                .format_localized(DATETIME_FORMAT, DEFAULT_LOCALE)
                .to_string();
        }

        "".to_string()
    }

    //
    // Effects
    //

    Effect::new(move |_| {
        spawn_local(async move {
            let editors = invoke("get_available_editors", JsValue::NULL).await;
            if let Ok(editors_vec) =
                serde_wasm_bindgen::from_value::<Vec<serde_json::Value>>(editors)
            {
                set_available_editors.set(editors_vec);
            }
        });
    });

    Effect::new(move |_| match code_ref.get() {
        Some(el) => highlightElement(&el),
        None => {}
    });

    view! {
        <div class="bg-slate-900 text-white my-4 rounded pb-2">
            <div class="bg-slate-950 flex flex-row justify-between py-2 pl-4 pr-2 rounded-t">
                <div
                    data-identifier="dump_header"
                    class="text-slate-500 font-normal w-full flex flex-row justify-between items-center"
                >
                    <div data-identifier="dump_project" class="flex-1 shrink-0">
                        <span
                            title="Filter dumps on this origin"
                            class="bg-slate-900 hover:bg-slate-950 rounded px-2 flex flex-row items-center justify-center gap-x-2 cursor-pointer w-48"
                        >
                            <LanguageIcon lang=dump.language.clone() class="mt-[2px]".to_string() />
                            <p>{format!(" {}", &dump.meta.origin)}</p>
                        </span>
                    </div>
                    <div data-identifier="dump_time" class="w-48 shrink-0 text-center">
                        <span>{format!(" {}", datetime_format(dump.meta.time_epoch_ms))}</span>
                    </div>
                    <div
                        data-identifier="dump_location"
                        class="flex-1 min-w-0 overflow-visible relative"
                    >
                        <div class="flex flex-row justify-end items-center gap-x-4">
                            <span
                                class="text-nowrap truncate [direction:rtl] text-left cursor-pointer hover:text-slate-300"
                                title=format!("{}", &dump.meta.sender_origin.replace("\\", "/"))
                                on:click=move |_| set_show_dropdown.update(|v| *v = !*v)
                            >
                                {format!("{}", &dump.meta.sender_origin.replace("\\", "/"))}
                            </span>
                            <Show when=move || show_dropdown.get()>
                                <div
                                    node_ref=dropdown_ref
                                    class="absolute top-8 right-10 bg-slate-800 border border-slate-700 rounded shadow-lg z-50 py-1 w-64 text-sm"
                                >
                                    <div
                                        class="flex flex-row items-center gap-x-1 px-4 py-2 hover:bg-slate-700 cursor-pointer text-slate-200"
                                        on:click=move |_| show_in_explorer.get_value()()
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            class="w-4 h-4"
                                            viewBox="0 0 24 24"
                                            fill="currentColor"
                                        >
                                            <path d="M2 4C2 3.44772 2.44772 3 3 3H10.4142L12.4142 5H21C21.5523 5 22 5.44772 22 6V20C22 20.5523 21.5523 21 21 21L3 21C2.45 21 2 20.55 2 20V4ZM10.5858 6L9.58579 5H4V7H9.58579L10.5858 6ZM4 9V19L20 19V7H12.4142L10.4142 9H4Z"></path>
                                        </svg>
                                        "Show in explorer"
                                    </div>
                                    <div
                                        class="flex flex-row items-center gap-x-1 px-4 py-2 hover:bg-slate-700 cursor-pointer text-slate-200"
                                        on:click=move |_| open_default.get_value()()
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            class="w-4 h-4"
                                            viewBox="0 0 24 24"
                                            fill="currentColor"
                                        >
                                            <path d="M3 3H21C21.5523 3 22 3.44772 22 4V20C22 20.5523 21.5523 21 21 21H3C2.44772 21 2 20.5523 2 20V4C2 3.44772 2.44772 3 3 3ZM4 5V19H20V5H4ZM20 12L16.4645 15.5355L15.0503 14.1213L17.1716 12L15.0503 9.87868L16.4645 8.46447L20 12ZM6.82843 12L8.94975 14.1213L7.53553 15.5355L4 12L7.53553 8.46447L8.94975 9.87868L6.82843 12ZM11.2443 17H9.11597L12.7557 7H14.884L11.2443 17Z"></path>
                                        </svg>
                                        "Open in default editor"
                                    </div>
                                    <For
                                        each=move || available_editors.get()
                                        key=|editor| {
                                            editor
                                                .get("id")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or_default()
                                                .to_string()
                                        }
                                        children=move |editor| {
                                            let id = editor
                                                .get("id")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or_default()
                                                .to_string();
                                            let name = editor
                                                .get("name")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or_default()
                                                .to_string();
                                            let cmd = editor
                                                .get("cmd")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or_default()
                                                .to_string();

                                            view! {
                                                <div
                                                    class="flex flex-row items-center gap-x-1 px-4 py-2 hover:bg-slate-700 cursor-pointer text-slate-200"
                                                    on:click=move |_| open_in_editor.get_value()(cmd.clone())
                                                >
                                                    <img
                                                        class="w-4 h-4"
                                                        src=format!("/public/assets/editor_icons/{}.svg", id)
                                                    />
                                                    {format!("Open in {}", name)}
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            </Show>
                            <span
                                title="Delete this dump"
                                class="opacity-50 hover:opacity-100 bg-red-800 text-slate-200 hover:bg-red-600 hover:text-white p-1 rounded transition-all cursor-pointer"
                                on:click=move |_| delete_self()
                            >

                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 24 24"
                                    fill="currentColor"
                                    class="w-4 h-4"
                                >
                                    <path d="M7 6V3C7 2.44772 7.44772 2 8 2H16C16.5523 2 17 2.44772 17 3V6H22V8H20V21C20 21.5523 19.5523 22 19 22H5C4.44772 22 4 21.5523 4 21V8H2V6H7ZM13.4142 13.9997L15.182 12.232L13.7678 10.8178L12 12.5855L10.2322 10.8178L8.81802 12.232L10.5858 13.9997L8.81802 15.7675L10.2322 17.1817L12 15.4139L13.7678 17.1817L15.182 15.7675L13.4142 13.9997ZM9 4V6H15V4H9Z"></path>
                                </svg>
                            </span>
                        </div>
                    </div>
                </div>
            </div>
            <pre class="font-mono text-wrap">
                <code
                    node_ref=code_ref
                    class=format!(
                        "language-{} language-rust rounded select-text",
                        serde_json::to_string(&dump.language).unwrap().replace("\"", ""),
                    )
                    style="background: transparent !important;"
                >
                    {format_code(&dump)}
                </code>
            </pre>
        </div>
    }
}
