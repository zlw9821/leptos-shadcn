use leptos::prelude::*;

#[component]
pub fn Button() -> impl IntoView {
    view! {
        <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
            { "Click me" }
        </button>
    }
}