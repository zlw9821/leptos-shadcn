use leptos::prelude::*;

#[component]
pub fn Slider() -> impl IntoView {
    let (value, set_value) = signal(50.0);

    let percent = Signal::derive(move || {
        let min = 0.0;
        let max = 100.0;
        let v = value.get();
        if max - min < f64::EPSILON {
            0.0
        } else {
            ((v - min) / (max - min)) * 100.0
        }
    });

    view! {
        <input
            type="range"
            class="input w-full"
            min="0"
            max="100"
            prop:value=move || value.get().to_string()
            on:input=move |ev| {
                if let Some(val) = event_target_value(&ev).parse::<f64>().ok() {
                    set_value.set(val);
                }
            }
            style=move || format!("--slider-value: {}%;", percent.get())
        />
        <div>{move || format!("当前值: {:.0}", value.get())}</div>
    }
}