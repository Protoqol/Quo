use crate::app::{AppSettings, SettingDto};
use leptos::prelude::*;
use leptos::serde_json;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    pub type TauriWindow;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"])]
    fn getCurrentWindow() -> TauriWindow;

    #[wasm_bindgen(method)]
    fn minimize(this: &TauriWindow);
    #[wasm_bindgen(method)]
    fn toggleMaximize(this: &TauriWindow);
    #[wasm_bindgen(method)]
    fn close(this: &TauriWindow);

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Taskbar() -> impl IntoView {
    let app_window = getCurrentWindow();
    let settings = use_context::<AppSettings>().expect("AppSettings context missing");
    let set_auto_group = settings.auto_group;
    let set_long_file_path = settings.long_file_path;
    let set_auto_expand = settings.auto_expand;
    let set_truncate_large_var_types = settings.truncate_large_var_types;
    let all_settings = settings.all_settings;

    let (show_menu, set_show_menu) = signal(false);
    let (show_settings, set_show_settings) = signal(false);
    let (active_category, set_active_category) = signal("UI".to_string());

    // Reload settings from the backend into the shared signal every time the modal opens.
    Effect::new(move |_| {
        if show_settings.get() {
            spawn_local(async move {
                let result = invoke("get_settings", JsValue::NULL).await;
                if let Ok(fresh) = serde_wasm_bindgen::from_value::<Vec<SettingDto>>(result) {
                    all_settings.set(fresh);
                }
            });
        }
    });

    // Derived: settings for the active category
    let active_settings = move || {
        let cat = active_category.get();
        all_settings.get().into_iter().filter(|s| s.category == cat).collect::<Vec<_>>()
    };

    // Derived: ordered unique categories from loaded settings (falls back to built-in order)
    let categories = move || {
        let mut seen = std::collections::HashSet::new();
        let mut cats: Vec<String> = all_settings
            .get()
            .into_iter()
            .filter_map(|s| if seen.insert(s.category.clone()) { Some(s.category) } else { None })
            .collect();
        // Always include "About" even though it has no dynamic settings
        if !seen.contains("About") {
            cats.push("About".to_string());
        }
        cats
    };

    let app_window = StoredValue::new(app_window);
    let on_minimize = move |_| app_window.get_value().minimize();
    let on_maximize = move |_| app_window.get_value().toggleMaximize();
    let on_close = move |_| app_window.get_value().close();
    let on_quit = move |_| app_window.get_value().close();

    view! {
        <>
        <div class="titlebar relative z-50">
            <div data-tauri-drag-region class="bg-slate-950">
                <div
                    data-tauri-drag-region
                    class="w-64 h-[30px] bg-slate-950 flex items-center justify-between px-2"
                >
                    <div class="relative flex items-center">
                        <svg
                            on:click=move |_| set_show_menu.update(|v| *v = !*v)
                            title="Open Quo settings"
                            class="w-5 h-5 fill-slate-500 hover:fill-slate-300 transition-colors cursor-pointer"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                        >
                            <path d="M3 4H21V6H3V4ZM3 11H21V13H3V11ZM3 18H21V20H3V18Z" />
                        </svg>

                        <Show when=move || show_menu.get()>
                            <div class="absolute top-[30px] left-2 w-56 bg-slate-900 border border-slate-800 rounded shadow-xl z-[100] overflow-hidden flex flex-col">
                                <button
                                    class="!w-full !flex !justify-start !items-center !gap-x-3 !px-4 !py-2.5 text-sm text-slate-300 hover:!bg-slate-800 hover:!text-white transition-colors"
                                    on:click=move |_| {
                                        set_show_settings.set(true);
                                        set_show_menu.set(false);
                                    }
                                >
                                    <svg class="w-4 h-4 shrink-0 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                                        <path d="M3.33946 17.0002C2.90721 16.2515 2.58277 15.4702 2.36133 14.6741C3.3338 14.1779 3.99972 13.1668 3.99972 12.0002C3.99972 10.8345 3.3348 9.824 2.36353 9.32741C2.81025 7.71651 3.65857 6.21627 4.86474 4.99001C5.7807 5.58416 6.98935 5.65534 7.99972 5.072C9.01009 4.48866 9.55277 3.40635 9.4962 2.31604C11.1613 1.8846 12.8847 1.90004 14.5031 2.31862C14.4475 3.40806 14.9901 4.48912 15.9997 5.072C17.0101 5.65532 18.2187 5.58416 19.1346 4.99007C19.7133 5.57986 20.2277 6.25151 20.66 7.00021C21.0922 7.7489 21.4167 8.53025 21.6381 9.32628C20.6656 9.82247 19.9997 10.8336 19.9997 12.0002C19.9997 13.166 20.6646 14.1764 21.6359 14.673C21.1892 16.2839 20.3409 17.7841 19.1347 19.0104C18.2187 18.4163 17.0101 18.3451 15.9997 18.9284C14.9893 19.5117 14.4467 20.5941 14.5032 21.6844C12.8382 22.1158 11.1148 22.1004 9.49633 21.6818C9.55191 20.5923 9.00929 19.5113 7.99972 18.9284C6.98938 18.3451 5.78079 18.4162 4.86484 19.0103C4.28617 18.4205 3.77172 17.7489 3.33946 17.0002ZM8.99972 17.1964C10.0911 17.8265 10.8749 18.8227 11.2503 19.9659C11.7486 20.0133 12.2502 20.014 12.7486 19.9675C13.1238 18.8237 13.9078 17.8268 14.9997 17.1964C16.0916 16.5659 17.347 16.3855 18.5252 16.6324C18.8146 16.224 19.0648 15.7892 19.2729 15.334C18.4706 14.4373 17.9997 13.2604 17.9997 12.0002C17.9997 10.74 18.4706 9.5632 19.2729 8.6665C19.1688 8.4405 19.0538 8.21822 18.9279 8.00021C18.802 7.78219 18.667 7.57148 18.5233 7.36842C17.3457 7.61476 16.0911 7.43414 14.9997 6.80405C13.9083 6.17395 13.1246 5.17768 12.7491 4.03455C12.2509 3.98714 11.7492 3.98646 11.2509 4.03292C10.8756 5.17671 10.0916 6.17364 8.99972 6.80405C7.9078 7.43447 6.65245 7.61494 5.47428 7.36803C5.18485 7.77641 4.93463 8.21117 4.72656 8.66637C5.52881 9.56311 5.99972 10.74 5.99972 12.0002C5.99972 13.2604 5.52883 14.4372 4.72656 15.3339C4.83067 15.5599 4.94564 15.7822 5.07152 16.0002C5.19739 16.2182 5.3324 16.4289 5.47612 16.632C6.65377 16.3857 7.90838 16.5663 8.99972 17.1964ZM11.9997 15.0002C10.3429 15.0002 8.99972 13.6571 8.99972 12.0002C8.99972 10.3434 10.3429 9.00021 11.9997 9.00021C13.6566 9.00021 14.9997 10.3434 14.9997 12.0002C14.9997 13.6571 13.6566 15.0002 11.9997 15.0002ZM11.9997 13.0002C12.552 13.0002 12.9997 12.5525 12.9997 12.0002C12.9997 11.4479 12.552 11.0002 11.9997 11.0002C11.4474 11.0002 10.9997 11.4479 10.9997 12.0002C10.9997 12.5525 11.4474 13.0002 11.9997 13.0002Z" />
                                    </svg>
                                    <span>"Settings"</span>
                                </button>
                                <a
                                    class="!w-full !flex !justify-start !items-center !gap-x-3 !px-4 !py-2.5 text-sm text-slate-300 hover:!bg-slate-800 hover:!text-white transition-colors cursor-pointer"
                                    href="https://github.com/Protoqol/Quo/issues/new"
                                    target="_blank"
                                    on:click=move |_| set_show_menu.set(false)
                                >
                                    <svg class="w-4 h-4 shrink-0 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                                        <path d="M13 19.9C15.2822 19.4367 17 17.419 17 15V12C17 11.299 16.8564 10.6219 16.5846 10H7.41538C7.14358 10.6219 7 11.299 7 12V15C7 17.419 8.71776 19.4367 11 19.9V14H13V19.9ZM5.5358 17.6907C5.19061 16.8623 5 15.9534 5 15H2V13H5V12C5 11.3573 5.08661 10.7348 5.2488 10.1436L3.0359 8.86602L4.0359 7.13397L6.05636 8.30049C6.11995 8.19854 6.18609 8.09835 6.25469 8H17.7453C17.8139 8.09835 17.88 8.19854 17.9436 8.30049L19.9641 7.13397L20.9641 8.86602L18.7512 10.1436C18.9134 10.7348 19 11.3573 19 12V13H22V15H19C19 15.9534 18.8094 16.8623 18.4642 17.6907L20.9641 19.134L19.9641 20.866L17.4383 19.4077C16.1549 20.9893 14.1955 22 12 22C9.80453 22 7.84512 20.9893 6.56171 19.4077L4.0359 20.866L3.0359 19.134L5.5358 17.6907ZM8 6C8 3.79086 9.79086 2 12 2C14.2091 2 16 3.79086 16 6H8Z" />
                                    </svg>
                                    <span>"Report a bug"</span>
                                </a>
                                <hr class="border-slate-700 mx-2" />
                                <button
                                    class="group !w-full !flex !justify-start !items-center !pl-[2.75rem] !pr-4 !py-2.5 text-sm text-slate-300 hover:!bg-slate-800 hover:!text-white transition-colors"
                                    on:click=on_quit
                                >
                                    <span class="!flex !gap-x-1">
                                        <span>"Quit"</span>
                                        <span class="text-slate-600 font-light group-hover:text-slate-500 transition-colors">"(pro)"</span>
                                        <span>"Quo"</span>
                                    </span>
                                </button>
                            </div>
                            <div
                                class="fixed inset-0 z-[90]"
                                on:click=move |_| set_show_menu.set(false)
                            />
                        </Show>
                    </div>
                </div>
            </div>
            <div class="controls bg-slate-950">
                <button title="Minimize" on:click=on_minimize>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="24"
                        height="24"
                        fill="currentColor"
                    >
                        <path d="M19 11H5V13H19V11Z" />
                    </svg>
                </button>
                <button title="Maximize" on:click=on_maximize>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="16"
                        height="16"
                        fill="currentColor"
                    >
                        <path d="M4 3H20C20.5523 3 21 3.44772 21 4V20C21 20.5523 20.5523 21 20 21H4C3.44772 21 3 20.5523 3 20V4C3 3.44772 3.44772 3 4 3ZM5 5V19H19V5H5Z" />
                    </svg>
                </button>
                <button id="titlebar-close" title="Close" on:click=on_close>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="24"
                        height="24"
                        fill="currentColor"
                    >
                        <path d="M11.9997 10.5865L16.9495 5.63672L18.3637 7.05093L13.4139 12.0007L18.3637 16.9504L16.9495 18.3646L11.9997 13.4149L7.04996 18.3646L5.63574 16.9504L10.5855 12.0007L5.63574 7.05093L7.04996 5.63672L11.9997 10.5865Z" />
                    </svg>
                </button>
            </div>
        </div>

        <Show when=move || show_settings.get()>
            <div class="fixed inset-0 z-[1000] flex items-center justify-center bg-slate-950/80 backdrop-blur-sm">
                <div class="bg-slate-900 w-[800px] max-w-[95vw] h-[500px] max-h-[85vh] rounded-xl shadow-2xl flex flex-col border border-slate-700 overflow-hidden text-slate-300">
                    <div class="flex items-center justify-between p-4 border-b border-slate-700 bg-slate-950">
                        <h2 class="text-white text-lg font-bold">"Settings"</h2>
                        <button
                            class="p-1 rounded-md text-slate-400 hover:text-white hover:bg-slate-800 transition-all"
                            on:click=move |_| set_show_settings.set(false)
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6">
                                <line x1="18" y1="6" x2="6" y2="18" />
                                <line x1="6" y1="6" x2="18" y2="18" />
                            </svg>
                        </button>
                    </div>
                    <div class="flex flex-1 overflow-hidden">
                        // Sidebar: categories derived from loaded settings
                        <nav class="w-48 border-r border-slate-700 p-2 bg-slate-950">
                            <For
                                each=categories
                                key=|cat| cat.clone()
                                children=move |cat| {
                                    let cat_for_class = cat.clone();
                                    let cat_for_click = cat.clone();
                                    let cat_for_display = cat.clone();
                                    view! {
                                        <button
                                            class=move || format!(
                                                "w-full text-left px-4 py-2 rounded-lg text-sm font-medium mb-1 transition-all border {}",
                                                if active_category.get() == cat_for_class { "bg-accent/10 text-accent border-accent/20" } else { "text-slate-400 hover:bg-slate-800 hover:text-slate-200 border-transparent" }
                                            )
                                            on:click=move |_| set_active_category.set(cat_for_click.clone())
                                        >
                                            {cat_for_display}
                                        </button>
                                    }
                                }
                            />
                        </nav>
                        // Main panel: settings for active category + static "About" panel
                        <div class="flex-1 p-6 overflow-y-auto [scrollbar-gutter:stable] bg-slate-800 text-slate-300">
                            <Show when=move || active_category.get() != "About">
                                <div class="space-y-4">
                                    <For
                                        each=active_settings
                                        key=|s| s.id.clone()
                                        children=move |setting| {
                                            let is_bool = setting.value.is_boolean();
                                            // Store the id so every closure can copy it cheaply.
                                            let stored_id = StoredValue::new(setting.id.clone());
                                            // Memo reads from all_settings; it is Copy and can be
                                            // captured independently by as many closures as needed.
                                            let checked = Memo::new(move |_| {
                                                all_settings
                                                    .get()
                                                    .into_iter()
                                                    .find(|s| s.id == stored_id.get_value())
                                                    .and_then(|s| s.value.as_bool())
                                                    .unwrap_or(false)
                                            });
                                            view! {
                                                <div class="flex items-start justify-between gap-4 py-3 border-b border-slate-700 last:border-0">
                                                    <div>
                                                        <p class="text-sm font-medium text-white">{setting.label}</p>
                                                        <p class="text-xs text-slate-400 mt-0.5">{setting.description}</p>
                                                    </div>
                                                    <Show when=move || is_bool>
                                                        <button
                                                            class=move || format!(
                                                                "relative inline-flex h-5 w-9 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none {}",
                                                                if checked.get() { "bg-accent" } else { "bg-slate-600" }
                                                            )
                                                            on:click=move |_| {
                                                                let new_val = !checked.get_untracked();
                                                                let id = stored_id.get_value();
                                                              
                                                                all_settings.update(|list| {
                                                                    if let Some(s) = list.iter_mut().find(|s| s.id == id) {
                                                                        s.value = serde_json::json!(new_val);
                                                                    }
                                                                });

                                                                match id.as_str() {
                                                                    "auto-group-dumps" => set_auto_group.set(new_val),
                                                                    "long-file-path"   => set_long_file_path.set(new_val),
                                                                    "auto-expand"      => set_auto_expand.set(new_val),
                                                                    "truncate-large-var-types" => set_truncate_large_var_types.set(new_val),
                                                                    _ => {}
                                                                }
                                                                spawn_local(async move {
                                                                    let args = serde_wasm_bindgen::to_value(
                                                                        &serde_json::json!({ "id": id, "value": new_val })
                                                                    ).unwrap();
                                                                    invoke("set_setting", args).await;
                                                                });
                                                            }
                                                        >
                                                            <span class=move || format!(
                                                                "inline-block h-4 w-4 transform rounded-full bg-white shadow transition duration-200 ease-in-out {}",
                                                                if checked.get() { "translate-x-4" } else { "translate-x-0" }
                                                            ) />
                                                        </button>
                                                    </Show>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            </Show>
                            <Show when=move || active_category.get() == "About">
                                <div class="space-y-4">
                                    <h3 class="text-white font-semibold mb-2">"About Quo"</h3>
                                    <div class="bg-slate-950/50 p-4 rounded-lg border border-slate-700">
                                        <p class="text-sm font-bold text-white">"Quo Debugging Client"</p>
                                        <p class="text-xs text-slate-500 mt-1">"Version 0.1.2"</p>
                                        <p class="text-xs text-slate-500 mt-4">"Developed by Protoqol"</p>
                                    </div>
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
        </>
    }
}
