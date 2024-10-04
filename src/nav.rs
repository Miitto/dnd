use leptos::{component, view, Children, IntoView};
use leptos_router::A;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="h-full">
            <ul class="flex flex-col h-full">
                <NavLink href="/">Home</NavLink>
                <NavLink href="/database">Database</NavLink>
            </ul>
        </nav>
    }
}

#[component]
fn NavLink(href: &'static str, children: Children) -> impl IntoView {
    view! {
        <li class="min-h-fit flex-shrink-0 flex">
            <A href=href class="px-4 py-2 w-full cur:bg-primary/20 hover:bg-primary/10">
                {children()}
            </A>
        </li>
    }
}
