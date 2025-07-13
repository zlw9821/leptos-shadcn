use leptos::prelude::*;

use crate::components::v1::{
    button::Button, button::ButtonVariant, button::ButtonSize,
};

#[component]
pub fn ButtonDemo() -> impl IntoView {
    let (variant, set_variant) =  signal(ButtonVariant::Default);
    let (size, set_size) = signal(ButtonSize::Lg);
    view! {
        <div class="flex flex-wrap items-center gap-2 md:flex-row">
            <Button variant=variant size=size>
                Button
            </Button>
        </div>
    }
}
