use crate::utils::settings::AppSettings;
use crate::modals::SettingsModal;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::utils::analytics::track_event;

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

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Taskbar() -> impl IntoView {
    let settings = use_context::<AppSettings>().expect("AppSettings context missing");
    let update_available = settings.update_available;
    let app_window = getCurrentWindow();
    let (show_menu, set_show_menu) = signal(false);
    let (show_settings, set_show_settings) = signal(false);

    let app_window = StoredValue::new(app_window);
    let on_minimize = move |_| app_window.get_value().minimize();
    let on_maximize = move |_| app_window.get_value().toggleMaximize();
    let on_close = move |_| app_window.get_value().close();
    let on_quit = move |_| app_window.get_value().close();

    view! {
        <>
        <div class="titlebar">
            <div data-tauri-drag-region class="titlebar-drag-region">
                <div
                    data-tauri-drag-region
                    class="titlebar-left-container"
                >
                    <div class="menu-button-container">
                        <div class="menu-button-wrapper">
                            <svg
                                on:click=move |_| set_show_menu.update(|v| *v = !*v)
                                title="Open Quo settings"
                                class="menu-button"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="currentColor"
                            >
                                <path d="M3 4H21V6H3V4ZM3 11H21V13H3V11ZM3 18H21V20H3V18Z" />
                            </svg>
                            <Show when=move || update_available.get()>
                                <span class="update-ping">
                                    <span class="ping-anim"></span>
                                    <span class="ping-dot"></span>
                                </span>
                            </Show>
                        </div>

                        <Show when=move || show_menu.get()>
                            <div class="taskbar-menu">
                                <button
                                    class="menu-item"
                                    on:click=move |_| {
                                        set_show_settings.set(true);
                                        set_show_menu.set(false);
                                        track_event("settings_opened", None);
                                    }
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                                        <path d="M3.33946 17.0002C2.90721 16.2515 2.58277 15.4702 2.36133 14.6741C3.3338 14.1779 3.99972 13.1668 3.99972 12.0002C3.99972 10.8345 3.3348 9.824 2.36353 9.32741C2.81025 7.71651 3.65857 6.21627 4.86474 4.99001C5.7807 5.58416 6.98935 5.65534 7.99972 5.072C9.01009 4.48866 9.55277 3.40635 9.4962 2.31604C11.1613 1.8846 12.8847 1.90004 14.5031 2.31862C14.4475 3.40806 14.9901 4.48912 15.9997 5.072C17.0101 5.65532 18.2187 5.58416 19.1346 4.99007C19.7133 5.57986 20.2277 6.25151 20.66 7.00021C21.0922 7.7489 21.4167 8.53025 21.6381 9.32628C20.6656 9.82247 19.9997 10.8336 19.9997 12.0002C19.9997 13.166 20.6646 14.1764 21.6359 14.673C21.1892 16.2839 20.3409 17.7841 19.1347 19.0104C18.2187 18.4163 17.0101 18.3451 15.9997 18.9284C14.9893 19.5117 14.4467 20.5941 14.5032 21.6844C12.8382 22.1158 11.1148 22.1004 9.49633 21.6818C9.55191 20.5923 9.00929 19.5113 7.99972 18.9284C6.98938 18.3451 5.78079 18.4162 4.86484 19.0103C4.28617 18.4205 3.77172 17.7489 3.33946 17.0002ZM8.99972 17.1964C10.0911 17.8265 10.8749 18.8227 11.2503 19.9659C11.7486 20.0133 12.2502 20.014 12.7486 19.9675C13.1238 18.8237 13.9078 17.8268 14.9997 17.1964C16.0916 16.5659 17.347 16.3855 18.5252 16.6324C18.8146 16.224 19.0648 15.7892 19.2729 15.334C18.4706 14.4373 17.9997 13.2604 17.9997 12.0002C17.9997 10.74 18.4706 9.5632 19.2729 8.6665C19.1688 8.4405 19.0538 8.21822 18.9279 8.00021C18.802 7.78219 18.667 7.57148 18.5233 7.36842C17.3457 7.61476 16.0911 7.43414 14.9997 6.80405C13.9083 6.17395 13.1246 5.17768 12.7491 4.03455C12.2509 3.98714 11.7492 3.98646 11.2509 4.03292C10.8756 5.17671 10.0916 6.17364 8.99972 6.80405C7.9078 7.43447 6.65245 7.61494 5.47428 7.36803C5.18485 7.77641 4.93463 8.21117 4.72656 8.66637C5.52881 9.56311 5.99972 10.74 5.99972 12.0002C5.99972 13.2604 5.52883 14.4372 4.72656 15.3339C4.83067 15.5599 4.94564 15.7822 5.07152 16.0002C5.19739 16.2182 5.3324 16.4289 5.47612 16.632C6.65377 16.3857 7.90838 16.5663 8.99972 17.1964ZM11.9997 15.0002C10.3429 15.0002 8.99972 13.6571 8.99972 12.0002C8.99972 10.3434 10.3429 9.00021 11.9997 9.00021C13.6566 9.00021 14.9997 10.3434 14.9997 12.0002C14.9997 13.6571 13.6566 15.0002 11.9997 15.0002ZM11.9997 13.0002C12.552 13.0002 12.9997 12.5525 12.9997 12.0002C12.9997 11.4479 12.552 11.0002 11.9997 11.0002C11.4474 11.0002 10.9997 11.4479 10.9997 12.0002C10.9997 12.5525 11.4474 13.0002 11.9997 13.0002Z" />
                                    </svg>
                                    <span>"Settings"</span>
                                </button>
                                <a
                                    class="menu-item cursor-pointer"
                                    href="https://github.com/Protoqol/Quo/issues/new"
                                    target="_blank"
                                    on:click=move |_| {
                                        set_show_menu.set(false);
                                        track_event("report_bug_clicked", None);
                                    }
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                                        <path d="M13 19.9C15.2822 19.4367 17 17.419 17 15V12C17 11.299 16.8564 10.6219 16.5846 10H7.41538C7.14358 10.6219 7 11.299 7 12V15C7 17.419 8.71776 19.4367 11 19.9V14H13V19.9ZM5.5358 17.6907C5.19061 16.8623 5 15.9534 5 15H2V13H5V12C5 11.3573 5.08661 10.7348 5.2488 10.1436L3.0359 8.86602L4.0359 7.13397L6.05636 8.30049C6.11995 8.19854 6.18609 8.09835 6.25469 8H17.7453C17.8139 8.09835 17.88 8.19854 17.9436 8.30049L19.9641 7.13397L20.9641 8.86602L18.7512 10.1436C18.9134 10.7348 19 11.3573 19 12V13H22V15H19C19 15.9534 18.8094 16.8623 18.4642 17.6907L20.9641 19.134L19.9641 20.866L17.4383 19.4077C16.1549 20.9893 14.1955 22 12 22C9.80453 22 7.84512 20.9893 6.56171 19.4077L4.0359 20.866L3.0359 19.134L5.5358 17.6907ZM8 6C8 3.79086 9.79086 2 12 2C14.2091 2 16 3.79086 16 6H8Z" />
                                    </svg>
                                    <span>"Report a bug"</span>
                                </a>
                                <hr />
                                <button
                                    class="group menu-item quit-item"
                                    on:click=on_quit
                                >
                                    <span class="quit-text">
                                        <span>"Quit"</span>
                                        <span class="pro-badge">"(pro)"</span>
                                        <span>"Quo"</span>
                                    </span>
                                </button>
                            </div>
                            <div
                                class="menu-overlay"
                                on:click=move |_| set_show_menu.set(false)
                            />
                        </Show>
                    </div>
                </div>
            </div>
            <div class="controls">
                <button title="Minimize" on:click=on_minimize>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="24"
                        height="24"
                        fill="currentColor"
                    >
                        <path d="M19 11H5V13H19V11Z" />
                    </svg>
                </button>
                <button title="Maximize" on:click=on_maximize>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="16"
                        height="16"
                        fill="currentColor"
                    >
                        <path d="M4 3H20C20.5523 3 21 3.44772 21 4V20C21 20.5523 20.5523 21 20 21H4C3.44772 21 3 20.5523 3 20V4C3 3.44772 3.44772 3 4 3ZM5 5V19H19V5H5Z" />
                    </svg>
                </button>
                <button id="titlebar-close" title="Close" on:click=on_close>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="24"
                        height="24"
                        fill="currentColor"
                    >
                        <path d="M11.9997 10.5865L16.9495 5.63672L18.3637 7.05093L13.4139 12.0007L18.3637 16.9504L16.9495 18.3646L11.9997 13.4149L7.04996 18.3646L5.63574 16.9504L10.5855 12.0007L5.63574 7.05093L7.04996 5.63672L11.9997 10.5865Z" />
                    </svg>
                </button>
            </div>
        </div>

        <SettingsModal
            show=show_settings
            on_close=Callback::new(move |_| set_show_settings.set(false))
        />
        </>
    }
}
