use gloo_timers::future::sleep;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToastMessage {
    pub id: usize,
    pub text: String,
    pub toast_type: ToastType,
    pub visible: RwSignal<bool>,
}

#[macro_export]
macro_rules! toast {
    ($text:expr) => {
        $crate::atoms::add_toast($text.to_string(), $crate::atoms::ToastType::Info)
    };
    ($text:expr, $type:expr) => {
        $crate::atoms::add_toast($text.to_string(), $type)
    };
}

pub fn add_toast(text: String, toast_type: ToastType) {
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    if let Some(set_toasts) = use_context::<WriteSignal<Vec<ToastMessage>>>() {
        let visible = RwSignal::new(true);

        set_toasts.update(|toasts| {
            toasts.push(ToastMessage {
                id,
                text,
                toast_type,
                visible,
            });
        });

        leptos::task::spawn_local(async move {
            sleep(std::time::Duration::from_millis(1500)).await;
            visible.set(false);

            sleep(std::time::Duration::from_millis(300)).await;

            set_toasts.update(|toasts| {
                toasts.retain(|t| t.id != id);
            });
        });
    }
}

pub fn remove_toast(id: usize) {
    if let Some(set_toasts) = use_context::<WriteSignal<Vec<ToastMessage>>>() {
        set_toasts.update(|toasts| {
            toasts.retain(|t| t.id != id);
        });
    }
}

#[component]
pub fn Toast(message: ToastMessage) -> impl IntoView {
    let id = message.id;
    let text = message.text.clone();

    let icon = match message.toast_type {
        ToastType::Success => view! {
            <svg
                class="w-6 h-6 text-green-500"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M5 11.917 9.724 16.5 19 7.5"
                />
            </svg>
        },
        ToastType::Info => view! {
            <svg
                class="w-6 h-6 text-blue-500"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
                />
            </svg>
        },
        ToastType::Warning => view! {
            <svg
                class="w-6 h-6 text-yellow-500"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 8v4m0 4h.01M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
                />
            </svg>
        },
        ToastType::Error => view! {
            <svg
                class="w-6 h-6 text-red-500"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M6 18 17.94 6M18 18 6.06 6"
                />
            </svg>
        },
    };

    view! {
        <div
            class=move || {
                let base_class = "flex items-center w-full max-w-xs p-4 text-slate-300 bg-slate-900 rounded-lg shadow-lg border border-slate-800 transition-all duration-300 overflow-hidden mb-2";
                if message.visible.get() {
                    format!("{} animate-toast-in", base_class)
                } else {
                    format!("{} animate-toast-out", base_class)
                }
            }
            style=move || {
                if !message.visible.get() {
                    "margin-top: 0; margin-bottom: 0; padding-top: 0; padding-bottom: 0; max-height: 0; border: none; opacity: 0;"
                        .to_string()
                } else {
                    "max-height: 200px;".to_string()
                }
            }
            role="alert"
        >
            <div class="inline-flex items-center justify-center flex-shrink-0 w-8 h-8 rounded-lg">
                {icon}
            </div>
            <div class="ms-3 text-sm font-normal">{text}</div>
            <button
                type="button"
                class="ms-auto -mx-1.5 -my-1.5 bg-transparent text-slate-500 hover:text-white rounded-lg focus:ring-2 focus:ring-slate-300 p-1.5 hover:bg-slate-800 inline-flex items-center justify-center h-8 w-8"
                on:click=move |_| remove_toast(id)
                aria-label="Close"
            >
                <span class="sr-only">Close</span>
                <svg
                    class="w-3 h-3"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 14 14"
                >
                    <path
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
                    />
                </svg>
            </button>
        </div>
    }
}

#[component]
pub fn Toaster() -> impl IntoView {
    let toasts = use_context::<ReadSignal<Vec<ToastMessage>>>()
        .expect("Toaster must be used within a ToastContext (provide_toast_context)");

    view! {
        <div class="fixed top-[35px] right-4 z-50 flex flex-col pointer-events-none">
            <div class="pointer-events-auto flex flex-col">
                <For
                    each=move || toasts.get()
                    key=|toast| toast.id
                    children=|toast| view! { <Toast message=toast /> }
                />
            </div>
        </div>
    }
}

pub fn provide_toast_context() {
    let (toasts, set_toasts) = signal(Vec::<ToastMessage>::new());
    provide_context(toasts);
    provide_context(set_toasts);
}
