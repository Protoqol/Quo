use crate::components::LanguageIcon;
use codee::string::JsonSerdeCodec;
use gloo_timers::callback::Timeout;
use itertools::Itertools;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use quo_common::payloads::{IncomingQuoPayload, QuoPayloadLanguage};

#[derive(Clone, PartialEq)]
struct ToggleSetting {
    id: String,
    title: String,
    description: String,
    position: bool, // Toggle position, true = on, false = off.
}

#[component]
pub fn SideBar() -> impl IntoView {
    let (clear_button_txt, set_clear_button_txt) = signal("Clear entries".to_string());
    let (clear_button_disabled, set_clear_button_disabled) = signal(false);
    let (payloads, set_payloads, _) =
        use_local_storage::<Vec<IncomingQuoPayload>, JsonSerdeCodec>("payloads");

    let (server_host, _, _) = use_local_storage::<String, JsonSerdeCodec>("server_host");
    let (server_port, _, _) = use_local_storage::<String, JsonSerdeCodec>("server_port");

    // Delete all dumps from local storage.
    let clear_dump_entries = move |_ev: MouseEvent| {
        if !payloads.get().is_empty() {
            set_clear_button_disabled.set(true);
            set_clear_button_txt.set("Clearing...".to_string());
            set_payloads.set(vec![]);

            let timeout = Timeout::new(3_000, move || {
                set_clear_button_disabled.set(false);
                set_clear_button_txt.set("Clear entries".to_string());
            });

            timeout.forget();
        } else {
            set_clear_button_txt.set("Nothing to delete".to_string());

            let timeout = Timeout::new(3_000, move || {
                set_clear_button_txt.set("Clear entries".to_string());
            });

            timeout.forget();
        }
    };

    let toggle_settings: Vec<ToggleSetting> = vec![ToggleSetting {
        id: "auto-group-dumps".to_string(),
        title: "Auto group dumps".to_string(),
        description:
            "When dumping multiple variables at once Quo will automatically group those together."
                .to_string(),
        position: false,
    }];

    // @TODO optimise lists
    view! {
        <div class="quo-sidebar">
            <div class="quo-sidebar-header">
                <div class="flex flex-row">
                    <img src="/public/assets/icons/animated_icon.apng" class="quo-logo w-10" />
                    <span class="quo-logo-text text-white">QUO</span>
                </div>
                <a
                    title="Visit protoqol.nl"
                    href="https://protoqol.nl?referer=quo-app"
                    target="_blank"
                    class="text-accent font-semibold cursor-hover text-xs tracking-wider ml-6 -mt-2"
                >
                    Protoqol
                </a>
            </div>
            <nav class="quo-nav">
                <div id="quo-tabs-container" class="quo-origin-tabs">
                    <h2 class="text-md font-bold uppercase tracking-wider text-slate-500">
                        Groups
                        <small class="text-xs font-normal tracking-normal normal-case ml-2 text-slate-600">
                            Click to filter
                        </small>
                    </h2>
                    <hr class="mt-2 mb-4 border-slate-700" />
                    <For
                        each=move || {
                            let mut sorted_payloads = payloads.get().clone();
                            sorted_payloads.sort_by(|a, b| a.meta.origin.cmp(&b.meta.origin));
                            sorted_payloads
                                .into_iter()
                                .chunk_by(|a| a.meta.origin.clone())
                                .into_iter()
                                .map(|(key, group)| (key, group.collect::<Vec<_>>()))
                                .collect::<Vec<_>>()
                        }
                        key=|(group, _items)| group.clone()
                        children=|(group, items): (String, Vec<IncomingQuoPayload>)| {
                            let language: QuoPayloadLanguage = match items.first() {
                                Some(payload) => payload.language.clone(),
                                None => QuoPayloadLanguage::Unknown,
                            };

                            view! {
                                <div class="flex flex-row gap-x-2 border-[1px] border-transparent bg-slate-950 hover:border-slate-600 rounded px-2 py-2 cursor-pointer transition-all text-slate-500">
                                    <LanguageIcon lang=language class="mt-[4px]".to_string() />
                                    <p class="font-semibold">
                                        {format!("{} - {}", group, items.len())}
                                    </p>
                                </div>
                            }
                        }
                    />
                </div>
            </nav>
            <div title="Copy Quo address" class="flex flex-row justify-center items-center w-full">
                <div class="flex px-2 py-1 gap-x-2 flex-row justify-center items-center text-sm text-slate-600 mb-4 bg-slate-950 rounded hover:text-slate-500">
                    <pre class="select-text">
                        {format!("http://{}:{}", server_host.get(), server_port.get())}
                    </pre>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                        class="w-4 h-4"
                    >
                        <path d="M6.9998 6V3C6.9998 2.44772 7.44752 2 7.9998 2H19.9998C20.5521 2 20.9998 2.44772 20.9998 3V17C20.9998 17.5523 20.5521 18 19.9998 18H16.9998V20.9991C16.9998 21.5519 16.5499 22 15.993 22H4.00666C3.45059 22 3 21.5554 3 20.9991L3.0026 7.00087C3.0027 6.44811 3.45264 6 4.00942 6H6.9998ZM8.9998 6H16.9998V16H18.9998V4H8.9998V6Z"></path>
                    </svg>
                </div>
            </div>
            <div class="quo-sidebar-footer">
                <div class="settings-container mb-6 px-2">
                    <div class="flex items-center justify-between mb-3">
                        <span class="text-xs font-semibold uppercase tracking-wider text-slate-500">
                            Settings
                        </span>
                    </div>
                    <For
                        each=move || toggle_settings.clone()
                        key=|setting| setting.title.clone()
                        children=|setting: ToggleSetting| {
                            view! {
                                <label
                                    class="flex items-center justify-between cursor-pointer group"
                                    title=setting.description
                                >
                                    <span class="text-sm text-slate-400 group-hover:text-slate-200 transition-colors">
                                        {setting.title}
                                    </span>
                                    <div class="relative inline-flex items-center cursor-pointer">
                                        <input
                                            type="checkbox"
                                            id="autoGroupToggle"
                                            class="sr-only peer"
                                            checked=setting.position
                                        />
                                        <div class="w-9 h-5 bg-slate-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-slate-400 after:border-slate-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-indigo-600 peer-checked:after:bg-white"></div>
                                    </div>
                                </label>
                            }
                        }
                    />

                </div>
                <button
                    on:click=clear_dump_entries
                    type="button"
                    title="Clear all entries"
                    class="quo-btn-clear cursor-hover"
                    disabled=clear_button_disabled
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="16"
                        height="16"
                    >
                        <path fill="none" d="M0 0h24v24H0z" />
                        <path d="M17 6h5v2h-2v13a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V8H2V6h5V3a1 1 0 0 1 1-1h8a1 1 0 0 1 1 1v3zm1 2H6v12h12V8zm-4.586 6l1.768 1.768-1.414 1.414L12 15.414l-1.768 1.768-1.414-1.414L10.586 14l-1.768-1.768 1.414-1.414L12 12.586l1.768-1.768 1.414 1.414L13.414 14zM9 4v2h6V4H9z" />
                    </svg>
                    <span>{clear_button_txt}</span>
                </button>
            </div>
        </div>
    }
}
