use leptos::prelude::*;
use leptos::{component, html, view, IntoView};
use quo_common::payloads::IncomingQuoPayload;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightElement(el: &leptos::web_sys::Element);
}

#[component]
#[allow(dead_code)]
pub fn DumpItem(dump: IncomingQuoPayload) -> impl IntoView {
    let code_ref = NodeRef::<html::Code>::new();

    Effect::new(move |_| {
        if let Some(el) = code_ref.get() {
            highlightElement(&el);
        }
    });

    view! {
        <div class="bg-slate-900 text-white my-4 rounded p-4">
            <div class="flex flex-row justify-between">
                <h2 class="text-slate-600 font-normal">{format!("{}", dump.meta.origin)}</h2>
                <h2 class="text-white">
                    {format!("{}", dump.meta.sender_origin.replace("\\", "/"))}
                </h2>
            </div>
            <pre class="monospace text-wrap">
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
