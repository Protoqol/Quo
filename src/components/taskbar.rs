use leptos::prelude::*;
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
}

#[component]
pub fn Taskbar() -> impl IntoView {
    let app_window = getCurrentWindow();
    let app_window_max = app_window.clone();
    let app_window_close = app_window.clone();

    view! {
        <div class="titlebar">
            <div data-tauri-drag-region class="bg-white">
                <div data-tauri-drag-region class="w-64 h-[30px] bg-slate-950"></div>
            </div>
            <div class="controls bg-white">
                <button title="Minimize" on:click=move |_| app_window.minimize()>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="24"
                        height="24"
                        fill="currentColor"
                    >
                        <path d="M19 11H5V13H19V11Z"></path>
                    </svg>
                </button>
                <button title="Maximize" on:click=move |_| app_window_max.toggleMaximize()>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="16"
                        height="16"
                        fill="currentColor"
                    >
                        <path d="M4 3H20C20.5523 3 21 3.44772 21 4V20C21 20.5523 20.5523 21 20 21H4C3.44772 21 3 20.5523 3 20V4C3 3.44772 3.44772 3 4 3ZM5 5V19H19V5H5Z"></path>
                    </svg>
                </button>
                <button title="Close" on:click=move |_| app_window_close.close()>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="24"
                        height="24"
                        fill="currentColor"
                    >
                        <path d="M11.9997 10.5865L16.9495 5.63672L18.3637 7.05093L13.4139 12.0007L18.3637 16.9504L16.9495 18.3646L11.9997 13.4149L7.04996 18.3646L5.63574 16.9504L10.5855 12.0007L5.63574 7.05093L7.04996 5.63672L11.9997 10.5865Z"></path>
                    </svg>
                </button>
            </div>
        </div>
    }
}