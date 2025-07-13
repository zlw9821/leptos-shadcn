use leptos::prelude::*;


#[component]
pub fn AlertDialog() -> impl IntoView {
    view! {
        <button
            type="button"
            onclick="document.getElementById('alert-dialog').showModal()"
            class="btn-outline"
        >
            Open alert dialog
        </button>

        <dialog
            id="alert-dialog"
            class="dialog"
            aria-labelledby="alert-dialog-title"
            aria-describedby="alert-dialog-description"
        >
            <article>
                <header>
                    <h2 id="alert-dialog-title">Are you absolutely sure?</h2>
                    <p id="alert-dialog-description">
                        This action cannot be undone. This will permanently delete your account and remove your data from our servers.
                    </p>
                </header>

                <footer>
                    <button
                        class="btn-outline"
                        onclick="document.getElementById('alert-dialog').close()"
                    >
                        Cancel
                    </button>
                    <button
                        class="btn-primary"
                        onclick="document.getElementById('alert-dialog').close()"
                    >
                        Continue
                    </button>
                </footer>
            </article>
        </dialog>
    }
}