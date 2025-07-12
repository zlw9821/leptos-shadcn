use leptos::prelude::*;
use crate::components::aspect_ratio::AspectRatio;

#[component]
pub fn AspectRatioDemo() -> impl IntoView {
    view! {
        <div class="w-[450px]">
            <AspectRatio ratio=Signal::from(16.0 / 9.0)>
                <img
                    src="https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800&dpr=2&q=80"
                    alt="Photo by Drew Beamer"
                    class="rounded-md object-cover w-full h-full"
                />
            </AspectRatio>
        </div>
    }
}
