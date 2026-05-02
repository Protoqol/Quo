use crate::components::LanguageIcon;
use crate::utils::formatter::format_by_language;
use chrono::prelude::*;
use chrono::{Duration, Locale};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{html, serde_json};
use leptos_use::on_click_outside;
use quo_common::payloads::{IncomingQuoPayload, QuoPayloadLanguage};
use std::string::ToString;
use std::sync::OnceLock;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use wasm_bindgen::prelude::*;

/// A group of payloads that originated from the same `quo(...)` call,
/// identified by a shared `grouping_hash`.
#[component]
pub fn DumpGroup(
    dumps: Vec<IncomingQuoPayload>,
    on_delete: Callback<String>,
    long_file_path: Signal<bool>,
    auto_group: Signal<bool>,
    auto_expand: Signal<bool>,
    truncate_large_var_types: Signal<bool>,
    #[prop(into)] expand_all_command: Signal<usize>,
    #[prop(into)] collapse_all_command: Signal<usize>,
    #[prop(optional, into)] is_selectable: Signal<bool>,
    #[prop(optional, into)] selected_uids: Signal<Vec<String>>,
    #[prop(optional)] on_select: Option<Callback<String>>,
) -> impl IntoView {
    let count = dumps.len();

    // Collect variable names for the header summary, e.g. "foo, bar, baz"
    let var_names = StoredValue::new(
        dumps
            .iter()
            .map(|d| d.meta.variable.name.as_str())
            .collect::<Vec<_>>()
            .join(", "),
    );

    // Call-site file shown in the header (from the first dump)
    let call_site = StoredValue::new(
        dumps
            .first()
            .map(|f| {
                f.meta
                    .sender_origin
                    .split('/')
                    .next_back()
                    .unwrap_or("")
                    .to_string()
            })
            .unwrap_or_default(),
    );

    // Collapsed state — groups start expanded
    let (collapsed, set_collapsed) = signal(false);

    Effect::new(move |prev: Option<usize>| {
        let current = expand_all_command.get();
        if prev.is_some() && auto_group.get_untracked() {
            set_collapsed.set(false);
        }
        current
    });

    Effect::new(move |prev: Option<usize>| {
        let current = collapse_all_command.get();
        if prev.is_some() && auto_group.get_untracked() {
            set_collapsed.set(true);
        }
        current
    });

    let dumps_stored = StoredValue::new(dumps);

    view! {
        <div class=move || format!(
            "quo-dump-group relative animate-slide-in-top {}",
            if auto_group.get() { "rounded-lg border border-accent/20 bg-slate-950/40 overflow-hidden shadow-lg" } else { "" }
        )>
            // Group header bar — click anywhere to expand/collapse
            <Show when=move || auto_group.get()>
                <div
                    class="flex items-center gap-x-2 px-4 py-2 bg-slate-950 rounded-t-lg border-b border-accent/20 cursor-pointer select-none hover:bg-slate-900/80 transition-colors"
                    on:click=move |_| set_collapsed.update(|v| *v = !*v)
                    title="Click to expand / collapse group"
                >
                    // Collapse/expand chevron
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class=move || format!(
                            "w-3 h-3 shrink-0 text-accent/60 transition-transform duration-200 {}",
                            if collapsed.get() { "-rotate-90" } else { "rotate-0" }
                        )
                        viewBox="0 0 24 24"
                        fill="currentColor"
                    >
                        <path d="M11.9999 13.1714L16.9497 8.22168L18.3639 9.63589L11.9999 15.9999L5.63599 9.63589L7.0502 8.22168L11.9999 13.1714Z" />
                    </svg>

                    // Count badge
                    <span class="inline-flex items-center justify-center px-1.5 py-0.5 rounded bg-accent/20 text-accent text-[10px] font-bold shrink-0">
                        {count} " items"
                    </span>

                    // Variable names summary
                    <span class="text-xs text-slate-400 font-medium font-mono truncate max-w-[50%]" title=move || var_names.get_value()>
                        {move || var_names.get_value()}
                    </span>

                    // Call site (filename)
                    <span class="text-xs text-slate-500 ml-auto font-mono shrink-0">
                        {move || call_site.get_value()}
                    </span>
                </div>
            </Show>

            // Each dump inside the group, separated by a centered vertical line
            <Show when=move || !collapsed.get()>
                <div class="flex flex-col">
                    {move || {
                        let items = dumps_stored.get_value();
                        let last_idx = items.len().saturating_sub(1);
                        items
                            .into_iter()
                            .enumerate()
                            .map(|(i, dump)| {
                                let uid_for_select = dump.meta.uid.clone();
                                let uid_for_on_select = dump.meta.uid.clone();
                                view! {
                                    <DumpItem
                                        dump=dump
                                        on_delete=on_delete
                                        long_file_path=long_file_path
                                        is_grouped=auto_group.get()
                                        auto_expand=auto_expand
                                        truncate_large_var_types=truncate_large_var_types
                                        expand_all_command=expand_all_command
                                        collapse_all_command=collapse_all_command
                                        is_selectable=is_selectable
                                        is_selected=Signal::derive(move || {
                                            selected_uids.get().contains(&uid_for_select)
                                        })
                                        on_select=Callback::new(move |_| {
                                            if let Some(on_select) = on_select {
                                                on_select.run(uid_for_on_select.clone());
                                            }
                                        })
                                    />
                                    // Centered vertical line between consecutive items
                                    <Show when=move || i < last_idx && !auto_group.get()>
                                        <div class="flex justify-center -mt-4 h-4">
                                            <div class="w-[4px] bg-slate-600 h-full"></div>
                                        </div>
                                    </Show>
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </Show>
        </div>
    }
}

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME: OnceLock<Theme> = OnceLock::new();

// @TODO configurable
const CODE_THEME: &[u8] = include_bytes!("../syntec_themes/Vision_(colorblind).tmTheme");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// @TODO configurable formats
const DATETIME_FORMAT: &str = "%_d %b %H:%M:%S.%3f";
const TIME_FORMAT: &str = "%H:%M:%S.%3f";

// @TODO configurable locale
const DEFAULT_LOCALE: Locale = Locale::nl_NL;

#[component]
pub fn DumpItem(
    dump: IncomingQuoPayload,
    on_delete: Callback<String>,
    long_file_path: Signal<bool>,
    auto_expand: Signal<bool>,
    truncate_large_var_types: Signal<bool>,
    #[prop(optional)] is_grouped: bool,
    #[prop(into)] expand_all_command: Signal<usize>,
    #[prop(into)] collapse_all_command: Signal<usize>,
    #[prop(optional, into)] is_selectable: Signal<bool>,
    #[prop(optional, into)] is_selected: Signal<bool>,
    #[prop(optional)] on_select: Option<Callback<()>>,
) -> impl IntoView {
    let code_ref = NodeRef::<html::Code>::new();
    let dropdown_ref = NodeRef::<html::Div>::new();

    let (show_dropdown, set_show_dropdown) = signal(false);
    let (available_editors, set_available_editors) = signal::<Vec<serde_json::Value>>(vec![]);

    let dump_stored = StoredValue::new(dump);

    let (manual_state, set_manual_state) = signal::<Option<bool>>(None);

    let formatted_code = Memo::new(move |_| {
        format_by_language(&dump_stored.get_value(), truncate_large_var_types.get())
    });

    let content_lines = Memo::new(move |_| formatted_code.get().lines().count());

    Effect::new(move |prev: Option<usize>| {
        let current = expand_all_command.get();
        if prev.is_some() && content_lines.get_untracked() > 6 {
            set_manual_state.set(Some(true));
        }
        current
    });

    Effect::new(move |prev: Option<usize>| {
        let current = collapse_all_command.get();
        if prev.is_some() && content_lines.get_untracked() > 6 {
            set_manual_state.set(Some(false));
        }
        current
    });

    let is_collapsed = Memo::new(move |_| match manual_state.get() {
        Some(expanded) => !expanded,
        None => !auto_expand.get() && content_lines.get() > 6,
    });

    let is_fresh = {
        let dump_time = DateTime::from_timestamp_millis(dump_stored.get_value().meta.time_epoch_ms)
            .unwrap_or_else(|| Local::now().with_timezone(&Utc).into());
        let (fresh, set_fresh) =
            signal(Utc::now().signed_duration_since(dump_time) < Duration::seconds(15));

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
                            }) as Box<dyn FnMut()>)
                            .into_js_value()
                            .unchecked_into(),
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

    let sender_origin = StoredValue::new(dump_stored.get_value().meta.sender_origin.clone());

    // Reactive file path label — updates when the setting changes
    let sender_origin_raw = dump_stored.get_value().meta.sender_origin.clone();
    let file_path_label =
        Memo::new(move |_| file_path_format(&sender_origin_raw, long_file_path.get()));

    let delete_uid = dump_stored.get_value().meta.uid.clone();
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

    // let copy_to_clipboard = StoredValue::new(move |text: String| {
    //     let window = window();
    //     let navigator = window.navigator();
    //     let clipboard = navigator.clipboard();
    //     let _ = clipboard.write_text(&text);
    //     toast!("Copied to clipboard", ToastType::Success);
    // });

    // Close dropdown when clicking outside
    let _ = on_click_outside(dropdown_ref, move |_| set_show_dropdown.set(false));

    //
    // Functions
    //

    fn code_format(dump: &IncomingQuoPayload, formatted_code: &str) -> String {
        let ss = SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines);
        let theme = THEME.get_or_init(|| {
            let mut cursor = std::io::Cursor::new(CODE_THEME);
            ThemeSet::load_from_reader(&mut cursor).expect("Failed to load embedded theme")
        });

        let resolved_file_extension_for_syntax = match dump.language {
            QuoPayloadLanguage::Javascript => "js",
            // @TODO TypeScript syntax highlighting
            QuoPayloadLanguage::Typescript => "js",
            QuoPayloadLanguage::Rust => "rs",
            QuoPayloadLanguage::Php => "php",
            QuoPayloadLanguage::Python => "py",
            QuoPayloadLanguage::Ruby => "rb",
            QuoPayloadLanguage::Go => "go",
            QuoPayloadLanguage::Unknown => "txt",
        };

        let syntax = ss.find_syntax_by_extension(resolved_file_extension_for_syntax).unwrap();

        let to_highlight = if dump.language == QuoPayloadLanguage::Php {
            format!("<?php{}", formatted_code)
        } else {
            formatted_code.to_string()
        };

        let html = highlighted_html_for_string(&to_highlight, &ss, syntax, theme).unwrap();

        if dump.language == QuoPayloadLanguage::Php {
            if let Some(pos) = html.find("&lt;?php") {
                if let Some(start_span) = html[..pos].rfind("<span") {
                    if let Some(end_span) = html[pos..].find("</span>") {
                        let mut result = html.clone();
                        result.replace_range(start_span..pos + end_span + 7, "");
                        return result;
                    }
                }
            }

            return html.replace("&lt;?php\n", "").replace("<?php\n", "");
        }

        html
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

    /// Format file path — controlled by the `long-file-path` setting
    fn file_path_format(filepath: &str, show_full: bool) -> String {
        let normalized = filepath.replace("\\", "/");

        if show_full {
            normalized
        } else {
            normalized.split('/').next_back().unwrap_or("").to_string()
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

    view! {
        <div data-grouping-hash=move || format!("{}", dump_stored.get_value().meta.variable.grouping_hash.unwrap().to_string())
            class="quo-dump-container relative animate-slide-in-top group/item transition-all duration-300">
            <Show when=move || is_selectable.get()>
                <div
                    class=move || format!(
                        "absolute inset-0 z-[60] cursor-pointer transition-all duration-200 rounded {}",
                        if is_selected.get() { "bg-accent/10 border-2 border-accent" } else { "hover:bg-accent/5 border-2 border-transparent" }
                    )
                    on:click=move |_| {
                        if let Some(on_select) = on_select {
                            on_select.run(());
                        }
                    }
                >
                    <div class="absolute top-3 right-10">
                         <div class=move || format!(
                            "w-5 h-5 rounded border-2 flex items-center justify-center transition-colors {}",
                            if is_selected.get() { "bg-accent border-accent" } else { "border-slate-500 bg-transparent" }
                         )>
                            <Show when=move || is_selected.get()>
                                <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-slate-950" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="4" stroke-linecap="round" stroke-linejoin="round">
                                    <polyline points="20 6 9 17 4 12"></polyline>
                                </svg>
                            </Show>
                         </div>
                    </div>
                </div>
            </Show>
            <Show when=move || is_fresh.get()>
                <span class="absolute -top-1 -left-1 flex h-2.5 w-2.5 z-20">
                    <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-accent opacity-75"></span>
                    <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-accent"></span>
                </span>
            </Show>
            <div class=move || format!(
                "flex flex-row justify-between py-2 pl-4 pr-2 border-b border-slate-900/50 {}",
                if is_grouped { "bg-slate-900/60 rounded-none" } else { "bg-slate-950 rounded-t" }
            )>
                <div
                    data-identifier="dump_header"
                    class="text-slate-500 font-normal w-full flex flex-row justify-between items-center"
                >
                    <div data-identifier="dump_project" class="flex-none flex items-center gap-x-3">
                        <Show when=move || !is_grouped>
                            <span
                                title="Filter dumps on this origin"
                                class="bg-slate-900 hover:bg-slate-800 text-slate-400 hover:text-slate-300 rounded px-2 py-0.5 flex flex-row items-center justify-center gap-x-2 cursor-pointer w-fit text-xs font-medium transition-colors"
                            >
                                {dump_stored.get_value().meta.origin.clone()}
                            </span>
                        </Show>
                        <span class="text-xs font-mono text-accent/90 font-bold bg-accent/10 px-2 py-0.5 rounded">
                            {dump_stored.get_value().meta.variable.name.clone()}
                        </span>
                    </div>
                    <div
                        data-identifier="dump_location"
                        class="flex-1 min-w-0 overflow-visible relative ml-4"
                    >
                        <div class="flex flex-row justify-end items-center gap-x-2 relative">
                            <Show when=move || !is_grouped>
                                <span
                                    class="text-sm text-slate-500 text-nowrap truncate [direction:rtl] text-left cursor-pointer hover:text-slate-300 transition-colors"
                                    title=move || file_path_label.get()
                                    on:click=move |_| set_show_dropdown.update(|v| *v = !*v)
                                >
                                    {move || file_path_label.get()}
                                </span>
                            </Show>
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
                                    <path d="M12 3C10.8954 3 10 3.89543 10 5C10 6.10457 10.8954 7 12 7C13.1046 7 14 6.10457 14 5C14 3.89543 13.1046 3 12 3ZM12 10C10.8954 10 10 10.8954 10 12C10 13.1046 10.8954 14 12 14C13.1046 14 14 13.1046 14 12C14 10.8954 13.1046 10 12 10ZM12 17C10.8954 17 10 17.8954 10 19C10 20.1046 10.8954 21 12 21C13.1046 21 14 20.1046 14 19C14 17.8954 13.1046 17 12 17Z" />
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
                                            <path d="M2 4C2 3.44772 2.44772 3 3 3H10.4142L12.4142 5H21C21.5523 5 22 5.44772 22 6V20C22 20.5523 21.5523 21 21 21L3 21C2.45 21 2 20.55 2 20V4ZM10.5858 6L9.58579 5H4V7H9.58579L10.5858 6ZM4 9V19L20 19V7H12.4142L10.4142 9H4Z" />
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
                                            <path d="M3 3H21C21.5523 3 22 3.44772 22 4V20C22 20.5523 21.5523 21 21 21H3C2.44772 21 2 20.5523 2 20V4C2 3.44772 2.44772 3 3 3ZM4 5V19H20V5H4ZM20 12L16.4645 15.5355L15.0503 14.1213L17.1716 12L15.0503 9.87868L16.4645 8.46447L20 12ZM6.82843 12L8.94975 14.1213L7.53553 15.5355L4 12L7.53553 8.46447L8.94975 9.87868L6.82843 12ZM11.2443 17H9.11597L12.7557 7H14.884L11.2443 17Z" />
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
                                            <path d="M7 6V3C7 2.44772 7.44772 2 8 2H16C16.5523 2 17 2.44772 17 3V6H22V8H20V21C20 21.5523 19.5523 22 19 22H5C4.44772 22 4 21.5523 4 21V8H2V6H7ZM13.4142 13.9997L15.182 12.232L13.7678 10.8178L12 12.5855L10.2322 10.8178L8.81802 12.232L10.5858 13.9997L8.81802 15.7675L10.2322 17.1817L12 15.4139L13.7678 17.1817L15.182 15.7675L13.4142 13.9997ZM9 4V6H15V4H9Z" />
                                        </svg>
                                        "Delete dump"
                                    </div>
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </div>
            <div class="relative group bg-slate-900">
                <div class="absolute right-4 top-1 z-10 flex flex-row items-center gap-x-2">
                    <div class="flex flex-row items-center gap-x-1.5 backdrop-blur-sm px-2 py-1 text-[12px] text-slate-500 font-medium opacity-50 group-hover:opacity-100 transition-opacity">
                        {datetime_format(dump_stored.get_value().meta.time_epoch_ms).to_string()}
                    </div>
                </div>
                <div class=move || format!(
                    "font-mono relative bg-slate-900 {} {}",
                    if is_grouped { "rounded-none" } else { "rounded-b" },
                    if is_collapsed.get() { "max-h-[150px] overflow-hidden" } else { "overflow-x-auto" }
                )>
                    <div class="sticky left-0 top-0 h-0 z-10 pointer-events-none">
                        <div class="absolute left-4 top-2 opacity-50 group-hover:opacity-100 transition-opacity">
                            <LanguageIcon
                                lang=dump_stored.get_value().language.clone()
                                class="w-10 h-10 text-slate-500".to_string()
                            />
                        </div>
                    </div>
                    // <span
                    //     title="Copy code to clipboard"
                    //     class="absolute bottom-0 right-4 z-10 text-slate-300 hover:text-white p-1.5 rounded-lg shadow-sm border border-slate-700/50 cursor-pointer transition-all opacity-50 group-hover:opacity-100"
                    //     on:click={
                    //         let content = code_format(&dump);
                    //         move |_| copy_to_clipboard.get_value()(content.clone())
                    //     }
                    // >
                    //     <svg
                    //         xmlns="http://www.w3.org/2000/svg"
                    //         width="24"
                    //         height="24"
                    //         viewBox="0 0 24 24"
                    //         class="w-4 h-4 cursor-pointer "
                    //     >
                    //         <path
                    //             fill="currentColor"
                    //             d="M15.24 2h-3.894c-1.764 0-3.162 0-4.255.148c-1.126.152-2.037.472-2.755 1.193c-.719.721-1.038 1.636-1.189 2.766C3 7.205 3 8.608 3 10.379v5.838c0 1.508.92 2.8 2.227 3.342c-.067-.91-.067-2.185-.067-3.247v-5.01c0-1.281 0-2.386.118-3.27c.127-.948.413-1.856 1.147-2.593s1.639-1.024 2.583-1.152c.88-.118 1.98-.118 3.257-.118h3.07c1.276 0 2.374 0 3.255.118A3.6 3.6 0 0 0 15.24 2"
                    //         />
                    //         <path
                    //             fill="currentColor"
                    //             d="M6.6 11.397c0-2.726 0-4.089.844-4.936c.843-.847 2.2-.847 4.916-.847h2.88c2.715 0 4.073 0 4.917.847S21 8.671 21 11.397v4.82c0 2.726 0 4.089-.843 4.936c-.844.847-2.202.847-4.917.847h-2.88c-2.715 0-4.073 0-4.916-.847c-.844-.847-.844-2.21-.844-4.936z"
                    //         />
                    //     </svg>
                    // </span>
                    <code
                        node_ref=code_ref
                        class="code_dump select-text inline-block min-w-full pl-4 pr-12 pt-9 pb-4"
                        inner_html=move || code_format(&dump_stored.get_value(), &formatted_code.get())
                    >
                    </code>
                    <Show when=move || is_collapsed.get()>
                        <div class="absolute bottom-0 left-0 right-0 h-14 bg-gradient-to-t from-slate-900 via-slate-900/90 to-transparent flex items-end justify-center pb-3 z-10 pointer-events-none">
                            <button
                                class="bg-slate-800 hover:bg-slate-700 text-slate-300 text-[10px] font-bold uppercase tracking-wider px-3 py-1 rounded border border-slate-700 transition-colors shadow-xl pointer-events-auto"
                                on:click=move |_| set_manual_state.set(Some(true))
                            >
                                "Expand"
                            </button>
                        </div>
                    </Show>
                    <Show when=move || { !is_collapsed.get() && content_lines.get() > 6 }>
                        <div class="sticky left-0 w-full flex justify-center pb-2 z-10">
                            <button
                                class="bg-slate-800/50 hover:bg-slate-700 text-slate-500 hover:text-slate-300 text-[10px] font-bold uppercase tracking-wider px-3 py-1 rounded border border-slate-700/50 opacity-50 hover:opacity-100 transition-all"
                                on:click=move |_| set_manual_state.set(Some(false))
                            >
                                "Collapse"
                            </button>
                        </div>
                    </Show>
                </div>
            </div>
        </div>
    }
}
