use leptos::prelude::*;
use tailwind_fuse::*;

#[component]
pub fn Breadcrumb(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <nav class=class.get() aria-label="breadcrumb">
            <ol class="flex items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5">
                {children()}
            </ol>
        </nav>
    }
}

#[component]
pub fn BreadcrumbItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <li class=tw_merge!("inline-flex items-center gap-1.5", class.get())>
            {children()}
        </li>
    }
}

#[component]
pub fn BreadcrumbLink(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] href: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <a href=href.get() class=tw_merge!("transition-colors hover:text-foreground", class.get())>
            {children()}
        </a>
    }
}

#[component]
pub fn BreadcrumbPage(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <span class=tw_merge!("font-normal text-foreground", class.get()) role="link" aria-disabled="true" aria-current="page">
            {children()}
        </span>
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <li role="presentation" aria-hidden="true" class=tw_merge!("[&>svg]:size-3.5", class.get())>
            {children()}
        </li>
    }
}

#[component]
pub fn BreadcrumbEllipsis(
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <span
            role="presentation"
            aria-hidden="true"
            class=tw_merge!("flex h-9 w-9 items-center justify-center", class.get())
        >
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="1"></circle>
                <circle cx="19" cy="12" r="1"></circle>
                <circle cx="5" cy="12" r="1"></circle>
            </svg>
            <span class="sr-only">"More"</span>
        </span>
    }
}
