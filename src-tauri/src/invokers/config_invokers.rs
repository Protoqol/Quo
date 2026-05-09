use quo_common::config::{DefaultValue, Setting, SETTINGS};
use quo_common::QUO_CONFIG_STORE_NAME;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use tauri_plugin_store::StoreExt;

#[derive(Serialize)]
pub struct SettingDto {
    #[serde(flatten)]
    pub setting: &'static Setting,
    pub value: Value,
}

#[tauri::command]
pub fn get_available_themes(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    use tauri::Manager;
    let mut themes = Vec::new();

    // Try multiple possible paths to find src/styles
    let mut paths = vec![
        "../src/styles".to_string(),
        "src/styles".to_string(),
        "./src/styles".to_string(),
    ];

    if let Ok(resource_dir) = app.path().resource_dir() {
        paths.push(resource_dir.join("src").join("styles").to_string_lossy().to_string());
        paths.push(resource_dir.join("styles").to_string_lossy().to_string());
    }

    for path in paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            themes.push(name.to_string());
                        }
                    }
                }
            }
            if !themes.is_empty() {
                break;
            }
        }
    }

    if themes.is_empty() {
        // Fallback to default themes if directory not found
        themes.push("Quo (default)".to_string());
    }

    // Ensure they are sorted and "Quo (default)" is first if present
    themes.sort();
    if let Some(pos) = themes.iter().position(|t| t == "Quo (default)") {
        let quo_default = themes.remove(pos);
        themes.insert(0, quo_default);
    }

    Ok(themes)
}

#[tauri::command]
pub fn get_settings(app: tauri::AppHandle) -> Result<Vec<SettingDto>, String> {
    let store = app
        .store(QUO_CONFIG_STORE_NAME)
        .map_err(|e| e.to_string())?;
    let dtos = SETTINGS
        .iter()
        .map(|s| {
            let value = store.get(s.id).unwrap_or_else(|| match s.default {
                DefaultValue::Bool(b) => serde_json::json!(b),
                DefaultValue::Str(v) => serde_json::json!(v),
                DefaultValue::Float(f) => serde_json::json!(f),
                DefaultValue::Int(i) => serde_json::json!(i),
            });
            SettingDto { setting: s, value }
        })
        .collect();
    Ok(dtos)
}

#[tauri::command]
pub fn set_setting(app: tauri::AppHandle, id: String, value: Value) -> Result<(), String> {
    let setting = SETTINGS
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| format!("Unknown setting: {id}"))?;

    let type_ok = matches!(
        (&setting.default, &value),
        (DefaultValue::Bool(_), Value::Bool(_))
            | (DefaultValue::Float(_), Value::Number(_))
            | (DefaultValue::Int(_), Value::Number(_))
            | (DefaultValue::Str(_), Value::String(_))
    );

    if !type_ok {
        return Err(format!("Type mismatch for setting: {id}"));
    }

    let store = app
        .store(QUO_CONFIG_STORE_NAME)
        .map_err(|e| e.to_string())?;
    store.set(&id, value);
    store.save().map_err(|e| e.to_string())
}
