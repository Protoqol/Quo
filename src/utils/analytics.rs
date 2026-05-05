use crate::utils::settings::AppSettings;
use leptos::prelude::{use_context, GetUntracked};
use leptos::serde_json;
use leptos::serde_json::Value;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn track_event(name: &str, props: Option<Value>) {
    #[cfg(debug_assertions)]
    println!("Tracking event {}", name);

    let enabled = use_context::<AppSettings>()
        .map(|settings| {
            settings
                .all_settings
                .get_untracked()
                .iter()
                .find(|s| s.id == "analytics")
                .and_then(|s| s.value.as_bool())
                .unwrap_or(false)
        })
        .unwrap_or(false);

    if !enabled {
        return;
    }

    let name = name.to_string();
    let props = props;

    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "name": name,
            "props": props
        }))
        .unwrap();

        invoke("plugin:aptabase|track_event", args).await;
    });
}
