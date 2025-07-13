use leptos::prelude::*;

use crate::components::{
    avatar::{Avatar, AvatarImage, AvatarFallback},
};

#[component]
pub fn AvatarDemo() -> impl IntoView {
    view! {
        <Avatar>
            <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
            <AvatarFallback>"CN"</AvatarFallback>
        </Avatar>
    }
}
