use crate::components::LanguageIcon;
use chrono::prelude::*;
use chrono::Locale;
use leptos::prelude::*;
use leptos::{html};
use quo_common::payloads::{IncomingQuoPayload};
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
pub fn DumpItem(dump: IncomingQuoPayload) -> impl IntoView {
    let code_ref = NodeRef::<html::Code>::new();

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

    Effect::new(move |_| match code_ref.get() {
        Some(el) => highlightElement(&el),
        None => {}
    });

    view! {
        <div class="bg-slate-900 text-white my-4 rounded pb-2">
            <div class="bg-slate-950 w-full flex flex-row justify-between py-2 px-4 rounded-t">
                <h2 class="text-slate-500 font-normal w-full flex flex-row justify-between items-center">
                    <span
                        title="Filter dumps on this origin"
                        class="bg-slate-900 hover:bg-slate-950 rounded px-2 flex flex-row items-center justify-center gap-x-2 cursor-pointer"
                    >
                        <LanguageIcon lang=dump.language.clone() class="mt-[2px]".to_string() />
                        <p>{format!(" {}", dump.meta.origin)}</p>
                    </span>
                    <span>{format!(" {}", datetime_format(dump.meta.time_epoch_ms))}</span>
                    {format!("{}", dump.meta.sender_origin.replace("\\", "/"))}
                </h2>
            </div>
            <pre class="font-mono text-wrap">
                <code
                    node_ref=code_ref
                    class="language-rust rounded select-text"
                    style="background: transparent !important;"
                >
                    {format!(
                        "{} {}: {} = {}",
                        if dump.meta.variable.is_constant { "const" } else { "let" },
                        dump.meta.variable.name,
                        dump.meta.variable.var_type,
                        dump.meta.variable.value,
                    )}
                </code>
            </pre>
        </div>
    }
}
