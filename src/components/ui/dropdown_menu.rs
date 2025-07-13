use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn DropdownMenu() -> impl IntoView {
    view! {
        <div id="demo-dropdown-menu" class="dropdown-menu">
            <button
                type="button"
                id="demo-dropdown-menu-trigger"
                aria-haspopup="menu"
                aria-controls="demo-dropdown-menu-menu"
                aria-expanded="false"
                class="btn-outline"
            >
                Open
            </button>
            <div id="demo-dropdown-menu-popover" data-popover aria-hidden="true" class="min-w-56">
                <div
                    role="menu"
                    id="demo-dropdown-menu-menu"
                    aria-labelledby="demo-dropdown-menu-trigger"
                >
                    <div role="group" aria-labelledby="account-options">
                        <div role="heading" id="account-options">
                            My Account
                        </div>
                        <div role="menuitem">
                            Profile
                            <span class="text-muted-foreground ml-auto text-xs tracking-widest">
                                "⇧⌘P"
                            </span>
                        </div>
                        <div role="menuitem">
                            Billing
                            <span class="text-muted-foreground ml-auto text-xs tracking-widest">
                                "⌘B"
                            </span>
                        </div>
                        <div role="menuitem">
                            Settings
                            <span class="text-muted-foreground ml-auto text-xs tracking-widest">
                                "⌘S"
                            </span>
                        </div>
                        <div role="menuitem">
                            Keyboard shortcuts
                            <span class="text-muted-foreground ml-auto text-xs tracking-widest">
                                "⌘K"
                            </span>
                        </div>
                    </div>
                    <hr role="separator" />
                    <div role="menuitem">GitHub</div>
                    <div role="menuitem">Support</div>
                    <div role="menuitem" aria-disabled="true">
                        API
                    </div>
                    <hr role="separator" />
                    <div role="menuitem">
                        Logout
                        <span class="text-muted-foreground ml-auto text-xs tracking-widest">
                            "⇧⌘P"
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}
