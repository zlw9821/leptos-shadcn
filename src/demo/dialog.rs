
use leptos::prelude::*;
use tailwind_fuse::tw_merge;
use crate::components::dialog::Dialog;

use leptos::html::{ElementType, HtmlElement};


#[component]
pub fn DialogDemo() -> impl IntoView {
    view! { <Dialog /> }
}