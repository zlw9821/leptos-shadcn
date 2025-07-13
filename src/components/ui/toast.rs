use leptos::prelude::*;

#[component]
pub fn Toast() -> impl IntoView {
    view! {
        <button
            class="btn-outline"
            onclick="document.dispatchEvent(new CustomEvent('basecoat:toast', {
            detail: {
            config: {
            category: 'success',
            title: 'Success',
            description: 'A success toast called from the front-end.',
            cancel: {
            label: 'Dismiss'
            }
            }
            }
            }))"
        >
            Toast from front-end
        </button>

        <div id="toaster" class="toaster">
            <div
                class="toast"
                role="status"
                aria-atomic="true"
                aria-hidden="false"
                data-category="success"
            >
                <div class="toast-content">
                    <svg
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <circle cx="12" cy="12" r="10" />
                        <path d="m9 12 2 2 4-4" />
                    </svg>

                    <section>
                        <h2>Success</h2>
                        <p>A success toast called from the front-end.</p>
                    </section>

                    <footer>
                        <button type="button" class="btn" data-toast-action>
                            Dismiss
                        </button>
                    </footer>
                </div>
            </div>
        </div>
    }
}
