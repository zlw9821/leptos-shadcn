use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::components::{banner::Banner, banner::Variant, home::Home};


#[component]
pub fn SomeOtherComponent() -> impl IntoView {
    view! {
        <Banner 
            variant=Variant::Green 
            title="Informational message" 
            message="Some additional text to explain said message."
        />
        <Banner 
            variant=Variant::Red 
            title="Warning message" 
            message="This is a warning."
        />
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // <Stylesheet id="leptos" href="assets/main.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
                <Route path=StaticSegment("/b") view=SomeOtherComponent/>
            </Routes>
        </Router>
    }
}
