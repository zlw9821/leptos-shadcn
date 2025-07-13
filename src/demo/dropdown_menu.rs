use leptos::prelude::*;
use tailwind_fuse::tw_merge;
use crate::components::dropdown_menu::DropdownMenu;

#[component]
pub fn DropdownMenuDemo() -> impl IntoView  {
    view! { <DropdownMenu /> }
}