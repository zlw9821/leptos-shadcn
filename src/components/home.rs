use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Hello() -> impl IntoView {
    let (value, set_value) = signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Leptos + Tailwindcss" />
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button
                        on:click=move |_| set_value.update(|value| *value += 1)
                        class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"
                    >
                        "+"
                    </button>
                    <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                        {value}
                    </button>
                    <button
                        on:click=move |_| set_value.update(|value| *value -= 1)
                        class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"
                        class:invisible=move || { value.get() < 1 }
                    >
                        "-"
                    </button>
                </div>
                <p class="text-center text-black-500">
                    This is a simple counter app built with Leptos and styled with Tailwind CSS.
                </p>
            </div>
        </main>
    }
}

