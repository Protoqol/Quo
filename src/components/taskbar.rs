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

    let (show_menu, set_show_menu) = signal(false);
    let (show_settings, set_show_settings) = signal(false);
    let (active_category, set_active_category) = signal("UI".to_string());

    let app_window = StoredValue::new(app_window);
    let on_minimize = move |_| app_window.get_value().minimize();
    let on_maximize = move |_| app_window.get_value().toggleMaximize();
    let on_close = move |_| app_window.get_value().close();
    let on_quit = move |_| app_window.get_value().close();

    view! {
        <>
        <div class="titlebar relative z-50">
            <div data-tauri-drag-region class="bg-slate-950">
                <div
                    data-tauri-drag-region
                    class="w-64 h-[30px] bg-slate-950 flex items-center justify-between px-2"
                >
                    <div class="relative flex items-center">
                        <svg
                            on:click=move |_| set_show_menu.update(|v| *v = !*v)
                            title="Open Quo settings"
                            class="w-5 h-5 fill-slate-500 hover:fill-slate-300 transition-colors cursor-pointer"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                        >
                            <path d="M3 4H21V6H3V4ZM3 11H21V13H3V11ZM3 18H21V20H3V18Z" />
                        </svg>

                        <Show when=move || show_menu.get()>
                            <div class="absolute top-[30px] left-2 w-48 bg-slate-900 border border-slate-800 rounded shadow-xl z-[100] overflow-hidden flex flex-col">
                                <button
                                    class="!w-full !flex !justify-start !items-center !px-5 !py-2.5 text-sm text-slate-300 hover:!bg-slate-800 hover:!text-white transition-colors"
                                    on:click=move |_| {
                                        set_show_settings.set(true);
                                        set_show_menu.set(false);
                                    }
                                >
                                    "Settings"
                                </button>
                                <button
                                    class="group !w-full !flex !justify-start !items-center !px-5 !py-2.5 text-sm text-slate-300 hover:!bg-slate-800 hover:!text-white transition-colors border-t border-slate-800 gap-x-1"
                                    on:click=on_quit
                                >
                                    <span>Quit</span>
                                    <span class="text-slate-700 font-light group-hover:text-slate-500 transition-colors">"(pro)"</span>
                                    <span>Quo</span>
                                </button>
                            </div>
                            <div
                                class="fixed inset-0 z-[90]"
                                on:click=move |_| set_show_menu.set(false)
                            />
                        </Show>
                    </div>
                    <a
                        title="Report a bug"
                        href="https://github.com/Protoqol/Quo/issues/new"
                        target="_blank"
                    >
                        <svg
                            class="w-5 h-5 fill-slate-500 hover:fill-slate-300 transition-colors cursor-pointer"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                        >
                            <path d="M6.05636 8.30049C6.11995 8.19854 6.18609 8.09835 6.25469 8H17.7453C17.8139 8.09835 17.88 8.19854 17.9436 8.30049L19.9641 7.13397L20.9641 8.86602L18.7512 10.1436C18.9134 10.7348 19 11.3573 19 12V13H22V15H19C19 15.9534 18.8094 16.8623 18.4642 17.6907L20.9641 19.134L19.9641 20.866L17.4383 19.4077C16.3533 20.7447 14.7853 21.6737 13 21.9291V14H11V21.9291C9.21467 21.6737 7.64665 20.7447 6.56171 19.4077L4.0359 20.866L3.0359 19.134L5.5358 17.6907C5.19061 16.8623 5 15.9534 5 15H2V13H5V12C5 11.3573 5.08661 10.7348 5.2488 10.1436L3.0359 8.86602L4.0359 7.13397L6.05636 8.30049ZM8 6C8 3.79086 9.79086 2 12 2C14.2091 2 16 3.79086 16 6H8Z" />
                        </svg>
                    </a>
                </div>
            </div>
            <div class="controls bg-slate-950">
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

        <Show when=move || show_settings.get()>
            <div class="fixed inset-0 z-[1000] flex items-center justify-center bg-slate-950/80 backdrop-blur-sm">
                <div class="bg-slate-900 w-[800px] max-w-[95vw] h-[500px] max-h-[85vh] rounded-xl shadow-2xl flex flex-col border border-slate-700 overflow-hidden text-slate-300">
                    <div class="flex items-center justify-between p-4 border-b border-slate-700 bg-slate-950">
                        <h2 class="text-white text-lg font-bold">"Settings"</h2>
                        <button
                            class="p-1 rounded-md text-slate-400 hover:text-white hover:bg-slate-800 transition-all"
                            on:click=move |_| set_show_settings.set(false)
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6">
                                <line x1="18" y1="6" x2="6" y2="18" />
                                <line x1="6" y1="6" x2="18" y2="18" />
                            </svg>
                        </button>
                    </div>
                    <div class="flex flex-1 overflow-hidden">
                        <nav class="w-48 border-r border-slate-700 p-2 bg-slate-950">
                            <For
                                each=move || vec!["UI", "Server", "Privacy", "About"]
                                key=|cat| cat.to_string()
                                children=move |cat| {
                                    let category = cat.to_string();
                                    let cat_clone = category.clone();
                                    view! {
                                        <button
                                            class=move || format!(
                                                "w-full text-left px-4 py-2 rounded-lg text-sm font-medium mb-1 transition-all border {}",
                                                if active_category.get() == category { "bg-accent/10 text-accent border-accent/20" } else { "text-slate-400 hover:bg-slate-800 hover:text-slate-200 border-transparent" }
                                            )
                                            on:click=move |_| set_active_category.set(cat_clone.clone())
                                        >
                                            {cat}
                                        </button>
                                    }
                                }
                            />
                        </nav>
                        <div class="flex-1 p-6 overflow-y-auto bg-slate-800 text-slate-300">
                            <Show when=move || active_category.get() == "UI">
                                <div class="space-y-4">
                                    <h3 class="text-white font-semibold mb-2">"Interface Settings"</h3>
                                    <p class="text-sm text-slate-400">"Customize how Quo looks and feels."</p>
                                </div>
                            </Show>
                            <Show when=move || active_category.get() == "Server">
                                <div class="space-y-4">
                                    <h3 class="text-white font-semibold mb-2">"Server Configuration"</h3>
                                    <p class="text-sm text-slate-400">"Manage connection and port settings."</p>
                                </div>
                            </Show>
                            <Show when=move || active_category.get() == "Privacy">
                                <div class="space-y-4">
                                    <h3 class="text-white font-semibold mb-2">"Privacy & Security"</h3>
                                    <p class="text-sm text-slate-400">"Control data collection and storage."</p>
                                </div>
                            </Show>
                            <Show when=move || active_category.get() == "About">
                                <div class="space-y-4">
                                    <h3 class="text-white font-semibold mb-2">"About Quo"</h3>
                                    <div class="bg-slate-950/50 p-4 rounded-lg border border-slate-700">
                                        <p class="text-sm font-bold text-white">"Quo Debugging Client"</p>
                                        <p class="text-xs text-slate-500 mt-1">"Version 0.1.0"</p>
                                        <p class="text-xs text-slate-500 mt-4">"Developed by Protoqol"</p>
                                    </div>
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
        </>
    }
}
