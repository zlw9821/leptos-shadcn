use leptos::prelude::*;
use leptos_icons::Icon;

use crate::components::{
    badge::Badge, badge::BadgeVariant,
};

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
