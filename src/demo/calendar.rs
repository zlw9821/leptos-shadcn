use leptos::prelude::*;
use crate::components::calendar::Calendar;

#[component]
pub fn CalendarDemo() -> impl IntoView {
    view! {
        <div class="flex justify-center p-6">
            <Calendar />
        </div>
    }
}
