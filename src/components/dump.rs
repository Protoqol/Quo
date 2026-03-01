use crate::atoms::ToastType;
use crate::components::LanguageIcon;
use crate::toast;
use crate::utils::formatter::format_by_language;
use chrono::prelude::*;
use chrono::{Duration, Locale};
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

    let is_fresh = {
        let dump_time = DateTime::from_timestamp_millis(dump.meta.time_epoch_ms).unwrap_or_else(|| Local::now().with_timezone(&Utc).into());
        let (fresh, set_fresh) = signal(Utc::now().signed_duration_since(dump_time) < Duration::seconds(15));

        if fresh.get_untracked() {
            Effect::new(move |_| {
                if fresh.get() {
                    let timeout_id = window()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            &Closure::wrap(Box::new(move || {
                                let current_age = Utc::now().signed_duration_since(dump_time);
                                
                                if current_age >= Duration::seconds(15) {
                                    set_fresh.set(false);
                                } else {
                                    set_fresh.set(true);
                                }
                            }) as Box<dyn FnMut()>).into_js_value().unchecked_into(),
                            1000,
                        )
                        .unwrap();
                    on_cleanup(move || {
                        window().clear_timeout_with_handle(timeout_id);
                    });
                }
            });
        }

        fresh
    };

    let sender_origin = StoredValue::new(dump.meta.sender_origin.clone());

    let delete_uid = dump.meta.uid.clone();
    let delete_self = StoredValue::new(move || {
        on_delete.run(delete_uid.clone());
    });

    let open_default = StoredValue::new(move || {
        // @TODO check if file exists use std::fs; assert!(!fs::exists("does_not_exist.txt").expect("Can't check existence of file does_not_exist.txt"));

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

    let copy_to_clipboard = StoredValue::new(move |text: String| {
        let window = window();
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();
        let _ = clipboard.write_text(&text);
        toast!("Copied to clipboard", ToastType::Success);
    });

    // Close dropdown when clicking outside
    let _ = on_click_outside(dropdown_ref, move |_| set_show_dropdown.set(false));

    //
    // Functions
    //

    /// POC code formatting for larger objects
    fn code_format(dump: &IncomingQuoPayload) -> String {
        format_by_language(&dump)
    }

    /// Pretty datetime formatting
    fn datetime_format(epoch: i64) -> String {
        let now: DateTime<Local> = Local::now();

        // Include date if not today
        if let Some(chrono_dt) = DateTime::from_timestamp_millis(epoch) {
            if chrono_dt.date_naive() == now.date_naive() {
                // @TODO use relative time ago instead of timestamp
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

    /// Format file path @TODO configurable full or truncated file path
    fn file_path_format(filepath: &String) -> String {
        let show_full = false;

        let normalized = filepath.replace("\\", "/");

        if show_full {
            normalized
        } else {
            normalized.split('/').last().unwrap_or("").to_string()
        }
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
        <div class="quo-dump-container relative animate-slide-in-top group/item transition-all duration-300">
            <Show when=move || is_fresh.get()>
                <span class="absolute -top-1 -left-1 flex h-2.5 w-2.5 z-20">
                    <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-accent opacity-75"></span>
                    <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-accent"></span>
                </span>
            </Show>
            <div class="bg-slate-950 flex flex-row justify-between py-2 pl-4 pr-2 rounded-t border-b border-slate-900">
                <div
                    data-identifier="dump_header"
                    class="text-slate-500 font-normal w-full flex flex-row justify-between items-center"
                >
                    <div data-identifier="dump_project" class="flex-none flex items-center gap-x-3">
                        <span
                            title="Filter dumps on this origin"
                            class="bg-slate-900 hover:bg-slate-800 text-slate-400 hover:text-slate-300 rounded px-2 py-0.5 flex flex-row items-center justify-center gap-x-2 cursor-pointer w-fit text-xs font-medium transition-colors"
                        >
                            {dump.meta.origin.clone()}
                        </span>
                    </div>
                    <div
                        data-identifier="dump_location"
                        class="flex-1 min-w-0 overflow-visible relative ml-4"
                    >
                        <div class="flex flex-row justify-end items-center gap-x-2 relative">
                            <span
                                class="text-sm text-slate-500 text-nowrap truncate [direction:rtl] text-left cursor-pointer hover:text-slate-300 transition-colors max-w-[300px]"
                                title=format!("{}", &dump.meta.sender_origin.replace("\\", "/"))
                                on:click=move |_| set_show_dropdown.update(|v| *v = !*v)
                            >
                                {file_path_format(&dump.meta.sender_origin)}
                            </span>
                            <div
                                class="p-1 rounded hover:bg-slate-800 cursor-pointer transition-colors"
                                on:click=move |_| set_show_dropdown.update(|v| *v = !*v)
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="w-4 h-4 text-slate-600 group-hover/item:text-slate-400 transition-colors"
                                    viewBox="0 0 24 24"
                                    fill="currentColor"
                                >
                                    <path d="M12 3C10.8954 3 10 3.89543 10 5C10 6.10457 10.8954 7 12 7C13.1046 7 14 6.10457 14 5C14 3.89543 13.1046 3 12 3ZM12 10C10.8954 10 10 10.8954 10 12C10 13.1046 10.8954 14 12 14C13.1046 14 14 13.1046 14 12C14 10.8954 13.1046 10 12 10ZM12 17C10.8954 17 10 17.8954 10 19C10 20.1046 10.8954 21 12 21C13.1046 21 14 20.1046 14 19C14 17.8954 13.1046 17 12 17Z"></path>
                                </svg>
                            </div>
                            <Show when=move || show_dropdown.get()>
                                <div
                                    node_ref=dropdown_ref
                                    class="absolute top-full right-0 mt-1 bg-slate-800 border border-slate-700 rounded shadow-lg z-[100] py-1 w-64 text-sm"
                                >
                                    <div
                                        class="flex flex-row items-center gap-x-2 px-4 py-2 hover:bg-slate-700 cursor-pointer text-slate-200"
                                        on:click=move |_| show_in_explorer.get_value()()
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            class="w-4 h-4 opacity-70"
                                            viewBox="0 0 24 24"
                                            fill="currentColor"
                                        >
                                            <path d="M2 4C2 3.44772 2.44772 3 3 3H10.4142L12.4142 5H21C21.5523 5 22 5.44772 22 6V20C22 20.5523 21.5523 21 21 21L3 21C2.45 21 2 20.55 2 20V4ZM10.5858 6L9.58579 5H4V7H9.58579L10.5858 6ZM4 9V19L20 19V7H12.4142L10.4142 9H4Z"></path>
                                        </svg>
                                        "Show in explorer"
                                    </div>
                                    <div
                                        class="flex flex-row items-center gap-x-2 px-4 py-2 hover:bg-slate-700 cursor-pointer text-slate-200"
                                        on:click=move |_| open_default.get_value()()
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            class="w-4 h-4 opacity-70"
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
                                                    class="flex flex-row items-center gap-x-2 px-4 py-2 hover:bg-slate-700 cursor-pointer text-slate-200"
                                                    on:click=move |_| open_in_editor.get_value()(cmd.clone())
                                                >
                                                    <img
                                                        class="w-4 h-4 opacity-70"
                                                        src=format!("/public/assets/editor_icons/{}.svg", id)
                                                    />
                                                    {format!("Open in {}", name)}
                                                </div>
                                            }
                                        }
                                    />
                                    <div class="border-t border-slate-700 my-1"></div>
                                    <div
                                        class="flex flex-row items-center gap-x-2 px-4 py-2 hover:bg-red-900/30 cursor-pointer text-red-400"
                                        on:click=move |_| {
                                            delete_self.get_value()();
                                            set_show_dropdown.set(false);
                                        }
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 24 24"
                                            fill="currentColor"
                                            class="w-4 h-4"
                                        >
                                            <path d="M7 6V3C7 2.44772 7.44772 2 8 2H16C16.5523 2 17 2.44772 17 3V6H22V8H20V21C20 21.5523 19.5523 22 19 22H5C4.44772 22 4 21.5523 4 21V8H2V6H7ZM13.4142 13.9997L15.182 12.232L13.7678 10.8178L12 12.5855L10.2322 10.8178L8.81802 12.232L10.5858 13.9997L8.81802 15.7675L10.2322 17.1817L12 15.4139L13.7678 17.1817L15.182 15.7675L13.4142 13.9997ZM9 4V6H15V4H9Z"></path>
                                        </svg>
                                        "Delete dump"
                                    </div>
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </div>
            <div class="relative group">
                <div class="absolute right-4 top-2 z-10 flex flex-row items-center gap-x-2">
                    <div class="flex flex-row items-center gap-x-1.5 bg-slate-950/80 backdrop-blur-sm border border-slate-800/50 px-2 py-1 rounded-lg text-[10px] text-slate-500 font-medium opacity-50 group-hover:opacity-100 transition-opacity">
                        <img
                            class="w-3 h-3 opacity-50"
                            src="/public/assets/icons/animated_clock.apng"
                        />
                        {format!("{}", datetime_format(dump.meta.time_epoch_ms))}
                    </div>
                </div>
                <pre class="font-mono text-wrap relative bg-slate-900 rounded-b">
                    <div class="absolute left-4 top-4 pointer-events-none">
                        <LanguageIcon
                            lang=dump.language.clone()
                            class="w-10 h-10 opacity-[0.03]".to_string()
                        />
                    </div>
                    <span
                        title="Copy code to clipboard"
                        class="absolute bottom-4 right-4 z-10 text-slate-300 hover:text-white p-1.5 rounded-lg shadow-sm border border-slate-700/50 cursor-pointer transition-all opacity-50 group-hover:opacity-100"
                        on:click={
                            let content = code_format(&dump);
                            move |_| copy_to_clipboard.get_value()(content.clone())
                        }
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            class="w-4 h-4 cursor-pointer "
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
                    </span>
                    <code
                        node_ref=code_ref
                        class=format!(
                            "language-{} rounded-b-2xl select-text block px-4 py-4",
                            serde_json::to_string(&dump.language)
                                .unwrap()
                                .replace("\"", "")
                                .to_lowercase(),
                        )
                        style="background: transparent !important;"
                    >
                        {code_format(&dump)}
                    </code>
                </pre>
            </div>
        </div>
    }
}
