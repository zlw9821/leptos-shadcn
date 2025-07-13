use chrono::{Datelike, Month, NaiveDate, Weekday};
use leptos::prelude::*;
use leptos_icons::Icon;
use tailwind_fuse::*;

use crate::components::v1::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn Calendar(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] initial_date: Option<NaiveDate>,
) -> impl IntoView {
    let initial_date = initial_date.unwrap_or_else(|| chrono::Local::now().date_naive());
    let (selected_date, set_selected_date) = signal(initial_date);
    let (view_date, set_view_date) = signal(initial_date);

    let month_name = move || {
        Month::try_from(view_date.get().month() as u8)
            .unwrap()
            .name()
    };
    let year = move || view_date.get().year();

    let days_in_month = move || {
        let date = view_date.get();
        let (year, month) = (date.year(), date.month());
        if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .unwrap()
        .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
        .num_days() as u32
    };

    let first_day_of_month_weekday = move || {
        let date = view_date.get();
        NaiveDate::from_ymd_opt(date.year(), date.month(), 1)
            .unwrap()
            .weekday()
            .num_days_from_sunday()
    };

    let prev_month = move |_| {
        set_view_date.update(|date| {
            let (year, month) = (date.year(), date.month());
            if month == 1 {
                *date = NaiveDate::from_ymd_opt(year - 1, 12, 1).unwrap();
            } else {
                *date = NaiveDate::from_ymd_opt(year, month - 1, 1).unwrap();
            }
        });
    };

    let next_month = move |_| {
        set_view_date.update(|date| {
            let (year, month) = (date.year(), date.month());
            if month == 12 {
                *date = NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap();
            } else {
                *date = NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap();
            }
        });
    };

    let weekdays = vec!["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

    view! {
        <div class=tw_merge!("rounded-lg border p-3", class.get())>
            <div class="flex items-center justify-between">
                <h2 class="text-lg font-semibold">
                    {move || format!("{} {}", month_name(), year())}
                </h2>
                <div class="flex items-center gap-2">
                    <Button
                        variant=Signal::from(ButtonVariant::Outline)
                        size=Signal::from(ButtonSize::Icon)
                        onclick=Callback::new(prev_month)
                    >
                        <Icon icon=icondata::LuChevronLeft style="h-4 w-4" />
                    </Button>
                    <Button
                        variant=Signal::from(ButtonVariant::Outline)
                        size=Signal::from(ButtonSize::Icon)
                        onclick=Callback::new(next_month)
                    >
                        <Icon icon=icondata::LuChevronRight style="h-4 w-4" />
                    </Button>
                </div>
            </div>
            <div class="grid grid-cols-7 gap-2 mt-4 text-center text-sm">
                {weekdays
                    .into_iter()
                    .map(|day| {
                        view! { <div class="text-muted-foreground">{day}</div> }
                    })
                    .collect_view()}
                {(0..first_day_of_month_weekday()).map(|_| view! { <div></div> }).collect_view()}
                {(1..=days_in_month())
                    .map(move |day| {
                        let current_date = NaiveDate::from_ymd_opt(
                                view_date.get().year(),
                                view_date.get().month(),
                                day,
                            )
                            .unwrap();
                        let is_selected = move || selected_date.get() == current_date;
                        let is_today = move || chrono::Local::now().date_naive() == current_date;
                        let day_class = move || {
                            tw_merge!(
                                "w-9 h-9 p-0 font-normal rounded-full",
                            is_today().then_some("bg-accent text-accent-foreground"),
                            is_selected().then_some("bg-primary text-primary-foreground"),
                            )
                        };

                        view! {
                            <Button
                                variant=Signal::from(ButtonVariant::Ghost)
                                size=Signal::from(ButtonSize::Icon)
                                class=day_class()
                                onclick=Callback::new(move |_| {
                                    set_selected_date.set(current_date);
                                })
                            >
                                {day}
                            </Button>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
