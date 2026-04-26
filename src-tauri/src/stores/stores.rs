use std::sync::Arc;
use tauri::Wry;
use tauri_plugin_store::Store;

pub fn check_config(store: Arc<Store<Wry>>) {
    // Locale
    // Formatting date/time
    //
    // Default toggles
    //  ToggleSetting {
    //             id: "auto-group-dumps".to_string(),
    //             title: "Auto group dumps".to_string(),
    //             description:
    //                 "When dumping multiple variables at once Quo will automatically group those together."
    //                     .to_string(),
    //             position: false,
    //         },
    //         ToggleSetting {
    //             id: "auto-expand".to_string(),
    //             title: "Collapse data".to_string(),
    //             description:
    //                 "Automatically expand larger data structures"
    //                     .to_string(),
    //             position: true,
    //         },
    //         ToggleSetting {
    //             id: "long-file-path".to_string(),
    //             title: "Show full file path".to_string(),
    //             description:
    //                 "Show full file path instead of the truncated version"
    //                     .to_string(),
    //             position: false,
    //         }
}
