use leptos::prelude::*;
use crate::components::bread_crumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator, BreadcrumbEllipsis,
};
use leptos_icons::Icon;

#[component]
pub fn BreadcrumbDemo() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbItem>
                <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator>
                <Icon icon=icondata::LuChevronRight />
            </BreadcrumbSeparator>
            <BreadcrumbItem>
                <BreadcrumbEllipsis />
            </BreadcrumbItem>
            <BreadcrumbSeparator>
                <Icon icon=icondata::LuChevronRight />
            </BreadcrumbSeparator>
            <BreadcrumbItem>
                <BreadcrumbLink href="/components/breadcrumb">"Components"</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator>
                <Icon icon=icondata::LuChevronRight />
            </BreadcrumbSeparator>
            <BreadcrumbItem>
                <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
            </BreadcrumbItem>
        </Breadcrumb>
    }
}
