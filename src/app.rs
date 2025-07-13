use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{A, Route, Router, Routes},
};

use crate::demo::{
    alert::AlertDemo, aspect_ratio::AspectRatioDemo, avatar::AvatarDemo, badge::BadgeDemo,
    bread_crumb::BreadcrumbDemo, button::ButtonDemo, calendar::CalendarDemo,
    progress::ProgressDemo,
};

use crate::components::ui::{
    accordion::Accordion, alert_dialog::AlertDialog, card::Card, checkbox::Checkbox,
    dialog::Dialog, dropdown_menu::DropdownMenu, form::Form, pagination::Pagination,
    popover::Popover, select::Select, sidebar::Sidebar, slider::Slider, switch::Switch,
    table::Table, theme::Theme, toast::Toast,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="assets/main.css" />
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Router>
            <aside class="sidebar" data-side="left" aria-hidden="false">
                <nav aria-label="Sidebar navigation">
                    <section class="scrollbar">
                        <div role="group" aria-labelledby="group-label-content-1">
                            <h3 id="group-label-content-1">Getting started</h3>

                            <ul>
                                <li>
                                    <A href="">
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                        >
                                            <path d="m7 11 2-2-2-2" />
                                            <path d="M11 13h4" />
                                            <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
                                        </svg>
                                        <span>Playground</span>
                                    </A>
                                </li>

                                <li>
                                    <A href="#">
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                        >
                                            <path d="M12 8V4H8" />
                                            <rect width="16" height="12" x="4" y="8" rx="2" />
                                            <path d="M2 14h2" />
                                            <path d="M20 14h2" />
                                            <path d="M15 13v2" />
                                            <path d="M9 13v2" />
                                        </svg>
                                        <span>Models</span>
                                    </A>
                                </li>

                                <li>
                                    <details id="submenu-content-1-3">
                                        <summary aria-controls="submenu-content-1-3-content">
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="24"
                                                height="24"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                            >
                                                <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
                                                <circle cx="12" cy="12" r="3" />
                                            </svg>
                                            Settings
                                        </summary>
                                        <ul id="submenu-content-1-3-content">
                                            <li>
                                                <A href="/button">
                                                    <span>Button</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/alert">
                                                    <span>Alert</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/card">
                                                    <span>Card</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/badge">
                                                    <span>Badge</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/avatar">
                                                    <span>Avatar</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/accordion">
                                                    <span>Accordion</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/alert-dialog">
                                                    <span>Alert Dialog</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/aspect-ratio">
                                                    <span>Aspect Ratio</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/breadcrumb">
                                                    <span>Breadcrumb</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/calendar">
                                                    <span>Calendar</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/progress">
                                                    <span>Progress</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/dialog">
                                                    <span>Dialog</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/dropdown-menu">
                                                    <span>Dropdown Menu</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/select">
                                                    <span>Select</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/checkbox">
                                                    <span>Checkbox</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/form">
                                                    <span>Form</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/pagination">
                                                    <span>Pagination</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/sidebar">
                                                    <span>Sidebar</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/slider">
                                                    <span>Slider</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/switch">
                                                    <span>Switch</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/table">
                                                    <span>Table</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/theme">
                                                    <span>Theme</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/toast">
                                                    <span>Toast</span>
                                                </A>
                                            </li>
                                            <li>
                                                <A href="/popover">
                                                    <span>Popover</span>
                                                </A>
                                            </li>
                                        </ul>
                                    </details>
                                </li>
                            </ul>
                        </div>
                    </section>
                </nav>
            </aside>

            <main>
                <Routes fallback=|| "Page not found.">
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
                    <Route path=StaticSegment("/dialog") view=Dialog />
                    <Route path=StaticSegment("/dropdown-menu") view=DropdownMenu />
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
            </main>
        </Router>
    }
}
