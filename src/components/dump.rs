use crate::components::LanguageIcon;
use crate::utils::analytics::track_event;
use crate::utils::formatter::format_by_language;
use chrono::prelude::*;
use chrono::{Duration, Locale};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{html, serde_json};
use leptos_use::on_click_outside;
use quo_common::payloads::{IncomingQuoPayload, QuoPayloadLanguage, ERROR_IDENTIFIER_KEY};
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
    #[prop(into)] selected_group: Signal<Option<String>>,
    set_selected_group: WriteSignal<Option<String>>,
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
            "quo-dump-group {}",
            if auto_group.get() { "auto-grouped" } else { "" }
        )>
            <Show when=move || auto_group.get()>
                <div
                    class="quo-dump-group-header"
                    on:click=move |_| set_collapsed.update(|v| *v = !*v)
                    title="Click to expand / collapse group"
                >
                    // Collapse/expand chevron
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class=move || format!(
                            "chevron {}",
                            if collapsed.get() { "collapsed" } else { "expanded" }
                        )
                        viewBox="0 0 24 24"
                        fill="currentColor"
                    >
                        <path d="M11.9999 13.1714L16.9497 8.22168L18.3639 9.63589L11.9999 15.9999L5.63599 9.63589L7.0502 8.22168L11.9999 13.1714Z" />
                    </svg>

                    // Count badge
                    <span class="count-badge">
                        {count} " items"
                    </span>

                    // Variable names summary
                    <span class="var-names" title=move || var_names.get_value()>
                        {move || var_names.get_value()}
                    </span>

                    // Call site (filename)
                    <span class="call-site">
                        {move || call_site.get_value()}
                    </span>
                </div>
            </Show>

            // Each dump inside the group, separated by a centered vertical line
            <Show when=move || !collapsed.get()>
                <div class="items-container">
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
                                        selected_group=selected_group
                                        set_selected_group=set_selected_group
                                        is_selectable=is_selectable
                                        selection_index=Signal::derive(move || {
                                            selected_uids.get().iter().position(|u| u == &uid_for_select)
                                        })
                                        on_select=Callback::new(move |_| {
                                            if let Some(on_select) = on_select {
                                                on_select.run(uid_for_on_select.clone());
                                            }
                                        })
                                    />
                                    // Centered vertical line between consecutive items
                                    <Show when=move || i < last_idx && !auto_group.get()>
                                        <div class="item-separator">
                                            <div class="line"></div>
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
    #[prop(into)] selected_group: Signal<Option<String>>,
    set_selected_group: WriteSignal<Option<String>>,
    #[prop(optional)] is_grouped: bool,
    #[prop(into)] expand_all_command: Signal<usize>,
    #[prop(into)] collapse_all_command: Signal<usize>,
    #[prop(optional, into)] is_selectable: Signal<bool>,
    #[prop(optional, into)] selection_index: Signal<Option<usize>>,
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
        let cmd_for_invoke = cmd.clone();
        spawn_local(async move {
            invoke(
                "open_in_editor",
                serde_wasm_bindgen::to_value(
                    &serde_json::json!({ "cmd": cmd_for_invoke, "path": path }),
                )
                .unwrap(),
            )
            .await;
        });
        set_show_dropdown.set(false);

        track_event(
            "dump_open_in_editor",
            Some(serde_json::json!({
                "editor": cmd
            })),
        );
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

        track_event("dump_show_in_explorer", None);
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

    fn code_syntax_highlighter(payload: &IncomingQuoPayload, formatted_code: &str) -> String {
        if payload.meta.variable.var_type == ERROR_IDENTIFIER_KEY {
            return formatted_code.to_string();
        }

        let ss = SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines);
        let theme = THEME.get_or_init(|| {
            let mut cursor = std::io::Cursor::new(CODE_THEME);
            ThemeSet::load_from_reader(&mut cursor).expect("Failed to load embedded theme")
        });

        let resolved_file_extension_for_syntax = match payload.language {
            QuoPayloadLanguage::Javascript => "js",
            QuoPayloadLanguage::Typescript => "js", // @TODO TypeScript syntax highlighting
            QuoPayloadLanguage::Rust => "rs",
            QuoPayloadLanguage::Php => "php",
            QuoPayloadLanguage::Python => "py",
            QuoPayloadLanguage::Ruby => "rb",
            QuoPayloadLanguage::Go => "go",
            QuoPayloadLanguage::Unknown => "txt",
        };

        let syntax = ss
            .find_syntax_by_extension(resolved_file_extension_for_syntax)
            .unwrap();

        let to_highlight = if payload.language == QuoPayloadLanguage::Php {
            format!("<?php{}", formatted_code)
        } else {
            formatted_code.to_string()
        };

        let html = highlighted_html_for_string(&to_highlight, ss, syntax, theme).unwrap();

        if payload.language == QuoPayloadLanguage::Php {
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
            class="quo-dump-container">
            <Show when=move || is_selectable.get()>
                <div
                    class=move || format!(
                        "selection-overlay {}",
                        if selection_index.get().is_some() { "selected" } else { "unselected" }
                    )
                    on:click=move |_| {
                        if let Some(on_select) = on_select {
                            on_select.run(());
                        }
                    }
                >
                    <div class="selection-badge-container">
                         <div class=move || format!(
                            "selection-badge {}",
                            if selection_index.get().is_some() { "selected" } else { "unselected" }
                         )>
                            {move || selection_index.get().map(|i| if i == 0 { "Source" } else { "Comparison" }).unwrap_or_default()}
                         </div>
                    </div>
                </div>
            </Show>
            <Show when=move || is_fresh.get()>
                <span class="fresh-indicator">
                    <span class="ping"></span>
                    <span class="dot"></span>
                </span>
            </Show>
            <div class=move || format!(
                "dump-header {} {}",
                if is_grouped { "grouped" } else { "not-grouped" },
                if dump_stored.get_value().meta.variable.var_type == ERROR_IDENTIFIER_KEY { "error" } else { "" }
            )>
                <div
                    data-identifier="dump_header"
                    class="header-content"
                >
                    <div data-identifier="dump_project" class="project-info">
                        <Show when=move || !is_grouped>
                            <span
                                title="Filter dumps on this origin"
                                class="origin-badge"
                                on:click={
                                    let origin = dump_stored.get_value().meta.origin.clone();
                                    move |_| {
                                        if selected_group.get_untracked() == Some(origin.clone()) {
                                            set_selected_group.set(None);
                                        } else {
                                            set_selected_group.set(Some(origin.clone()));
                                            track_event("dump_filtered", Some(serde_json::json!({
                                                "origin": origin
                                            })));
                                        }
                                    }
                                }
                            >
                                {dump_stored.get_value().meta.origin.clone()}
                            </span>
                        </Show>
                        <span class="var-name">
                            {dump_stored.get_value().meta.variable.name.clone()}
                        </span>
                    </div>
                    <div
                        data-identifier="dump_location"
                        class="location-info"
                    >
                        <div class="location-container">
                            <Show when=move || !is_grouped>
                                <span
                                    class="file-path"
                                    title=move || file_path_label.get()
                                    on:click=move |_| set_show_dropdown.update(|v| *v = !*v)
                                >
                                    {move || file_path_label.get()}
                                </span>
                            </Show>
                            <div
                                class="menu-trigger"
                                on:click=move |_| set_show_dropdown.update(|v| *v = !*v)
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 24 24"
                                    fill="currentColor"
                                >
                                    <path d="M12 3C10.8954 3 10 3.89543 10 5C10 6.10457 10.8954 7 12 7C13.1046 7 14 6.10457 14 5C14 3.89543 13.1046 3 12 3ZM12 10C10.8954 10 10 10.8954 10 12C10 13.1046 10.8954 14 12 14C13.1046 14 14 13.1046 14 12C14 10.8954 13.1046 10 12 10ZM12 17C10.8954 17 10 17.8954 10 19C10 20.1046 10.8954 21 12 21C13.1046 21 14 20.1046 14 19C14 17.8954 13.1046 17 12 17Z" />
                                </svg>
                            </div>
                            <Show when=move || show_dropdown.get()>
                                <div
                                    node_ref=dropdown_ref
                                    class="dropdown-menu"
                                >
                                    <div
                                        class="menu-item"
                                        on:click=move |_| show_in_explorer.get_value()()
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 24 24"
                                            fill="currentColor"
                                        >
                                            <path d="M2 4C2 3.44772 2.44772 3 3 3H10.4142L12.4142 5H21C21.5523 5 22 5.44772 22 6V20C22 20.5523 21.5523 21 21 21L3 21C2.45 21 2 20.55 2 20V4ZM10.5858 6L9.58579 5H4V7H9.58579L10.5858 6ZM4 9V19L20 19V7H12.4142L10.4142 9H4Z" />
                                        </svg>
                                        "Show in explorer"
                                    </div>
                                    <div
                                        class="menu-item"
                                        on:click=move |_| open_default.get_value()()
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
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
                                                .clone()
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
                                                    class="menu-item"
                                                    on:click=move |_| open_in_editor.get_value()(cmd.clone())
                                                >
                                                    <img draggable="false"
                                                        oncontextmenu=move || false
                                                        src=format!("/public/assets/editor_icons/{}.svg", id)
                                                    />
                                                    {format!("Open in {}", name)}
                                                </div>
                                            }
                                        }
                                    />
                                    <div class="separator"></div>
                                    <div
                                        class="menu-item delete"
                                        on:click=move |_| {
                                            delete_self.get_value()();
                                            set_show_dropdown.set(false);
                                            track_event("dump_deleted", None);
                                        }
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 24 24"
                                            fill="currentColor"
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
            <div class="dump-body group">
                <div class="timestamp-container">
                    <div class="timestamp">
                        {datetime_format(dump_stored.get_value().meta.time_epoch_ms).to_string()}
                    </div>
                </div>
                <div class=move || format!(
                    "code-container {} {} {}",
                    if is_grouped { "grouped" } else { "not-grouped" },
                    if is_collapsed.get() { "collapsed" } else { "expanded" },
                    if dump_stored.get_value().meta.variable.var_type == ERROR_IDENTIFIER_KEY { "error" } else { "" }
                )>
                    <div class="language-overlay">
                        <div class="overlay-content">
                           <LanguageIcon
                               lang=dump_stored.get_value().language.clone()
                               class="w-10 h-10 text-slate-500".to_string()
                           />
                            <Show when=move || dump_stored.get_value().meta.variable.var_type == ERROR_IDENTIFIER_KEY>
                                <span class="error-badge">
                                    <p>error</p>
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M13 19.9C15.2822 19.4367 17 17.419 17 15V12C17 11.299 16.8564 10.6219 16.5846 10H7.41538C7.14358 10.6219 7 11.299 7 12V15C7 17.419 8.71776 19.4367 11 19.9V14H13V19.9ZM5.5358 17.6907C5.19061 16.8623 5 15.9534 5 15H2V13H5V12C5 11.3573 5.08661 10.7348 5.2488 10.1436L3.0359 8.86602L4.0359 7.13397L6.05636 8.30049C6.11995 8.19854 6.18609 8.09835 6.25469 8H17.7453C17.8139 8.09835 17.88 8.19854 17.9436 8.30049L19.9641 7.13397L20.9641 8.86602L18.7512 10.1436C18.9134 10.7348 19 11.3573 19 12V13H22V15H19C19 15.9534 18.8094 16.8623 18.4642 17.6907L20.9641 19.134L19.9641 20.866L17.4383 19.4077C16.1549 20.9893 14.1955 22 12 22C9.80453 22 7.84512 20.9893 6.56171 19.4077L4.0359 20.866L3.0359 19.134L5.5358 17.6907ZM8 6C8 3.79086 9.79086 2 12 2C14.2091 2 16 3.79086 16 6H8Z"></path></svg>
                                </span>
                            </Show>
                        </div>
                    </div>
                    <code
                        node_ref=code_ref
                        class=move || format!(
                            "code_dump {}",
                            if dump_stored.get_value().meta.variable.var_type == ERROR_IDENTIFIER_KEY { "error" } else { "" }
                        )
                        inner_html=move || code_syntax_highlighter(&dump_stored.get_value(), &formatted_code.get())
                    >
                    </code>
                    <Show when=move || is_collapsed.get()>
                        <div class="expand-overlay">
                            <button
                                on:click=move |_| {
                                    set_manual_state.set(Some(true));
                                    track_event("dump_expanded", None);
                                }
                            >
                                "Expand"
                            </button>
                        </div>
                    </Show>
                    <Show when=move || { !is_collapsed.get() && content_lines.get() > 6 }>
                        <div class="collapse-trigger">
                            <button
                                on:click=move |_| {
                                    set_manual_state.set(Some(false));
                                    track_event("dump_collapsed", None);
                                }
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
