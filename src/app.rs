use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::demo::{
    alert::AlertDemo,
    button::ButtonDemo,
    card::CardDemo,
    badge::BadgeDemo,
    avatar::AvatarDemo,
    accordion::AccordionDemo,
};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Home"/>
        <main>
            <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100">
               <a href="/button">button</a>
               <a href="/alert">alert</a>
               <a href="/card">card</a>
               <a href="/badge">badge</a>
               <a href="/avatar">avatar</a>
               <a href="/accordion">accordion</a>
            </div>
        </main>
    }
    
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="assets/main.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
                <Route path=StaticSegment("/alert") view=AlertDemo/>
                <Route path=StaticSegment("/button") view=ButtonDemo />
                <Route path=StaticSegment("/card") view=CardDemo />
                <Route path=StaticSegment("/badge") view=BadgeDemo />
                <Route path=StaticSegment("/avatar") view=AvatarDemo />
                <Route path=StaticSegment("/accordion") view=AccordionDemo />
            </Routes>
        </Router>
    }
}
