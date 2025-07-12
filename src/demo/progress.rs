use leptos::prelude::*;

use crate::components::progress::Progress;
use std::time::Duration;

#[component]
pub fn ProgressDemo() -> impl IntoView {
    let (progress, set_progress) = signal(50.0);

    view! {
        <div class="w-[60%]">
            <Progress value=progress />
        </div>
    }
}
