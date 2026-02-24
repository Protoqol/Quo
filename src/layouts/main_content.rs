use leptos::{component, view, IntoView};
use leptos::prelude::*;

#[component]
pub fn MainDumpContent() -> impl IntoView {
    view! {
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
                </div>
            </div>
        </main>
    }
}
