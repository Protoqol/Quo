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
                <div
                    data-tauri-drag-region
                    class="w-64 h-[30px] bg-slate-950 flex items-center justify-between px-2"
                >
                    <svg
                        title="Open Quo settings"
                        class="w-5 h-5 fill-slate-700 hover:fill-slate-600 cursor-pointer"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                    >
                        <path d="M3 4H21V6H3V4ZM3 11H21V13H3V11ZM3 18H21V20H3V18Z"></path>
                    </svg>
                    <a
                        title="Report a bug"
                        href="https://github.com/Protoqol/Quo/issues/new"
                        target="_blank"
                    >
                        <svg
                            class="w-5 h-5 fill-slate-800 hover:fill-slate-600 cursor-pointer"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                        >
                            <path d="M6.05636 8.30049C6.11995 8.19854 6.18609 8.09835 6.25469 8H17.7453C17.8139 8.09835 17.88 8.19854 17.9436 8.30049L19.9641 7.13397L20.9641 8.86602L18.7512 10.1436C18.9134 10.7348 19 11.3573 19 12V13H22V15H19C19 15.9534 18.8094 16.8623 18.4642 17.6907L20.9641 19.134L19.9641 20.866L17.4383 19.4077C16.3533 20.7447 14.7853 21.6737 13 21.9291V14H11V21.9291C9.21467 21.6737 7.64665 20.7447 6.56171 19.4077L4.0359 20.866L3.0359 19.134L5.5358 17.6907C5.19061 16.8623 5 15.9534 5 15H2V13H5V12C5 11.3573 5.08661 10.7348 5.2488 10.1436L3.0359 8.86602L4.0359 7.13397L6.05636 8.30049ZM8 6C8 3.79086 9.79086 2 12 2C14.2091 2 16 3.79086 16 6H8Z"></path>
                        </svg>
                    </a>
                </div>
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
