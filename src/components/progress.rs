use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Progress(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into)] value: Signal<f64>,
    #[prop(into, optional, default = 100.0.into())] max: Signal<f64>,
) -> impl IntoView {
    let percentage = Signal::derive(move || {
        let val = value.get();
        let max_val = max.get();
        if max_val == 0.0 {
            0.0
        } else {
            (val / max_val * 100.0).clamp(0.0, 100.0)
        }
    });

    view! {
        <div
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax=move || max.get()
            aria-valuenow=move || value.get()
            class=tw_merge!("relative h-4 w-full overflow-hidden rounded-full bg-secondary", class.get())
        >
            <div
                class="h-full w-full flex-1 bg-primary transition-all"
                style:transform=move || format!("translateX(-{}%)", 100.0 - percentage.get())
            />
        </div>
    }
}