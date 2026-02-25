use codee::string::JsonSerdeCodec;
use gloo_timers::callback::Timeout;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use quo_common::payloads::IncomingQuoPayload;

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

    view! {
        <div class="quo-sidebar">
            <div class="quo-sidebar-header">
                <img src="/public/animated_icon.apng" class="quo-logo w-8" />
                <span class="quo-logo-text">QUO</span>
            </div>
            <nav class="quo-nav">
                <div id="quo-tabs-container" class="quo-origin-tabs"></div>
            </nav>
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
