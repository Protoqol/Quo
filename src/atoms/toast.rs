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
                class="success"
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
                class="info"
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
                class="warning"
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
                class="error"
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
                if message.visible.get() { "toast visible" } else { "toast hidden" }
            }
            role="alert"
        >
            <div class="toast-icon-container">
                {icon}
            </div>
            <div class="toast-text">{text}</div>
            <button
                type="button"
                class="toast-close-btn"
                on:click=move |_| remove_toast(id)
                aria-label="Close"
            >
                <span class="sr-only">Close</span>
                <svg
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
        <div class="toaster-container">
            <div class="toaster-content">
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
