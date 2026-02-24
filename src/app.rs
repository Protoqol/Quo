use crate::components::DumpItem;
use crate::components::SideBar;
use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos::reactive::traits::{Get, GetUntracked, Set};
use leptos::task::spawn_local;
use leptos_use::storage::use_local_storage;
use quo_common::payloads::IncomingQuoPayload;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (payloads, set_payloads, _) =
        use_local_storage::<Vec<IncomingQuoPayload>, JsonSerdeCodec>("payloads");

    Effect::new(move |_| {
        let handle_event = Closure::wrap(Box::new(move |event_obj: JsValue| {
            #[derive(serde::Deserialize)]
            struct TauriEvent<T> {
                payload: T,
            }

            match serde_wasm_bindgen::from_value::<TauriEvent<IncomingQuoPayload>>(event_obj) {
                Ok(event) => {
                    let mut current = GetUntracked::get_untracked(&payloads);
                    current.insert(0, event.payload);
                    Set::set(&set_payloads, current);
                }
                Err(_e) => {
                    // @TODO error handle
                    println!("Could not store incoming payload");
                }
            };
        }) as Box<dyn FnMut(JsValue)>);

        spawn_local(async move {
            listen("payload-received", &handle_event).await;
            handle_event.forget();
        });
    });

    view! {
        <div class="quo-layout">
            <SideBar />
            <main class="quo-main">
                <header class="quo-main-header">
                    <div class="input-container">
                        <label for="search">
                            <svg
                                class="search-icon"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                width="16"
                                height="16"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <circle cx="11" cy="11" r="8"></circle>
                                <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
                            </svg>
                            <input
                                type="text"
                                id="search"
                                placeholder="Search payloads... (Press '/' to focus)"
                            />
                        </label>
                        <span id="searchResult"></span>
                    </div>
                </header>
                <div class="quo-body">
                    <div id="quo">
                        <Show
                            when=move || !Get::get(&payloads).is_empty()
                            fallback=|| {
                                view! {
                                    <div id="quoNoRequestsMessage">
                                        <div class="empty-state">
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="48"
                                                height="48"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="1"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                class="mb-4 text-slate-300"
                                            >
                                                <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
                                            </svg>
                                            <p>Waiting for incoming payloads...</p>
                                            <span class="text-xs text-slate-400 mt-2">
                                                Dumps from your application will appear here automatically.
                                            </span>
                                        </div>
                                    </div>
                                }
                            }
                        >
                            <For
                                each=move || Get::get(&payloads)
                                key=|payload| payload.meta.uid.clone()
                                children=|payload: IncomingQuoPayload| {
                                    view! { <DumpItem dump=payload /> }
                                }
                            />
                        </Show>
                    </div>
                </div>
            </main>
        </div>
    }
}
