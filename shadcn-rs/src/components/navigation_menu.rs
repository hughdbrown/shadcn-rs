//! Navigation Menu component
//!
//! A collection of links for navigating websites.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{NavigationMenu, NavigationMenuList, NavigationMenuItem, NavigationMenuTrigger, NavigationMenuContent, NavigationMenuLink};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <NavigationMenu>
//!             <NavigationMenuList>
//!                 <NavigationMenuItem>
//!                     <NavigationMenuTrigger>{ "Products" }</NavigationMenuTrigger>
//!                     <NavigationMenuContent>
//!                         <NavigationMenuLink href="/products/1">{ "Product 1" }</NavigationMenuLink>
//!                         <NavigationMenuLink href="/products/2">{ "Product 2" }</NavigationMenuLink>
//!                     </NavigationMenuContent>
//!                 </NavigationMenuItem>
//!                 <NavigationMenuItem>
//!                     <NavigationMenuLink href="/about">{ "About" }</NavigationMenuLink>
//!                 </NavigationMenuItem>
//!             </NavigationMenuList>
//!         </NavigationMenu>
//!     }
//! }
//! ```

use crate::components::tabs::{TabsOrientation, query_all, roving_index};
use crate::hooks::{use_click_outside_conditional, use_escape_key_conditional};
use crate::utils::{focus_element, generate_id};
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

/// Context shared by `NavigationMenu`: which item is open.
#[derive(Clone, PartialEq)]
pub struct NavigationMenuContext {
    /// Value of the open item, if any
    pub value: Option<AttrValue>,
    /// Opens the item with the given value, or closes with `None`
    pub set_value: Callback<Option<AttrValue>>,
}

/// Context shared by a `NavigationMenuItem` with its trigger, content and links.
#[derive(Clone, PartialEq)]
pub struct NavigationMenuItemContext {
    /// Value identifying this item
    pub value: AttrValue,
    /// Whether this item's content is shown
    pub open: bool,
    /// Opens or closes this item
    pub set_open: Callback<bool>,
    /// Id of the trigger button
    pub trigger_id: AttrValue,
    /// Id of the content element
    pub content_id: AttrValue,
}

/// Normalizes a value: an empty string means "no item open".
fn open_value(value: Option<AttrValue>) -> Option<AttrValue> {
    value.filter(|value| !value.is_empty())
}

/// Navigation menu container properties
#[derive(Properties, PartialEq, Clone)]
pub struct NavigationMenuProps {
    /// Value of the open item (controlled). `Some("")` means no item is open.
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Value of the item open initially (uncontrolled)
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Called with the newly open item's value, or `None` when it closes
    #[prop_or_default]
    pub on_value_change: Option<Callback<Option<AttrValue>>>,

    /// Orientation of the navigation menu ("horizontal" or "vertical")
    #[prop_or(AttrValue::from("horizontal"))]
    pub orientation: AttrValue,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Navigation menu container component
///
/// The main container for navigation menus. Tracks which item is open:
/// clicking a trigger toggles its content, hovering another trigger while
/// one is open switches to it, and clicking outside or pressing Escape
/// closes it.
///
/// # Accessibility
/// - `aria-expanded` / `aria-controls` on triggers
/// - ArrowLeft/ArrowRight move between top-level triggers, ArrowDown opens
/// - Escape closes and returns focus to the trigger
#[function_component(NavigationMenu)]
pub fn navigation_menu(props: &NavigationMenuProps) -> Html {
    let NavigationMenuProps {
        value,
        default_value,
        on_value_change,
        orientation,
        class,
        children,
    } = props.clone();

    let internal = use_state(|| open_value(default_value));
    let root_ref = use_node_ref();

    let current = if value.is_some() {
        open_value(value)
    } else {
        (*internal).clone()
    };
    let is_open = current.is_some();

    // The internal value always follows changes made here, so a parent that
    // drops back to `value: None` after closing stays closed.
    let set_value = {
        let internal = internal.clone();
        Callback::from(move |new_value: Option<AttrValue>| {
            internal.set(new_value.clone());
            if let Some(callback) = on_value_change.as_ref() {
                callback.emit(new_value);
            }
        })
    };

    {
        let set_value = set_value.clone();
        let root_ref = root_ref.clone();
        use_escape_key_conditional(
            move || {
                if let Some(root) = root_ref.cast::<Element>()
                    && let Ok(Some(trigger)) =
                        root.query_selector(".navigation-menu-trigger[data-state='open']")
                {
                    focus_element(&trigger);
                }
                set_value.emit(None);
            },
            is_open,
        );
    }

    {
        let set_value = set_value.clone();
        use_click_outside_conditional(root_ref.clone(), move || set_value.emit(None), is_open);
    }

    let orientation_class = format!("navigation-menu-{}", orientation);

    let classes: Classes = vec![
        Classes::from("navigation-menu"),
        Classes::from(orientation_class),
        class,
    ]
    .into_iter()
    .collect();

    let context = NavigationMenuContext {
        value: current,
        set_value,
    };

    html! {
        <ContextProvider<NavigationMenuContext> context={context}>
            <nav ref={root_ref} class={classes} data-orientation={orientation}>
                { children }
            </nav>
        </ContextProvider<NavigationMenuContext>>
    }
}

/// Navigation menu list properties
#[derive(Properties, PartialEq, Clone)]
pub struct NavigationMenuListProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Navigation menu list component
///
/// Contains navigation menu items.
#[function_component(NavigationMenuList)]
pub fn navigation_menu_list(props: &NavigationMenuListProps) -> Html {
    let NavigationMenuListProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("navigation-menu-list"), class]
        .into_iter()
        .collect();

    html! {
        <ul class={classes}>
            { children }
        </ul>
    }
}

/// Navigation menu item properties
#[derive(Properties, PartialEq, Clone)]
pub struct NavigationMenuItemProps {
    /// Value identifying this item. Generated when omitted.
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Navigation menu item component
///
/// A single item in the navigation menu.
#[function_component(NavigationMenuItem)]
pub fn navigation_menu_item(props: &NavigationMenuItemProps) -> Html {
    let NavigationMenuItemProps {
        value,
        class,
        children,
    } = props.clone();

    let generated = use_state(|| AttrValue::from(generate_id("navigation-menu-item")));
    let value = value.unwrap_or_else(|| (*generated).clone());
    let menu = use_context::<NavigationMenuContext>();
    // Standalone items (outside a NavigationMenu) keep their own open state.
    let local_open = use_state(|| false);

    let open = match menu.as_ref() {
        Some(menu) => menu.value.as_ref() == Some(&value),
        None => *local_open,
    };

    let set_open = {
        let value = value.clone();
        let local_open = local_open.clone();
        Callback::from(move |open: bool| match menu.as_ref() {
            Some(menu) => menu
                .set_value
                .emit(if open { Some(value.clone()) } else { None }),
            None => local_open.set(open),
        })
    };

    let context = NavigationMenuItemContext {
        trigger_id: AttrValue::from(format!("{}-trigger", *generated)),
        content_id: AttrValue::from(format!("{}-content", *generated)),
        value,
        open,
        set_open,
    };

    let classes: Classes = vec![Classes::from("navigation-menu-item"), class]
        .into_iter()
        .collect();

    html! {
        <ContextProvider<NavigationMenuItemContext> context={context}>
            <li class={classes} data-state={if open { "open" } else { "closed" }}>
                { children }
            </li>
        </ContextProvider<NavigationMenuItemContext>>
    }
}

/// Navigation menu trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct NavigationMenuTriggerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Mouse enter handler (for hover triggers)
    #[prop_or_default]
    pub onmouseenter: Option<Callback<MouseEvent>>,

    /// Mouse leave handler (for hover triggers)
    #[prop_or_default]
    pub onmouseleave: Option<Callback<MouseEvent>>,

    /// Children elements
    pub children: Children,
}

/// Navigation menu trigger component
///
/// Toggles the content of its item.
#[function_component(NavigationMenuTrigger)]
pub fn navigation_menu_trigger(props: &NavigationMenuTriggerProps) -> Html {
    let NavigationMenuTriggerProps {
        class,
        onclick,
        onmouseenter,
        onmouseleave,
        children,
    } = props.clone();

    let item = use_context::<NavigationMenuItemContext>();
    let menu = use_context::<NavigationMenuContext>();
    let open = item.as_ref().is_some_and(|item| item.open);

    let classes: Classes = vec![Classes::from("navigation-menu-trigger"), class]
        .into_iter()
        .collect();

    let handle_click = {
        let item = item.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(callback) = onclick.as_ref() {
                callback.emit(e);
            }
            if let Some(item) = item.as_ref() {
                item.set_open.emit(!item.open);
            }
        })
    };

    let handle_mouseenter = {
        let item = item.clone();
        let menu = menu.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(callback) = onmouseenter.as_ref() {
                callback.emit(e);
            }
            // While another item is open, hovering switches to this one.
            if let (Some(item), Some(menu)) = (item.as_ref(), menu.as_ref())
                && menu.value.is_some()
                && !item.open
            {
                item.set_open.emit(true);
            }
        })
    };

    let onkeydown = {
        let item = item.clone();
        Callback::from(move |e: KeyboardEvent| {
            let Some(trigger) = e
                .target()
                .and_then(|target| target.dyn_into::<Element>().ok())
                .and_then(|target| target.closest(".navigation-menu-trigger").ok().flatten())
            else {
                return;
            };
            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    if let Some(item) = item.as_ref() {
                        item.set_open.emit(true);
                    }
                }
                key @ ("ArrowLeft" | "ArrowRight" | "Home" | "End") => {
                    let Some(list) = trigger.closest(".navigation-menu-list").ok().flatten() else {
                        return;
                    };
                    let triggers = query_all(
                        &list,
                        ":scope > .navigation-menu-item > .navigation-menu-trigger, \
                         :scope > .navigation-menu-item > .navigation-menu-link",
                    );
                    let Some(current) = triggers.iter().position(|t| t == &trigger) else {
                        return;
                    };
                    if let Some(next) =
                        roving_index(current, triggers.len(), key, TabsOrientation::Horizontal)
                    {
                        e.prevent_default();
                        focus_element(&triggers[next]);
                    }
                }
                _ => {}
            }
        })
    };

    let (id, controls) = match item.as_ref() {
        Some(item) => (
            Some(item.trigger_id.clone()),
            open.then(|| item.content_id.clone()),
        ),
        None => (None, None),
    };

    html! {
        <button
            type="button"
            class={classes}
            {id}
            onclick={handle_click}
            onmouseenter={handle_mouseenter}
            onmouseleave={onmouseleave}
            onkeydown={onkeydown}
            aria-expanded={open.to_string()}
            aria-controls={controls}
            data-state={if open { "open" } else { "closed" }}
        >
            { children }
            <span class="navigation-menu-trigger-icon" aria-hidden="true">{ "▾" }</span>
        </button>
    }
}

/// Navigation menu content properties
#[derive(Properties, PartialEq, Clone)]
pub struct NavigationMenuContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Navigation menu content component
///
/// Contains the dropdown content for a navigation menu item. Rendered only
/// while its item is open.
#[function_component(NavigationMenuContent)]
pub fn navigation_menu_content(props: &NavigationMenuContentProps) -> Html {
    let NavigationMenuContentProps { class, children } = props.clone();

    let item = use_context::<NavigationMenuItemContext>();
    // Without an item there is no open state to follow; always render.
    if item.as_ref().is_some_and(|item| !item.open) {
        return html! {};
    }

    let classes: Classes = vec![Classes::from("navigation-menu-content"), class]
        .into_iter()
        .collect();

    let (id, labelledby) = match item.as_ref() {
        Some(item) => (Some(item.content_id.clone()), Some(item.trigger_id.clone())),
        None => (None, None),
    };

    html! {
        <div class={classes} {id} aria-labelledby={labelledby} data-state="open">
            { children }
        </div>
    }
}

/// Navigation menu link properties
#[derive(Properties, PartialEq, Clone)]
pub struct NavigationMenuLinkProps {
    /// Link href
    pub href: AttrValue,

    /// Whether this is the active link
    #[prop_or(false)]
    pub active: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Children elements
    pub children: Children,
}

/// Navigation menu link component
///
/// A clickable link in the navigation menu. Clicking a link inside open
/// content closes the menu.
#[function_component(NavigationMenuLink)]
pub fn navigation_menu_link(props: &NavigationMenuLinkProps) -> Html {
    let NavigationMenuLinkProps {
        href,
        active,
        class,
        onclick,
        children,
    } = props.clone();

    let menu = use_context::<NavigationMenuContext>();

    let classes: Classes = vec![
        Classes::from("navigation-menu-link"),
        if active {
            Classes::from("navigation-menu-link-active")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let aria_current = if active {
        Some(AttrValue::from("page"))
    } else {
        None
    };

    let handle_click = Callback::from(move |e: MouseEvent| {
        if let Some(callback) = onclick.as_ref() {
            callback.emit(e);
        }
        if let Some(menu) = menu.as_ref()
            && menu.value.is_some()
        {
            menu.set_value.emit(None);
        }
    });

    html! {
        <a class={classes} href={href} onclick={handle_click} aria-current={aria_current}>
            { children }
        </a>
    }
}

/// Navigation menu indicator properties
#[derive(Properties, PartialEq, Clone)]
pub struct NavigationMenuIndicatorProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Navigation menu indicator component
///
/// Visual indicator for the active navigation item.
#[function_component(NavigationMenuIndicator)]
pub fn navigation_menu_indicator(props: &NavigationMenuIndicatorProps) -> Html {
    let NavigationMenuIndicatorProps { class } = props.clone();

    let classes: Classes = vec![Classes::from("navigation-menu-indicator"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} aria-hidden="true">
            <div class="navigation-menu-indicator-arrow" />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_menu_link_active() {
        let props = NavigationMenuLinkProps {
            href: AttrValue::from("/test"),
            active: true,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert!(props.active);
        assert_eq!(props.href, AttrValue::from("/test"));
    }

    #[test]
    fn test_navigation_menu_link_inactive() {
        let props = NavigationMenuLinkProps {
            href: AttrValue::from("/test"),
            active: false,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert!(!props.active);
    }

    #[test]
    fn test_navigation_menu_orientation_default() {
        let props = NavigationMenuProps {
            value: None,
            default_value: None,
            on_value_change: None,
            orientation: AttrValue::from("horizontal"),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.orientation, AttrValue::from("horizontal"));
    }

    #[test]
    fn test_navigation_menu_orientation_vertical() {
        let props = NavigationMenuProps {
            value: None,
            default_value: None,
            on_value_change: None,
            orientation: AttrValue::from("vertical"),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.orientation, AttrValue::from("vertical"));
    }

    #[test]
    fn test_open_value_treats_empty_as_closed() {
        assert_eq!(open_value(None), None);
        assert_eq!(open_value(Some(AttrValue::from(""))), None);
        assert_eq!(
            open_value(Some(AttrValue::from("docs"))),
            Some(AttrValue::from("docs"))
        );
    }

    #[test]
    fn test_navigation_menu_orientation_class_format() {
        let orientation = AttrValue::from("horizontal");
        let orientation_class = format!("navigation-menu-{}", orientation);
        assert_eq!(orientation_class, "navigation-menu-horizontal");

        let orientation = AttrValue::from("vertical");
        let orientation_class = format!("navigation-menu-{}", orientation);
        assert_eq!(orientation_class, "navigation-menu-vertical");
    }

    #[test]
    fn test_navigation_menu_trigger_hover_props() {
        let props = NavigationMenuTriggerProps {
            class: Classes::new(),
            onclick: None,
            onmouseenter: None,
            onmouseleave: None,
            children: Children::new(vec![]),
        };

        assert!(props.onmouseenter.is_none());
        assert!(props.onmouseleave.is_none());
    }

    #[test]
    fn test_navigation_menu_trigger_with_hover_callbacks() {
        let on_enter = Callback::from(|_: MouseEvent| {});
        let on_leave = Callback::from(|_: MouseEvent| {});

        let props = NavigationMenuTriggerProps {
            class: Classes::new(),
            onclick: None,
            onmouseenter: Some(on_enter),
            onmouseleave: Some(on_leave),
            children: Children::new(vec![]),
        };

        assert!(props.onmouseenter.is_some());
        assert!(props.onmouseleave.is_some());
    }

    #[test]
    fn test_navigation_menu_trigger_with_all_callbacks() {
        let on_click = Callback::from(|_: MouseEvent| {});
        let on_enter = Callback::from(|_: MouseEvent| {});
        let on_leave = Callback::from(|_: MouseEvent| {});

        let props = NavigationMenuTriggerProps {
            class: Classes::new(),
            onclick: Some(on_click),
            onmouseenter: Some(on_enter),
            onmouseleave: Some(on_leave),
            children: Children::new(vec![]),
        };

        assert!(props.onclick.is_some());
        assert!(props.onmouseenter.is_some());
        assert!(props.onmouseleave.is_some());
    }
}
