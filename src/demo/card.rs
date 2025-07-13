use leptos::prelude::*;

use crate::components::v1::{
    card::Card, card::CardHeader, card::CardTitle,
    card::CardContent, card::CardFooter, card::CardDescription, 
};

#[component]
pub fn CardDemo() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>Card Title</CardTitle>
                <CardDescription>Card Description</CardDescription>
            </CardHeader>
            <CardContent>
                <p>Card Content</p>
            </CardContent>
            <CardFooter>
                <p>Card Footer</p>
            </CardFooter>
        </Card>
    }
}
