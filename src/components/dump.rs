use leptos::{component, view, IntoView};
use leptos::prelude::*;
use quo_common::payloads::IncomingQuoPayload;

#[component]
#[allow(dead_code)]
pub fn DumpItem(dump: IncomingQuoPayload) -> impl IntoView {
    view! {
        <div class="text-red-500">
            <h1 class="text-red-500">#{dump.meta.uid} {dump.meta.time}</h1>
            {dump.meta.called_variable}
        </div>
    }
}
