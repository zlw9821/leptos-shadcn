use leptos::prelude::*;

use tailwind_fuse::{tw_merge};

#[component]
pub fn Avatar(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=tw_merge!(
            "relative flex size-8 shrink-0 overflow-hidden rounded-full",
            class.get()
        )>
            {children()}
        </div>
    }
}

#[component]
pub fn AvatarImage(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] src: MaybeProp<String>,
    #[prop(optional, into)] alt: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <img
            class=tw_merge!("aspect-square size-full", class.get())
            src=src.get()
            alt=alt.get()
        />
    }
}

#[component]
pub fn AvatarFallback(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=tw_merge!(
            "bg-muted flex size-full items-center justify-center rounded-full",
            class.get()
        )>
            {children()}
        </div>
    }
}
