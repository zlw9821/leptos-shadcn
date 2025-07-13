use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Dialog() -> impl IntoView {
    view! {
        <button
            type="button"
            onclick="document.getElementById('demo-dialog-edit-profile').showModal()"
            class="btn-outline"
        >
            Edit Profile
        </button>

        <dialog
            id="demo-dialog-edit-profile"
            class="dialog w-full sm:max-w-[425px] max-h-[612px]"
            aria-labelledby="demo-dialog-edit-profile-title"
            aria-describedby="demo-dialog-edit-profile-description"
            onclick="this.close()"
        >
            <article onclick="event.stopPropagation()">
                <header>
                    <h2 id="demo-dialog-edit-profile-title">Edit profile</h2>
                    <p id="demo-dialog-edit-profile-description">
                        "Make changes to your profile here. Click save when you're done."
                    </p>
                </header>

                <section>
                    <form class="form grid gap-4">
                        <div class="grid gap-3">
                            <label for="demo-dialog-edit-profile-name">Name</label>
                            <input
                                type="text"
                                value="Pedro Duarte"
                                id="demo-dialog-edit-profile-name"
                                autofocus
                            />
                        </div>
                        <div class="grid gap-3">
                            <label for="demo-dialog-edit-profile-username">Username</label>
                            <input
                                type="text"
                                value="@peduarte"
                                id="demo-dialog-edit-profile-username"
                            />
                        </div>
                    </form>
                </section>

                <footer>
                    <button class="btn-outline" onclick="this.closest('dialog').close()">
                        Cancel
                    </button>
                    <button class="btn" onclick="this.closest('dialog').close()">
                        Save changes
                    </button>
                </footer>

                <form method="dialog">
                    <button aria-label="Close dialog">
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
                            class="lucide lucide-x-icon lucide-x"
                        >
                            <path d="M18 6 6 18" />
                            <path d="m6 6 12 12" />
                        </svg>
                    </button>
                </form>
            </article>
        </dialog>
    }
}