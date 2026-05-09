use chrono::{DateTime, Local};
use crate::utils::formatter::format_by_language;
use crate::utils::analytics::track_event;
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
            <div class="modal-overlay">
                <div class="modal-container">
                    <div class="modal-header">
                        <h3>"Payload Diff"</h3>
                        <button
                            class="close-btn"
                            on:click=move |_| {
                                on_close.run(());
                                track_event("diff_modal_closed", None);
                            }
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                    <Show when=move || diff.get().is_some()>
                        <div class="diff-info-bar">
                            <div class="info-group">
                                <span class="badge">"Source"</span>
                                {move || p1_info.get().map(|p| view! {
                                    <div class="info-text">
                                        <span class="name">{p.meta.variable.name.clone()}</span>
                                        <span class="origin">{p.meta.origin.clone()} " - " {format_time(p.meta.time_epoch_ms)}</span>
                                    </div>
                                })}
                            </div>

                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="diff-arrow"><path d="M16.1716 10.9999L10.8076 5.63589L12.2218 4.22168L20 11.9999L12.2218 19.778L10.8076 18.3638L16.1716 12.9999H4V10.9999H16.1716Z"></path></svg>

                            <div class="info-group">
                                {move || p2_info.get().map(|p| view! {
                                    <div class="info-text">
                                        <span class="name">{p.meta.variable.name.clone()}</span>
                                        <span class="origin">{p.meta.origin.clone()} " - " {format_time(p.meta.time_epoch_ms)}</span>
                                    </div>
                                })}
                                <span class="badge">"Comparison"</span>
                            </div>
                        </div>
                    </Show>
                    <div class="modal-content">
                        {move || {
                            if is_loading.get() {
                                view! {
                                    <div class="empty-state">
                                        "Calculating diff..."
                                    </div>
                                }.into_any()
                            } else if let Some(diff_val) = diff.get() {
                                let lines: Vec<String> = diff_val.lines().map(|s| s.to_string()).collect();
                                let len = lines.len();

                                lines.into_iter().enumerate().map(|(i, mut line)| {
                                    let (label, type_class) = if line.starts_with('+') {
                                        ("Comparison", "added")
                                    } else if line.starts_with('-') {
                                        ("Source", "removed")
                                    } else {
                                        ("", "unchanged")
                                    };

                                    if i == 0 || i == len - 1 {
                                        line = line.trim_start().to_string();
                                    }

                                    view! {
                                        <div class=format!("diff-line {}", type_class)>
                                            <span>{line}</span>
                                            <Show when=move || !label.is_empty()>
                                                <span class="line-label">{label}</span>
                                            </Show>
                                        </div>
                                    }
                                }).collect_view().into_any()
                            } else {
                                view! {
                                    <div class="empty-state">
                                        "No diff available"
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                    <div class="modal-footer">
                        <button
                            class="footer-btn"
                            on:click=move |_| {
                                on_close.run(());
                                track_event("diff_modal_closed", None);
                            }
                        >
                            "Close"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
