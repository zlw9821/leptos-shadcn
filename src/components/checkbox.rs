use leptos::prelude::*;

#[component]
pub fn Checkbox() -> impl IntoView {
    view! {
        <form class="form flex flex-col gap-4">
            <header>
                <label for="demo-form-checkboxes" class="text-base leading-normal">
                    Sidebar
                </label>
                <p class="text-muted-foreground text-sm">
                    Select the items you want to display in the sidebar.
                </p>
            </header>
            <fieldset id="demo-form-checkboxes" class="flex flex-col gap-2">
                <label class="font-normal leading-tight">
                    <input type="checkbox" name="demo-form-checkboxes" value="1" checked />
                    Recents
                </label>
                <label class="font-normal leading-tight">
                    <input type="checkbox" name="demo-form-checkboxes" value="2" checked />
                    Home
                </label>
                <label class="font-normal leading-tight">
                    <input type="checkbox" name="demo-form-checkboxes" value="3" />
                    Applications
                </label>
                <label class="font-normal leading-tight">
                    <input type="checkbox" name="demo-form-checkboxes" value="4" />
                    Desktop
                </label>
                <label class="font-normal leading-tight">
                    <input type="checkbox" name="demo-form-checkboxes" value="5" />
                    Download
                </label>
                <label class="font-normal leading-tight">
                    <input type="checkbox" name="demo-form-checkboxes" value="6" />
                    Documents
                </label>
            </fieldset>
        </form>
    }
}