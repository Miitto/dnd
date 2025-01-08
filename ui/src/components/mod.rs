use dioxus::prelude::*;

pub mod info;
pub mod inputs;
pub mod nav;

mod description;
pub use description::Description;

#[component]
pub fn ReplaceLink(props: LinkProps) -> Element {
    let LinkProps {
        active_class,
        children,
        new_tab,
        to,
        class,
        onclick,
        onclick_only,
        rel,
        ..
    } = props;

    let nav = router();

    let current_url = nav.full_route_string();
    let href = match &to {
        NavigationTarget::Internal(url) => url.clone(),
        NavigationTarget::External(route) => route.clone(),
    };
    // Add the history's prefix to the href for use in the rsx
    let full_href = nav.prefix().unwrap_or_default() + &href;

    let mut class_ = String::new();
    if let Some(c) = class {
        class_.push_str(&c);
    }
    if let Some(c) = active_class {
        if href == current_url {
            if !class_.is_empty() {
                class_.push(' ');
            }
            class_.push_str(&c);
        }
    }

    let class = if class_.is_empty() {
        None
    } else {
        Some(class_)
    };

    let aria_current = (href == current_url).then_some("page");

    let tag_target = new_tab.then_some("_blank");

    let is_external = matches!(to, NavigationTarget::External(_));
    let is_router_nav = !is_external && !new_tab;
    let rel = rel.or_else(|| is_external.then_some("noopener noreferrer".to_string()));

    let do_default = onclick.is_none() || !onclick_only;

    let action = move |event: MouseEvent| {
        // Only handle events without modifiers
        if !event.modifiers().is_empty() {
            return;
        }
        // Only handle left clicks
        if event.trigger_button() != Some(dioxus_elements::input_data::MouseButton::Primary) {
            return;
        }

        // todo(jon): this is extra hacky for no reason - we should fix prevent default on Links
        if do_default && is_external {
            return;
        }

        event.prevent_default();

        if do_default && is_router_nav {
            nav.replace(to.clone());
        }

        if let Some(handler) = onclick {
            handler.call(event);
        }
    };

    let onmounted = move |event| {
        if let Some(handler) = props.onmounted {
            handler.call(event);
        }
    };

    rsx! {
        a {
            onclick: action,
            href: full_href,
            onmounted,
            class,
            rel,
            target: tag_target,
            aria_current,
            {children}
        }
    }
}
