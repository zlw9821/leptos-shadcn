use leptos::prelude::*;


#[component]
pub fn Form() -> impl IntoView {
    view! {
        <form class="form grid gap-6">
            <div class="grid gap-2">
                <label for="demo-form-text">Username</label>
                <input type="text" id="demo-form-text" placeholder="hunvreus" />
                <p class="text-muted-foreground text-sm">This is your public display name.</p>
            </div>

            <div class="grid gap-2">
                <label for="demo-form-select">Email</label>
                <select id="demo-form-select">
                    <option value="bob@example.com">m@example.com</option>
                    <option value="alice@example.com">m@google.com</option>
                    <option value="john@example.com">m@support.com</option>
                </select>
                <p class="text-muted-foreground text-sm">
                    You can manage email addresses in your email settings.
                </p>
            </div>

            <div class="grid gap-2">
                <label for="demo-form-text">Bio</label>
                <textarea id="demo-form-textarea" placeholder="I like to..." rows="3"></textarea>
                <p class="text-muted-foreground text-sm">
                    You can @mention other users and organizations.
                </p>
            </div>

            <div class="grid gap-2">
                <label for="demo-form-date">Date of birth</label>
                <input type="date" id="demo-form-date" />
                <p class="text-muted-foreground text-sm">
                    Your date of birth is used to calculate your age.
                </p>
            </div>

            <div class="flex flex-col gap-3">
                <label for="demo-form-radio">Notify me about...</label>
                <fieldset id="demo-form-radio" class="grid gap-3">
                    <label class="font-normal">
                        <input type="radio" name="demo-form-radio" value="1" checked />
                        All new messages
                    </label>
                    <label class="font-normal">
                        <input type="radio" name="demo-form-radio" value="2" />
                        Direct messages and mentions
                    </label>
                    <label class="font-normal">
                        <input type="radio" name="demo-form-radio" value="3" />
                        Nothing
                    </label>
                </fieldset>
            </div>

            <section class="grid gap-4">
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
                            Security emails
                        </label>
                        <p class="text-muted-foreground text-sm">
                            Receive emails about your account security.
                        </p>
                    </div>
                    <input type="checkbox" id="demo-form-switch-disabled" role="switch" disabled />
                </div>
            </section>

            <button type="submit" class="btn">
                Submit
            </button>
        </form>
    }
}