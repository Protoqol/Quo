mod app;
mod components;
pub mod atoms;
pub mod utils;

use crate::components::Taskbar;
use app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(move || {
        view! {
            <div>
                <Taskbar />
                <App />
            </div>
        }
    })
}
