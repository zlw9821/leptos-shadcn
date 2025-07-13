use leptos::prelude::*;

#[component]
pub fn Accordion() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <section class="accordion">
            <details class="group border-b last:border-b-0">
                <summary class="w-full focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] transition-all outline-none rounded-md">
                    <h2 class="flex flex-1 items-start justify-between gap-4 py-4 text-left text-sm font-medium hover:underline">
                        Is it accessible?
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="text-muted-foreground pointer-events-none size-4 shrink-0 translate-y-0.5 transition-transform duration-200 group-open:rotate-180"
                        >
                            <path d="m6 9 6 6 6-6" />
                        </svg>
                    </h2>
                </summary>
                <section class="pb-4">
                    <p class="text-sm">Yes. It adheres to the WAI-ARIA design pattern.</p>
                </section>
            </details>

            <details class="group border-b last:border-b-0">
                <summary class="w-full focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] transition-all outline-none rounded-md">
                    <h2 class="flex flex-1 items-start justify-between gap-4 py-4 text-left text-sm font-medium hover:underline">
                        Is it styled?
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="text-muted-foreground pointer-events-none size-4 shrink-0 translate-y-0.5 transition-transform duration-200 group-open:rotate-180"
                        >
                            <path d="m6 9 6 6 6-6" />
                        </svg>
                    </h2>
                </summary>
                <section class="pb-4">
                    <p class="text-sm">
                        "Yes. It comes with default styles that matches the other components' aesthetic."
                    </p>
                </section>
            </details>

            <details class="group border-b last:border-b-0">
                <summary class="w-full focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] transition-all outline-none rounded-md">
                    <h2 class="flex flex-1 items-start justify-between gap-4 py-4 text-left text-sm font-medium hover:underline">
                        Is it animated?
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="text-muted-foreground pointer-events-none size-4 shrink-0 translate-y-0.5 transition-transform duration-200 group-open:rotate-180"
                        >
                            <path d="m6 9 6 6 6-6" />
                        </svg>
                    </h2>
                </summary>
                <section>
                    <p class="text-sm whitespace-pre-wrap">
                        "Yes. It's animated by default, but you can disable it if you prefer."
                    </p>
                </section>
            </details>

        </section>
    }
}
