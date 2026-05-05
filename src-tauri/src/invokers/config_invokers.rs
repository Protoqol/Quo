use quo_common::config::{DefaultValue, Setting, SETTINGS};
use quo_common::QUO_CONFIG_STORE_NAME;
use serde::Serialize;
use serde_json::Value;
use tauri_plugin_store::StoreExt;

#[derive(Serialize)]
pub struct SettingDto {
    #[serde(flatten)]
    pub setting: &'static Setting,
    pub value: Value,
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
