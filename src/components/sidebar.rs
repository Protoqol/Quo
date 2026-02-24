use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
        <div class="quo-sidebar">
            <div class="quo-sidebar-header">
                <img src="/public/ico-quo.svg" class="quo-logo w-8" />
                <span class="quo-logo-text">Quo</span>
            </div>
            <nav class="quo-nav">
                <div id="quo-tabs-container" class="quo-origin-tabs"></div>
            </nav>
            <div class="quo-sidebar-footer">
                <div class="settings-container mb-6 px-2">
                    <div class="flex items-center justify-between mb-3">
                        <span class="text-xs font-semibold uppercase tracking-wider text-slate-500">
                            Settings
                        </span>
                    </div>
                    <label class="flex items-center justify-between cursor-pointer group">
                        <span class="text-sm text-slate-400 group-hover:text-slate-200 transition-colors">
                            Auto group dumps
                        </span>
                        <div class="relative inline-flex items-center cursor-pointer">
                            <input
                                type="checkbox"
                                id="autoGroupToggle"
                                class="sr-only peer"
                                checked
                            />
                            <div class="w-9 h-5 bg-slate-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-slate-400 after:border-slate-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-indigo-600 peer-checked:after:bg-white"></div>
                        </div>
                    </label>
                </div>
                <div class="loader-container">
                    <div class="loader">
                        <svg
                            width="24"
                            height="24"
                            viewBox="0 0 45 45"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <g
                                fill="none"
                                fill-rule="evenodd"
                                transform="translate(1 1)"
                                stroke-width="2"
                            >
                                <circle cx="22" cy="22" r="6" stroke-opacity="0">
                                    <animate
                                        attributeName="r"
                                        begin="1.5s"
                                        dur="3s"
                                        values="6;22"
                                        calcMode="linear"
                                        repeatCount="indefinite"
                                    />
                                    <animate
                                        attributeName="stroke-opacity"
                                        begin="1.5s"
                                        dur="3s"
                                        values="1;0"
                                        calcMode="linear"
                                        repeatCount="indefinite"
                                    />
                                    <animate
                                        attributeName="stroke-width"
                                        begin="1.5s"
                                        dur="3s"
                                        values="2;0"
                                        calcMode="linear"
                                        repeatCount="indefinite"
                                    />
                                </circle>
                                <circle cx="22" cy="22" r="6" stroke-opacity="0">
                                    <animate
                                        attributeName="r"
                                        begin="3s"
                                        dur="3s"
                                        values="6;22"
                                        calcMode="linear"
                                        repeatCount="indefinite"
                                    />
                                    <animate
                                        attributeName="stroke-opacity"
                                        begin="3s"
                                        dur="3s"
                                        values="1;0"
                                        calcMode="linear"
                                        repeatCount="indefinite"
                                    />
                                    <animate
                                        attributeName="stroke-width"
                                        begin="3s"
                                        dur="3s"
                                        values="2;0"
                                        calcMode="linear"
                                        repeatCount="indefinite"
                                    />
                                </circle>
                                <circle cx="22" cy="22" r="8">
                                    <animate
                                        attributeName="r"
                                        begin="0s"
                                        dur="1.5s"
                                        values="6;1;2;3;4;5;6"
                                        calcMode="linear"
                                        repeatCount="indefinite"
                                    />
                                </circle>
                            </g>
                        </svg>
                    </div>
                </div>
                <button
                    type="button"
                    id="clearLog"
                    title="Clear all entries"
                    class="quo-btn-clear cursor-hover"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="16"
                        height="16"
                    >
                        <path fill="none" d="M0 0h24v24H0z" />
                        <path d="M17 6h5v2h-2v13a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V8H2V6h5V3a1 1 0 0 1 1-1h8a1 1 0 0 1 1 1v3zm1 2H6v12h12V8zm-4.586 6l1.768 1.768-1.414 1.414L12 15.414l-1.768 1.768-1.414-1.414L10.586 14l-1.768-1.768 1.414-1.414L12 12.586l1.768-1.768 1.414 1.414L13.414 14zM9 4v2h6V4H9z" />
                    </svg>
                    <span>Clear</span>
                </button>
            </div>
        </div>
    }
}
