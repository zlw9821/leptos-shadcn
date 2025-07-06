use leptos::prelude::*;

// 定义枚举 Variant，用于表示不同的样式类型
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Green,
    Blue,
    Amber,
    Red,
    Purple,
    Gray,
}

// 为 Variant 实现一个方法，用于返回对应的 class 字符串
impl Variant {
    fn class(&self) -> &'static str {
        match self {
            Variant::Green => "bg-green-100 border-t border-b border-green-500 text-green-700 px-4 py-3",
            Variant::Blue => "bg-blue-100 border-t border-b border-blue-500 text-blue-700 px-4 py-3",
            Variant::Amber => "bg-amber-100 border-t border-b border-amber-500 text-amber-700 px-4 py-3",
            Variant::Red => "bg-red-100 border-t border-b border-red-500 text-red-700 px-4 py-3",
            Variant::Purple => "bg-purple-100 border-t border-b border-purple-500 text-purple-700 px-4 py-3",
            Variant::Gray => "bg-gray-100 border-t border-b border-gray-500 text-gray-700 px-4 py-3",
        }
    }
}

// 修改 Banner 组件，接收 variant 参数
#[component]
pub fn Banner(
    #[prop(into)] variant: Variant,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] message: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="m-12 space-y-6">
            <div class=variant.class() role="alert">
                <p class="font-bold">{title}</p>
                <p class="text-sm">{message}</p>
            </div>
        </div>
    }
}