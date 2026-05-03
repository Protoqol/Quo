use chrono::{DateTime, Local};
use crate::utils::formatter::format_by_language;
use leptos::prelude::*;
use leptos::serde_json;
use leptos::task::spawn_local;
use quo_common::payloads::IncomingQuoPayload;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn DiffModal(
    #[prop(into)] show: Signal<bool>,
    on_close: Callback<()>,
    #[prop(into)] selected_uids: Signal<Vec<String>>,
    #[prop(into)] payloads: Signal<Vec<IncomingQuoPayload>>,
) -> impl IntoView {
    let (diff, set_diff) = signal::<Option<String>>(None);
    let (is_loading, set_is_loading) = signal(false);
    let (p1_info, set_p1_info) = signal::<Option<IncomingQuoPayload>>(None);
    let (p2_info, set_p2_info) = signal::<Option<IncomingQuoPayload>>(None);

    let format_time = |epoch: i64| {
        DateTime::from_timestamp_millis(epoch)
            .map(|dt| dt.with_timezone(&Local).format("%H:%M:%S.%3f").to_string())
            .unwrap_or_default()
    };

    Effect::new(move |_| {
        if show.get() {
            let uids = selected_uids.get();
            
            if uids.len() == 2 {
                let all_payloads = payloads.get();
                let payload1 = all_payloads.iter().find(|p| p.meta.uid == uids[0]);
                let payload2 = all_payloads.iter().find(|p| p.meta.uid == uids[1]);

                if let (Some(p1), Some(p2)) = (payload1, payload2) {
                    set_p1_info.set(Some(p1.clone()));
                    set_p2_info.set(Some(p2.clone()));

                    let s1 = format_by_language(p1, false);
                    let s2 = format_by_language(p2, false);

                    set_is_loading.set(true);
                    set_diff.set(None);
                    spawn_local(async move {
                        let diff_result = invoke(
                            "get_diff_for_snippets",
                            serde_wasm_bindgen::to_value(&serde_json::json!({ "first": s1, "second": s2 }))
                                .unwrap(),
                        )
                        .await;
                        if let Ok(diff_str) = serde_wasm_bindgen::from_value::<String>(diff_result) {
                            set_diff.set(Some(diff_str));
                        }
                        set_is_loading.set(false);
                    });
                }
            }
        } else {
            set_diff.set(None);
            set_is_loading.set(false);
            set_p1_info.set(None);
            set_p2_info.set(None);
        }
    });

    view! {
        <Show when=move || show.get()>
            <div class="fixed inset-0 bg-slate-950/80 backdrop-blur-sm z-[100] flex items-center justify-center p-4">
                <div class="bg-slate-900 border border-slate-800 rounded-xl shadow-2xl w-full max-w-4xl max-h-[80vh] flex flex-col">
                    <div class="p-4 border-b border-slate-800 flex items-center justify-between">
                        <h3 class="text-lg font-bold text-slate-200">"Payload Diff"</h3>
                        <button
                            class="text-slate-500 hover:text-slate-300"
                            on:click=move |_| on_close.run(())
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                    <Show when=move || diff.get().is_some()>
                        <div class="px-6 py-3 border-b border-slate-800 bg-slate-900/50 flex flex-row justify-between gap-8 items-center shrink-0">
                            <div class="flex items-center gap-3 text-xs font-mono">
                                <span class="px-2 h-6 rounded bg-accent/10 border border-accent/20 flex items-center justify-center text-accent font-bold whitespace-nowrap">"Source"</span>
                                {move || p1_info.get().map(|p| view! {
                                    <div class="flex flex-col">
                                        <span class="text-slate-200 font-bold">{p.meta.variable.name.clone()}</span>
                                        <span class="text-slate-500">{p.meta.origin.clone()} " - " {format_time(p.meta.time_epoch_ms)}</span>
                                    </div>
                                })}
                            </div>

                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-8 h-8"><path d="M16.1716 10.9999L10.8076 5.63589L12.2218 4.22168L20 11.9999L12.2218 19.778L10.8076 18.3638L16.1716 12.9999H4V10.9999H16.1716Z"></path></svg>

                            <div class="flex items-center gap-3 text-xs font-mono">
                                {move || p2_info.get().map(|p| view! {
                                    <div class="flex flex-col">
                                        <span class="text-slate-200 font-bold">{p.meta.variable.name.clone()}</span>
                                        <span class="text-slate-500">{p.meta.origin.clone()} " - " {format_time(p.meta.time_epoch_ms)}</span>
                                    </div>
                                })}
                                <span class="px-2 h-6 rounded bg-accent/10 border border-accent/20 flex items-center justify-center text-accent font-bold whitespace-nowrap">"Comparison"</span>
                            </div>
                        </div>
                    </Show>
                    <div class="select-text p-4 overflow-auto font-mono text-sm whitespace-pre-wrap flex-grow">
                        {move || {
                            if is_loading.get() {
                                view! {
                                    <div class="flex items-center justify-center h-full text-slate-500 italic">
                                        "Calculating diff..."
                                    </div>
                                }.into_any()
                            } else if let Some(diff_val) = diff.get() {
                                let lines: Vec<String> = diff_val.lines().map(|s| s.to_string()).collect();
                                let len = lines.len();

                                lines.into_iter().enumerate().map(|(i, mut line)| {
                                    let (label, color) = if line.starts_with('+') {
                                        ("Comparison", "text-green-400 bg-green-400/10")
                                    } else if line.starts_with('-') {
                                        ("Source", "text-red-400 bg-red-400/10")
                                    } else {
                                        ("", "text-slate-400")
                                    };

                                    if i == 0 || i == len - 1 {
                                        line = line.trim_start().to_string();
                                    }

                                    view! {
                                        <div class=format!("px-2 py-0.5 rounded flex justify-between items-baseline gap-4 {}", color)>
                                            <span>{line}</span>
                                            <Show when=move || !label.is_empty()>
                                                <span class="opacity-50 font-bold text-[10px] uppercase shrink-0">{label}</span>
                                            </Show>
                                        </div>
                                    }
                                }).collect_view().into_any()
                            } else {
                                view! {
                                    <div class="flex items-center justify-center h-full text-slate-500 italic">
                                        "No diff available"
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                    <div class="p-4 border-t border-slate-800 flex justify-end">
                        <button
                            class="bg-slate-800 hover:bg-slate-700 text-slate-200 px-4 py-2 rounded font-bold transition-colors"
                            on:click=move |_| on_close.run(())
                        >
                            "Close"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
