use crate::atoms::ToastType;
use crate::components::LanguageIcon;
use crate::toast;
use codee::string::JsonSerdeCodec;
use gloo_timers::callback::Timeout;
use itertools::Itertools;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use leptos_use::{use_clipboard, UseClipboardReturn};
use quo_common::payloads::{IncomingQuoPayload, QuoPayloadLanguage};

#[derive(Clone, PartialEq)]
struct ToggleSetting {
    id: String,
    title: String,
    description: String,
    position: bool, // Toggle position, true = on, false = off.
}

#[component]
pub fn SideBar(
    #[prop(into)] server_host: Signal<String>,
    #[prop(into)] server_port: Signal<String>,
) -> impl IntoView {
    let UseClipboardReturn {
        is_supported,
        copy: copy_fn,
        ..
    } = use_clipboard();

    let (clear_button_txt, set_clear_button_txt) = signal("Clear entries".to_string());
    let (clear_button_disabled, set_clear_button_disabled) = signal(false);
    let (payloads, set_payloads, _) =
        use_local_storage::<Vec<IncomingQuoPayload>, JsonSerdeCodec>("payloads");

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

    let toggle_settings: Vec<ToggleSetting> = vec![
        ToggleSetting {
            id: "auto-group-dumps".to_string(),
            title: "Auto group dumps".to_string(),
            description:
                "When dumping multiple variables at once Quo will automatically group those together."
                    .to_string(),
            position: false,
        },
        ToggleSetting {
            id: "auto-expand".to_string(),
            title: "Collapse data".to_string(),
            description:
                "Automatically expand larger data structures"
                    .to_string(),
            position: true,
        },
        ToggleSetting {
            id: "long-file-path".to_string(),
            title: "Show full file path".to_string(),
            description:
                "Show full file path instead of the truncated version"
                    .to_string(),
            position: false,
        }
    ];

    let copy_address = move |server_host: String, server_port: String, is_supported: bool| {
        if !is_supported {
            toast!("Clipboard is not available for writing", ToastType::Error);
            return;
        }

        copy_fn(format!("{}:{}", server_host, server_port).as_str());
        toast!("Copied to clipboard", ToastType::Success);
    };

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
                                <div class="flex flex-row justify-between items-center font-mono border-[1px] border-transparent bg-slate-950 hover:border-slate-700 rounded px-2 py-2 cursor-pointer transition-all text-slate-500 hover:text-slate-400">
                                    <span class="flex flex-row gap-x-2">
                                        <LanguageIcon lang=language class="mt-[4px]".to_string() />
                                        <p class="font-medium">{format!("{}", group)}</p>
                                    </span>
                                    <p class="text-sm align-middle">{format!("{}", items.len())}</p>
                                </div>
                            }
                        }
                    />
                </div>
            </nav>
            <div
                title="Copy Quo address"
                class="cursor-pointer flex flex-row justify-center items-center w-full"
                on:click=move |_| copy_address(
                    server_host.get(),
                    server_port.get(),
                    is_supported.get(),
                )
            >
                <div class="flex px-2 py-2 gap-x-2 flex-row justify-center items-center text-sm text-slate-600 mb-4 bg-slate-950 rounded hover:text-slate-500">
                    <pre class="cursor-pointer select-text">
                        {format!("http://{}:{}", server_host.get(), server_port.get())}
                    </pre>
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
                                    class="flex items-center justify-between cursor-pointer group mb-3"
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
                        class="w-4 h-4"
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
