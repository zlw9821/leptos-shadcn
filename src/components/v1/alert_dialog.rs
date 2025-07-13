use leptos::ev::MouseEvent;
use leptos::prelude::*;
use tailwind_fuse::*;

use crate::components::v1::button::{Button, ButtonVariant};

// --- Context ---

#[derive(Clone)]
struct AlertDialogContext {
    open: RwSignal<bool>,
}

// --- Components ---

#[component]
pub fn AlertDialog(
    #[prop(optional)] open: Option<RwSignal<bool>>,
    children: Children,
) -> impl IntoView {
    let open = open.unwrap_or_else(|| RwSignal::new(false));
    provide_context(AlertDialogContext { open });
    view! { {children()} }
}

#[component]
pub fn AlertDialogTrigger(children: Children) -> impl IntoView {
    let context =
        use_context::<AlertDialogContext>().expect("AlertDialogTrigger must be used in an AlertDialog");

    let on_click = move |_| {
        context.open.set(true);
    };

    view! {
        <div on:click=on_click data-slot="alert-dialog-trigger">
            {children()}
        </div>
    }
}

#[component]
pub fn AlertDialogPortal(children: Children) -> impl IntoView {
    let context =
        use_context::<AlertDialogContext>().expect("AlertDialogPortal must be used in an AlertDialog");

    view! {
        // <Show when=move || context.open.get()>
        // <div class="alert-dialog-portal">{children()}</div>
        // </Show>
        <div class="alert-dialog-portal">{children()}</div>
    }
}

#[component]
pub fn AlertDialogOverlay(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let context =
        use_context::<AlertDialogContext>().expect("AlertDialogOverlay must be used in an AlertDialog");

    let class = move || {
        tw_merge!(
            "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-black/50",
            class.get()
        )
    };

    view! {
        <div
            data-slot="alert-dialog-overlay"
            class=class
            data-state=move || if context.open.get() { "open" } else { "closed" }
        />
    }
}

#[component]
pub fn AlertDialogContent(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let context =
        use_context::<AlertDialogContext>().expect("AlertDialogContent must be used in an AlertDialog");

    let class = move || {
        tw_merge!(
            "bg-background data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border p-6 shadow-lg duration-200 sm:max-w-lg",
            class.get()
        )
    };

    view! {
        <AlertDialogPortal>
            <AlertDialogOverlay />
            <div
                data-slot="alert-dialog-content"
                class=class
                role="alertdialog"
                aria-modal="true"
                data-state=move || if context.open.get() { "open" } else { "closed" }
            >
                {children()}
            </div>
        </AlertDialogPortal>
    }
}

#[component]
pub fn AlertDialogHeader(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="alert-dialog-header"
            class=move || tw_merge!("flex flex-col gap-2 text-center sm:text-left", class.get())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AlertDialogFooter(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="alert-dialog-footer"
            class=move || {
                tw_merge!(
                    "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end",
                class.get()
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AlertDialogTitle(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <h2
            data-slot="alert-dialog-title"
            class=move || tw_merge!("text-lg font-semibold", class.get())
        >
            {children()}
        </h2>
    }
}

#[component]
pub fn AlertDialogDescription(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="alert-dialog-description"
            class=move || tw_merge!("text-muted-foreground text-sm", class.get())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AlertDialogAction(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] onclick: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let onclick = onclick.unwrap_or_else(|| Callback::new(|_| {}));
    view! {
        <Button class=class.get() onclick=onclick>
            {children()}
        </Button>
    }
}

#[component]
pub fn AlertDialogCancel(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let context =
        use_context::<AlertDialogContext>().expect("AlertDialogCancel must be used in an AlertDialog");

    let on_click = move |_| {
        context.open.set(false);
    };

    view! {
        <Button
            variant=Signal::from(ButtonVariant::Outline)
            onclick=Callback::new(on_click)
            class=class.get()
        >
            {children()}
        </Button>
    }
}
