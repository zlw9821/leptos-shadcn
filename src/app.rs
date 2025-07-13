use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::demo::{
    alert::AlertDemo,
    aspect_ratio::AspectRatioDemo,
    bread_crumb::BreadcrumbDemo,
    calendar::CalendarDemo,
    progress::ProgressDemo,
    button::ButtonDemo,
    badge::BadgeDemo,
    avatar::AvatarDemo,
    dialog::DialogDemo,
    dropdown_menu::DropdownMenuDemo,
};

use crate::components::{
    select::Select,
    card::Card,
    checkbox::Checkbox,
    form::Form,
    pagination::Pagination,
    sidebar::Sidebar,
    slider::Slider,
    switch::Switch,
    table::Table,
    theme::Theme,
    toast::Toast,
    popover::Popover,
    accordion::Accordion,
    alert_dialog::AlertDialog
};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Home" />
        <main>
            <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100">
                <a href="/button">button</a>
                <a href="/alert">alert</a>
                <a href="/card">card</a>
                <a href="/badge">badge</a>
                <a href="/avatar">avatar</a>
                <a href="/accordion">accordion</a>
                <a href="/alert-dialog">alert-dialog</a>
                <a href="/aspect-ratio">aspect-ratio</a>
                <a href="/breadcrumb">breadcrumb</a>
                <a href="/calendar">calendar</a>
                <a href="/progress">progress</a>
                <a href="/dialog">dialog</a>
                <a href="/dropdown-menu">dropdown-menu</a>
                <a href="/select">select</a>
                <a href="/checkbox">checkbox</a>
                <a href="/form">form</a>
                <a href="/pagination">pagination</a>
                <a href="/sidebar">sidebar</a>
                <a href="/slider">slider</a>
                <a href="/switch">switch</a>
                <a href="/table">table</a>
                <a href="/theme">theme</a>
                <a href="/toast">toast</a>
                <a href="/popover">popover</a>
            </div>
        </main>
    }
    
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="assets/main.css" />
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home />
                <Route path=StaticSegment("/alert") view=AlertDemo />
                <Route path=StaticSegment("/button") view=ButtonDemo />
                <Route path=StaticSegment("/card") view=Card />
                <Route path=StaticSegment("/badge") view=BadgeDemo />
                <Route path=StaticSegment("/avatar") view=AvatarDemo />
                <Route path=StaticSegment("/accordion") view=Accordion />
                <Route path=StaticSegment("/alert-dialog") view=AlertDialog />
                <Route path=StaticSegment("/aspect-ratio") view=AspectRatioDemo />
                <Route path=StaticSegment("/breadcrumb") view=BreadcrumbDemo />
                <Route path=StaticSegment("/calendar") view=CalendarDemo />
                <Route path=StaticSegment("/progress") view=ProgressDemo />
                <Route path=StaticSegment("/dialog") view=DialogDemo />
                <Route path=StaticSegment("/dropdown-menu") view=DropdownMenuDemo />
                <Route path=StaticSegment("/select") view=Select />
                <Route path=StaticSegment("/checkbox") view=Checkbox />
                <Route path=StaticSegment("/form") view=Form />
                <Route path=StaticSegment("/pagination") view=Pagination />
                <Route path=StaticSegment("/sidebar") view=Sidebar />
                <Route path=StaticSegment("/slider") view=Slider />
                <Route path=StaticSegment("/switch") view=Switch />
                <Route path=StaticSegment("/table") view=Table />
                <Route path=StaticSegment("/theme") view=Theme />
                <Route path=StaticSegment("/toast") view=Toast />
                <Route path=StaticSegment("/popover") view=Popover />
            </Routes>
        </Router>
    }
}