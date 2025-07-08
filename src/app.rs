use leptos::{html::var, prelude::*};
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use leptos_icons::Icon;

use crate::components::{
    alert::Alert, alert::AlertDescription, alert::AlertTitle, alert::AlertVariant,
    button::Button, button::ButtonVariant, button::ButtonSize,
    card::Card, card::CardHeader, card::CardTitle,
    card::CardContent, card::CardFooter, card::CardDescription, 
    badge::Badge, badge::BadgeVariant,

};
use crate::components::{banner::Banner, banner::Variant, };

#[component]
pub fn AlertDemo() -> impl IntoView {
    let (variant, set_variant) = signal(AlertVariant::Default);
    view! {
        <Alert variant=variant>
            // 你可以在这里插入 <Terminal /> 或其他图标组件
            <AlertTitle>Heads up!</AlertTitle>
            <AlertDescription>
                You can add components and dependencies to your app using the cli.
            </AlertDescription>
        </Alert>
    }
}

#[component]
pub fn ButtonDemo() -> impl IntoView {
    let (variant, set_variant) =  signal(ButtonVariant::Default);
    let (size, set_size) = signal(ButtonSize::Lg);
    view! {
        <div class="flex flex-wrap items-center gap-2 md:flex-row">
            <Button variant=variant size=size>Button</Button>
        </div>
    }
}

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

#[component]
pub fn BadgeDemo() -> impl IntoView {
    let (variant, set_variant) = signal(BadgeVariant::Default);
    let (secondary, set_secondary) = signal(BadgeVariant::Secondary);
    let (destructive, set_destructive) = signal(BadgeVariant::Destructive);
    let (outline, set_outline) = signal(BadgeVariant::Outline);
    view! {
    <div class="flex flex-col items-center gap-2">
      <div class="flex w-full flex-wrap gap-2">
        <Badge>Badge</Badge>
        <Badge variant=secondary>Secondary</Badge>
        <Badge variant=destructive>Destructive</Badge>
        <Badge variant=outline>Outline</Badge>
      </div>
       <div class="flex w-full flex-wrap gap-2">
        <Badge
          variant=secondary
          class="bg-blue-500 text-white dark:bg-blue-600"
        >
          <Icon icon={icondata::LuBadgeCheck} width="2em" height="2em" style="color: green"/>
          Verified
        </Badge>
        <Badge class="h-5 min-w-5 rounded-full px-1 font-mono tabular-nums">
          8
        </Badge>
        <Badge
          class="h-5 min-w-5 rounded-full px-1 font-mono tabular-nums"
          variant=destructive
        >
          99
        </Badge>
        <Badge
          class="h-5 min-w-5 rounded-full px-1 font-mono tabular-nums"
          variant=outline
        >
          20+
        </Badge>
      </div>
    </div>
         
    }
}


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
            </Routes>
        </Router>
    }
}
