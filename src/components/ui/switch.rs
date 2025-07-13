use leptos::prelude::*;


#[component]
pub fn Switch() -> impl IntoView {
    let (checked, set_checked) = signal(false);

    view! {
        <form class="form grid gap-4">
            <h3 class="text-lg font-medium">Email Notifications</h3>
            <div class="gap-2 flex flex-row items-start justify-between rounded-lg border p-4 shadow-xs">
                <div class="flex flex-col gap-0.5">
                    <label for="demo-form-switch" class="leading-normal">
                        Marketing emails
                    </label>
                    <p class="text-muted-foreground text-sm">
                        Receive emails about new products, features, and more.
                    </p>
                </div>
                <input type="checkbox" id="demo-form-switch" role="switch" />
            </div>
            <div class="gap-2 flex flex-row items-start justify-between rounded-lg border p-4 shadow-xs">
                <div class="flex flex-col gap-0.5 opacity-60">
                    <label for="demo-form-switch-disabled" class="leading-normal">
                        Marketing emails
                    </label>
                    <p class="text-muted-foreground text-sm">
                        Receive emails about new products, features, and more.
                    </p>
                </div>
                <input type="checkbox" id="demo-form-switch-disabled" role="switch" disabled />
            </div>
        </form>
    }
}