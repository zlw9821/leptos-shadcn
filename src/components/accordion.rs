use leptos::prelude::*;
use leptos::ev::MouseEvent;
use leptos_icons::Icon;
use tailwind_fuse::*;

// AccordionContext holds the state of the accordion
#[derive(Clone)]
struct AccordionContext {
    // which item is currently selected.
    selected: ReadSignal<Option<String>>,
    // callback to select an item.
    on_select: Callback<String>,
}

// AccordionItemContext holds information about a specific item.
#[derive(Clone)]
struct AccordionItemContext {
    // The value of the item.
    value: String,
    // Whether this item is currently selected.
    is_selected: Memo<bool>,
}

// The main Accordion component.
#[component]
pub fn Accordion(
    #[prop(optional, into)] class: MaybeProp<String>,
    // "single" or "multiple". Currently only "single" is supported.
    #[prop(optional, into)] type_: MaybeProp<String>,
    // Whether an open item can be closed by clicking its trigger.
    #[prop(optional)] collapsible: bool,
    // The item that should be open by default.
    #[prop(optional, into)] default_value: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    // We ignore the `type_` prop for now and assume it's "single".
    let _ = type_;
    
    // The state of the selected item.
    let (selected, set_selected) = signal(default_value.get());

    // The callback that is called when an item's trigger is clicked.
    let on_select = Callback::new(move |value: String| {
        set_selected.update(|s| {
            if collapsible && s.as_ref() == Some(&value) {
                *s = None;
            } else {
                *s = Some(value);
            }
        });
    });

    // Provide the context to child components.
    provide_context(AccordionContext {
        selected: selected.into(),
        on_select,
    });

    view! {
        <div class=class.get().unwrap_or_default()>
            {children()}
        </div>
    }
}

// An item within the accordion.
#[component]
pub fn AccordionItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let accordion_context =
        use_context::<AccordionContext>().expect("AccordionItem must be used in an Accordion");

    let value_clone = value.clone();
    let is_selected =
        Memo::new(move |_| accordion_context.selected.get().as_ref() == Some(&value_clone));

    provide_context(AccordionItemContext { value, is_selected });

    let class = move || {
        let class = class.get().unwrap_or_default();
        tw_merge!("border-b last:border-b-0", class)
    };

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

// The trigger that toggles an accordion item.
#[component]
pub fn AccordionTrigger(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let accordion_context =
        use_context::<AccordionContext>().expect("AccordionTrigger must be used in an Accordion");
    let item_context = use_context::<AccordionItemContext>()
        .expect("AccordionTrigger must be used in an AccordionItem");

    let on_click = move |_ev: MouseEvent| {
        // let value = item_context.value.clone();
        // // Directly call the Callback instance
        // (accordion_context.on_select)(value);
    };

    let class = move || {
        let class = class.get().unwrap_or_default();
        tw_merge!(
            "focus-visible:border-ring focus-visible:ring-ring/50 flex flex-1 items-start justify-between gap-4 rounded-md py-4 text-left text-sm font-medium transition-all outline-none hover:underline focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&[data-state=open]>svg]:rotate-180",
            class
        )
    };

    view! {
        <h3 class="flex">
            <button
                class=class
                data-state=move || if item_context.is_selected.get() { "open" } else { "closed" }
                on:click=on_click
            >
                {children()}
                <span class="text-muted-foreground pointer-events-none size-4 shrink-0 translate-y-0.5 transition-transform duration-200">
                    <Icon icon={icondata::LuChevronDown} />
                </span>
            </button>
        </h3>
    }
}


// The content of an accordion item.
#[component]
pub fn AccordionContent(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let item_context = use_context::<AccordionItemContext>()
        .expect("AccordionContent must be used in an AccordionItem");

    let outer_class = "data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down overflow-hidden text-sm";
    
    let inner_class = move || {
        let class = class.get().unwrap_or_default();
        tw_merge!("pt-0 pb-4", class)
    };

    view! {
        <div
            class=outer_class
            data-state=move || if item_context.is_selected.get() { "open" } else { "closed" }
        >
            <div class=inner_class>
                {children()}
            </div>
        </div>
    }
}
