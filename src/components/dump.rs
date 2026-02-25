use chrono::{DateTime, Locale};
use leptos::prelude::*;
use leptos::html;
use quo_common::payloads::IncomingQuoPayload;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightElement(el: &leptos::web_sys::Element);
}

#[component]
pub fn DumpItem(dump: IncomingQuoPayload) -> impl IntoView {
    let code_ref = NodeRef::<html::Code>::new();

    Effect::new(move |_| {
        match code_ref.get() {
            Some(el) => highlightElement(&el),
            None => {}
        }
    });

    // @TODO configurable locale

    view! {
        <div class="bg-slate-900 text-white my-4 rounded px-4 py-2">
            <div class="flex flex-row justify-between">
                <h2 class="text-slate-500 font-normal w-full flex flex-row justify-between items-center">
                    <span class="bg-pink-800 rounded text-white px-2 ">
                        {format!(" {}", dump.meta.origin)}
                    </span>
                    <span>
                        {format!(
                            " {}",
                            match DateTime::from_timestamp_millis(dump.meta.time_epoch_ms) {
                                Some(time) => {
                                    time.format_localized("%_d %b %H:%M:%S", Locale::nl_NL)
                                        .to_string()
                                }
                                None => "-".to_string(),
                            },
                        )}
                    </span>
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
