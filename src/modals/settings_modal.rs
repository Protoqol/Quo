use crate::utils::settings::{AppSettings, SettingDto};
use crate::utils::analytics::track_event;
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
    let set_theme = settings.theme;

    let (active_category, set_active_category) = signal("Quo".to_string());
    let (available_themes, set_available_themes) = signal::<Vec<String>>(vec![]);

    Effect::new(move |_| {
        if show.get() {
            spawn_local(async move {
                let result = invoke("get_settings", JsValue::NULL).await;
                if let Ok(fresh) = serde_wasm_bindgen::from_value::<Vec<SettingDto>>(result) {
                    all_settings.set(fresh);
                }
            });
            spawn_local(async move {
                let result = invoke("get_available_themes", JsValue::NULL).await;
                if let Ok(themes) = serde_wasm_bindgen::from_value::<Vec<String>>(result) {
                    set_available_themes.set(themes);
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
            <div class="settings-modal-overlay">
                <div class="settings-modal-container">
                    <div class="settings-header">
                        <h2>"Settings"</h2>
                        <button
                            class="close-btn"
                            on:click=move |_| on_close.run(())
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <line x1="18" y1="6" x2="6" y2="18" />
                                <line x1="6" y1="6" x2="18" y2="18" />
                            </svg>
                        </button>
                    </div>
                    <div class="settings-body">
                        <nav class="settings-nav">
                            <For
                                each=categories
                                key=|cat: &String| cat.clone()
                                children=move |cat: String| {
                                    let categories_for_class = cat.clone();
                                    let categories_for_click = cat.clone();
                                    let categories_for_display = cat.clone();

                                    view! {
                                        <button
                                            class=move || format!(
                                                "nav-item {}",
                                                if active_category.get() == categories_for_class { "active" } else { "inactive" }
                                            )
                                            on:click=move |_| {
                                                set_active_category.set(categories_for_click.clone());
                                                track_event("settings_modal_category_changed", Some(serde_json::json!({
                                                    "category": categories_for_click
                                                })));
                                            }
                                        >
                                            <span>{categories_for_display.clone()}</span>
                                            <Show when=move || categories_for_display == "About" && settings.update_available.get()>
                                                <span class="update-dot">
                                                    <span class="ping"></span>
                                                    <span class="dot"></span>
                                                </span>
                                            </Show>
                                        </button>
                                    }
                                }
                            />
                        </nav>
                        // Main panel: settings for active category + static "About" panel
                        <div class="settings-content">
                            <Show when=move || active_category.get() != "About">
                                <div class="settings-list">
                                    <For
                                        each=active_settings
                                        key=|s: &SettingDto| s.id.clone()
                                        children=move |setting: SettingDto| {
                                            let is_bool = setting.value.is_boolean();
                                            let is_str = setting.value.is_string();
                                            let stored_id = StoredValue::new(setting.id.clone());
                                            let checked = Memo::new(move |_| {
                                                all_settings
                                                    .get()
                                                    .into_iter()
                                                    .find(|s| s.id == stored_id.get_value())
                                                    .and_then(|s| s.value.as_bool())
                                                    .unwrap_or(false)
                                            });
                                            let string_value = Memo::new(move |_| {
                                                all_settings
                                                    .get()
                                                    .into_iter()
                                                    .find(|s| s.id == stored_id.get_value())
                                                    .and_then(|s| s.value.as_str().map(|v| v.to_string()))
                                                    .unwrap_or_default()
                                            });

                                            view! {
                                                <div class="setting-item">
                                                    <div class="setting-info">
                                                        <p class="label">{setting.label}</p>
                                                        <p class="description">{setting.description}</p>
                                                    </div>
                                                    <Show when=move || is_bool>
                                                        <button
                                                            class=move || format!(
                                                                "setting-toggle {}",
                                                                if checked.get() { "checked" } else { "unchecked" }
                                                            )
                                                            on:click=move |_| {
                                                                let new_val = !checked.get_untracked();
                                                                let id = stored_id.get_value();
                                                              
                                                                all_settings.update(|list: &mut Vec<SettingDto>| {
                                                                    if let Some(s) = list.iter_mut().find(|s| s.id == id) {
                                                                        s.value = serde_json::json!(new_val);
                                                                    }
                                                                });

                                                                track_event("settings_modal_setting_changed", Some(serde_json::json!({
                                                                    "id": id,
                                                                    "value": new_val
                                                                })));

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
                                                                "toggle-dot {}",
                                                                if checked.get() { "translated" } else { "initial" }
                                                            ) />
                                                        </button>
                                                    </Show>
                                                    <Show when=move || is_str>
                                                        <select
                                                            class="setting-select"
                                                            on:change=move |ev| {
                                                                let new_val = event_target_value(&ev);
                                                                let id = stored_id.get_value();

                                                                all_settings.update(|list: &mut Vec<SettingDto>| {
                                                                    if let Some(s) = list.iter_mut().find(|s| s.id == id) {
                                                                        s.value = serde_json::json!(new_val);
                                                                    }
                                                                });

                                                                track_event("settings_modal_setting_changed", Some(serde_json::json!({
                                                                    "id": id,
                                                                    "value": new_val
                                                                })));

                                                                match id.as_str() {
                                                                    "theme" => set_theme.set(new_val.clone()),
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
                                                            <For
                                                                each=move || available_themes.get()
                                                                key=|t| t.clone()
                                                                children=move |theme| {
                                                                    let is_selected = theme == string_value.get();
                                                                    let theme_name = theme.clone();
                                                                    view! {
                                                                        <option value=theme_name selected=is_selected>
                                                                            {theme}
                                                                        </option>
                                                                    }
                                                                }
                                                            />
                                                        </select>
                                                    </Show>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            </Show>
                            <Show when=move || active_category.get() == "About">
                                <div class="about-panel">
                                    <h3>"About Quo"</h3>
                                    <div class="info-card">
                                        <p class="title">"Quo Debugging Client"</p>
                                        <p class="version">{move || format!("Version v{}", VERSION)}</p>
                                        <p class="author">"Developed by Protoqol"</p>
                                    </div>
                                    <Show when=move || settings.update_available.get()>
                                        <div class="update-card">
                                            <p class="message">
                                                {move || format!("A new version (v{}) is available!", settings.latest_version.get().unwrap_or_default())}
                                            </p>
                                            <p class="details">
                                                "Download the new version at "
                                                <a 
                                                    href=format!("https://quo.protoqol.sh/download?utm_source=app-{}", VERSION) 
                                                    target="_blank" 
                                                >
                                                    "quo.protoqol.sh/download"
                                                </a>
                                                " or via "
                                                <a 
                                                    href="https://github.com/Protoqol/Quo/releases" 
                                                    target="_blank" 
                                                >
                                                    "GitHub"
                                                </a>
                                                "."
                                            </p>
                                        </div>
                                    </Show>
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
