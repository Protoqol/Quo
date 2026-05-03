use crate::app::{AppSettings, SettingDto};
use leptos::prelude::*;
use leptos::serde_json;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn SettingsModal(
    #[prop(into)] show: Signal<bool>,
    on_close: Callback<()>,
) -> impl IntoView {
    let settings = use_context::<AppSettings>().expect("AppSettings context missing");
    let set_auto_group = settings.auto_group;
    let set_long_file_path = settings.long_file_path;
    let set_auto_expand = settings.auto_expand;
    let set_truncate_large_var_types = settings.truncate_large_var_types;
    let all_settings = settings.all_settings;

    let (active_category, set_active_category) = signal("UI".to_string());

    Effect::new(move |_| {
        if show.get() {
            spawn_local(async move {
                let result = invoke("get_settings", JsValue::NULL).await;
                if let Ok(fresh) = serde_wasm_bindgen::from_value::<Vec<SettingDto>>(result) {
                    all_settings.set(fresh);
                }
            });
        }
    });

    let active_settings = move || {
        let cat = active_category.get();
        all_settings.get().into_iter().filter(|s| s.category == cat).collect::<Vec<_>>()
    };

    let categories = move || {
        let mut seen = std::collections::HashSet::new();
        let mut categories: Vec<String> = all_settings
            .get()
            .into_iter()
            .filter_map(|s| if seen.insert(s.category.clone()) { Some(s.category) } else { None })
            .collect();

        if !seen.contains("About") {
            categories.push("About".to_string());
        }

        categories
    };

    view! {
        <Show when=move || show.get()>
            <div class="fixed inset-0 z-[1000] flex items-center justify-center bg-slate-950/80 backdrop-blur-sm">
                <div class="bg-slate-900 w-[800px] max-w-[95vw] h-[500px] max-h-[85vh] rounded-xl shadow-2xl flex flex-col border border-slate-700 overflow-hidden text-slate-300">
                    <div class="flex items-center justify-between p-4 border-b border-slate-700 bg-slate-950">
                        <h2 class="text-white text-lg font-bold">"Settings"</h2>
                        <button
                            class="p-1 rounded-md text-slate-400 hover:text-white hover:bg-slate-800 transition-all"
                            on:click=move |_| on_close.run(())
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6">
                                <line x1="18" y1="6" x2="6" y2="18" />
                                <line x1="6" y1="6" x2="18" y2="18" />
                            </svg>
                        </button>
                    </div>
                    <div class="flex flex-1 overflow-hidden">
                        <nav class="w-48 border-r border-slate-700 p-2 bg-slate-950">
                            <For
                                each=categories
                                key=|cat| cat.clone()
                                children=move |cat| {
                                    let categories_for_class = cat.clone();
                                    let categories_for_click = cat.clone();
                                    let categories_for_display = cat.clone();

                                    view! {
                                        <button
                                            class=move || format!(
                                                "w-full text-left px-4 py-2 rounded-lg text-sm font-medium mb-1 transition-all border {}",
                                                if active_category.get() == categories_for_class { "bg-accent/10 text-accent border-accent/20" } else { "text-slate-400 hover:bg-slate-800 hover:text-slate-200 border-transparent" }
                                            )
                                            on:click=move |_| set_active_category.set(categories_for_click.clone())
                                        >
                                            {categories_for_display}
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
                                            let stored_id = StoredValue::new(setting.id.clone());
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
                                        <p class="text-xs text-slate-500 mt-1">{move || format!("Version v{}", VERSION)}</p>
                                        <p class="text-xs text-slate-500 mt-4">"Developed by Protoqol"</p>
                                    </div>
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
