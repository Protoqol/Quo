use leptos::prelude::*;
use leptos::serde_json;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SettingDto {
    pub id: String,
    pub category: String,
    pub label: String,
    pub description: String,
    pub show_in_sidebar: bool,
    pub value: serde_json::Value,
}

#[derive(Clone, Copy)]
pub struct AppSettings {
    pub all_settings: RwSignal<Vec<SettingDto>>,
    pub auto_group: RwSignal<bool>,
    pub long_file_path: RwSignal<bool>,
    pub auto_expand: RwSignal<bool>,
    pub truncate_large_var_types: RwSignal<bool>,
    pub theme: RwSignal<String>,
    pub update_available: RwSignal<bool>,
    pub latest_version: RwSignal<Option<String>>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl AppSettings {
    pub fn new() -> Self {
        Self {
            all_settings: RwSignal::new(vec![]),
            auto_group: RwSignal::new(true),
            long_file_path: RwSignal::new(false),
            auto_expand: RwSignal::new(true),
            truncate_large_var_types: RwSignal::new(false),
            theme: RwSignal::new("Quo (default)".to_string()),
            update_available: RwSignal::new(false),
            latest_version: RwSignal::new(None),
        }
    }
}
