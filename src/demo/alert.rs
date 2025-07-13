use leptos::prelude::*;
use leptos_icons::Icon;

use crate::components::{
    alert::Alert, alert::AlertDescription, alert::AlertTitle, alert::AlertVariant,
};

#[component]
pub fn AlertDemo() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-xl items-start gap-4">
            <Alert>
                <Icon icon=icondata::LuCircleAlert />
                <AlertTitle>"Success! Your changes have been saved"</AlertTitle>
                <AlertDescription>
                    "This is an alert with icon, title and description."
                </AlertDescription>
            </Alert>
            <Alert>
                <Icon icon=icondata::LuCircleAlert />
                <AlertTitle>"This Alert has a title and an icon. No description."</AlertTitle>
            </Alert>
            <Alert variant=AlertVariant::Destructive>
                <Icon icon=icondata::LuCircleAlert />
                <AlertTitle>"Unable to process your payment."</AlertTitle>
                <AlertDescription>
                    <p>"Please verify your billing information and try again."</p>
                    <ul class="list-inside list-disc text-sm">
                        <li>"Check your card details"</li>
                        <li>"Ensure sufficient funds"</li>
                        <li>"Verify billing address"</li>
                    </ul>
                </AlertDescription>
            </Alert>
        </div>
    }
}
