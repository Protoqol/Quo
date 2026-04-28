use quo_common::config::{DefaultValue, SETTINGS};
use serde_json::json;
use std::sync::Arc;
use tauri::Wry;
use tauri_plugin_store::Store;

pub fn check_config(store: Arc<Store<Wry>>) {
    for setting in SETTINGS {
        if store.get(setting.id).is_none() {
            let val = match setting.default {
                DefaultValue::Bool(b) => json!(b),
                DefaultValue::Str(s) => json!(s),
                DefaultValue::Float(f) => json!(f),
                DefaultValue::Int(i) => json!(i),
            };
            let _ = store.set(setting.id, val);
        }
    }
    let _ = store.save();
}
