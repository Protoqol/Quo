mod app;
mod components;
mod modals;
pub mod atoms;
pub mod utils;
pub mod layout;

use crate::layout::Taskbar;
use crate::utils::settings::AppSettings;
use app::App;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(move || {
        let settings = AppSettings::new();
        provide_context(settings);
        view! {
            <div>
                <Taskbar />
                <App />
            </div>
        }
    })
}
