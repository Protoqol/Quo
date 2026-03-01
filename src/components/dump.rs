use crate::components::LanguageIcon;
use crate::utils::formatter::format_by_language;
use chrono::prelude::*;
use chrono::Locale;
use leptos::prelude::*;
use leptos::{html, serde_json};
use quo_common::payloads::IncomingQuoPayload;
use std::string::ToString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightElement(el: &leptos::web_sys::Element);
}

// @TODO configurable formats
const DATETIME_FORMAT: &str = "%_d %b %H:%M:%S.%3f";
const TIME_FORMAT: &str = "%H:%M:%S.%3f";

// @TODO configurable locale
const DEFAULT_LOCALE: Locale = Locale::nl_NL;

#[component]
pub fn DumpItem(dump: IncomingQuoPayload, on_delete: Callback<String>) -> impl IntoView {
    let code_ref = NodeRef::<html::Code>::new();

    // POC code formatting for larger objects
    fn format_code(dump: &IncomingQuoPayload) -> String {
        format_by_language(&dump)
    }

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

    let delete_uid = dump.meta.uid.clone();
    let delete_self = move || {
        on_delete.run(delete_uid.clone());
    };

    Effect::new(move |_| match code_ref.get() {
        Some(el) => highlightElement(&el),
        None => {}
    });

    view! {
        <div class="bg-slate-900 text-white my-4 rounded pb-2">
            <div class="bg-slate-950 w-full flex flex-row justify-between py-2 pl-4 pr-2 rounded-t">
                <h2 class="text-slate-500 font-normal w-full flex flex-row justify-between items-center">
                    <span
                        title="Filter dumps on this origin"
                        class="bg-slate-900 hover:bg-slate-950 rounded px-2 flex flex-row items-center justify-center gap-x-2 cursor-pointer"
                    >
                        <LanguageIcon lang=dump.language.clone() class="mt-[2px]".to_string() />
                        <p>{format!(" {}", &dump.meta.origin)}</p>
                    </span>
                    <span>{format!(" {}", datetime_format(dump.meta.time_epoch_ms))}</span>
                    <div class="flex flex-row justify-center items-center gap-x-4">
                        <span>{format!("{}", &dump.meta.sender_origin.replace("\\", "/"))}</span>
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
                </h2>
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
