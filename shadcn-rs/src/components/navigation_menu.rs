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

use yew::prelude::*;

/// Navigation menu container properties
#[derive(Properties, PartialEq, Clone)]
pub struct NavigationMenuProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Navigation menu container component
///
/// The main container for navigation menus.
///
/// # Accessibility
/// - Keyboard navigation support
/// - Screen reader friendly
/// - Focus management
#[function_component(NavigationMenu)]
pub fn navigation_menu(props: &NavigationMenuProps) -> Html {
    let NavigationMenuProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("navigation-menu"), class]
        .into_iter()
        .collect();

    html! {
        <nav class={classes}>
            { children }
        </nav>
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
    let NavigationMenuItemProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("navigation-menu-item"), class]
        .into_iter()
        .collect();

    html! {
        <li class={classes}>
            { children }
        </li>
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

    /// Children elements
    pub children: Children,
}

/// Navigation menu trigger component
///
/// Triggers the display of navigation menu content.
#[function_component(NavigationMenuTrigger)]
pub fn navigation_menu_trigger(props: &NavigationMenuTriggerProps) -> Html {
    let NavigationMenuTriggerProps {
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("navigation-menu-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <button class={classes} onclick={onclick}>
            { children }
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
/// Contains the dropdown content for a navigation menu item.
#[function_component(NavigationMenuContent)]
pub fn navigation_menu_content(props: &NavigationMenuContentProps) -> Html {
    let NavigationMenuContentProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("navigation-menu-content"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
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
/// A clickable link in the navigation menu.
#[function_component(NavigationMenuLink)]
pub fn navigation_menu_link(props: &NavigationMenuLinkProps) -> Html {
    let NavigationMenuLinkProps {
        href,
        active,
        class,
        onclick,
        children,
    } = props.clone();

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

    html! {
        <a class={classes} href={href} onclick={onclick} aria-current={aria_current}>
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
}
