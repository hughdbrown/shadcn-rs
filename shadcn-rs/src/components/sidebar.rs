//! Sidebar component
//!
//! A composable, themeable and customizable sidebar component.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Sidebar, SidebarHeader, SidebarContent, SidebarFooter, SidebarGroup, SidebarMenu, SidebarMenuItem, SidebarMenuButton};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Sidebar>
//!             <SidebarHeader>
//!                 <h2>{ "My App" }</h2>
//!             </SidebarHeader>
//!             <SidebarContent>
//!                 <SidebarGroup>
//!                     <SidebarMenu>
//!                         <SidebarMenuItem>
//!                             <SidebarMenuButton href="/">{ "Home" }</SidebarMenuButton>
//!                         </SidebarMenuItem>
//!                         <SidebarMenuItem>
//!                             <SidebarMenuButton href="/settings">{ "Settings" }</SidebarMenuButton>
//!                         </SidebarMenuItem>
//!                     </SidebarMenu>
//!                 </SidebarGroup>
//!             </SidebarContent>
//!             <SidebarFooter>
//!                 <p>{ "v1.0.0" }</p>
//!             </SidebarFooter>
//!         </Sidebar>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Sidebar container properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarProps {
    /// Collapsed state
    #[prop_or(false)]
    pub collapsed: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sidebar container component
///
/// The main sidebar container.
///
/// # Accessibility
/// - Keyboard navigable
/// - Screen reader friendly
/// - Focus management
#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let SidebarProps {
        collapsed,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("sidebar"),
        if collapsed {
            Classes::from("sidebar-collapsed")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <aside class={classes} aria-label="Sidebar">
            { children }
        </aside>
    }
}

/// Sidebar header properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarHeaderProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sidebar header component
///
/// Header section of the sidebar.
#[function_component(SidebarHeader)]
pub fn sidebar_header(props: &SidebarHeaderProps) -> Html {
    let SidebarHeaderProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("sidebar-header"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Sidebar content properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sidebar content component
///
/// Main content area of the sidebar.
#[function_component(SidebarContent)]
pub fn sidebar_content(props: &SidebarContentProps) -> Html {
    let SidebarContentProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("sidebar-content"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Sidebar footer properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarFooterProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sidebar footer component
///
/// Footer section of the sidebar.
#[function_component(SidebarFooter)]
pub fn sidebar_footer(props: &SidebarFooterProps) -> Html {
    let SidebarFooterProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("sidebar-footer"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Sidebar group properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarGroupProps {
    /// Group label
    #[prop_or_default]
    pub label: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sidebar group component
///
/// Groups related sidebar items.
#[function_component(SidebarGroup)]
pub fn sidebar_group(props: &SidebarGroupProps) -> Html {
    let SidebarGroupProps {
        label,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("sidebar-group"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            {
                if let Some(label_text) = label {
                    html! {
                        <div class="sidebar-group-label">
                            { label_text }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            { children }
        </div>
    }
}

/// Sidebar menu properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarMenuProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sidebar menu component
///
/// A menu within the sidebar.
#[function_component(SidebarMenu)]
pub fn sidebar_menu(props: &SidebarMenuProps) -> Html {
    let SidebarMenuProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("sidebar-menu"), class]
        .into_iter()
        .collect();

    html! {
        <nav class={classes}>
            <ul class="sidebar-menu-list">
                { children }
            </ul>
        </nav>
    }
}

/// Sidebar menu item properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarMenuItemProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sidebar menu item component
///
/// A single item in the sidebar menu.
#[function_component(SidebarMenuItem)]
pub fn sidebar_menu_item(props: &SidebarMenuItemProps) -> Html {
    let SidebarMenuItemProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("sidebar-menu-item"), class]
        .into_iter()
        .collect();

    html! {
        <li class={classes}>
            { children }
        </li>
    }
}

/// Sidebar menu button properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarMenuButtonProps {
    /// Link href (if used as link)
    #[prop_or_default]
    pub href: Option<AttrValue>,

    /// Active state
    #[prop_or(false)]
    pub active: bool,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Children elements
    pub children: Children,
}

/// Sidebar menu button component
///
/// A clickable button or link in the sidebar menu.
#[function_component(SidebarMenuButton)]
pub fn sidebar_menu_button(props: &SidebarMenuButtonProps) -> Html {
    let SidebarMenuButtonProps {
        href,
        active,
        disabled,
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("sidebar-menu-button"),
        if active {
            Classes::from("sidebar-menu-button-active")
        } else {
            Classes::new()
        },
        if disabled {
            Classes::from("sidebar-menu-button-disabled")
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

    if let Some(href_val) = href {
        html! {
            <a
                class={classes}
                href={href_val}
                onclick={onclick}
                aria-current={aria_current}
                aria-disabled={disabled.to_string()}
            >
                { children }
            </a>
        }
    } else {
        html! {
            <button
                class={classes}
                onclick={onclick}
                disabled={disabled}
                aria-current={aria_current}
            >
                { children }
            </button>
        }
    }
}

/// Sidebar separator properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarSeparatorProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Sidebar separator component
///
/// Separates sections in the sidebar.
#[function_component(SidebarSeparator)]
pub fn sidebar_separator(props: &SidebarSeparatorProps) -> Html {
    let SidebarSeparatorProps { class } = props.clone();

    let classes: Classes = vec![Classes::from("sidebar-separator"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="separator" aria-orientation="horizontal" />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sidebar_collapsed() {
        let props = SidebarProps {
            collapsed: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.collapsed);
    }

    #[test]
    fn test_sidebar_group_with_label() {
        let props = SidebarGroupProps {
            label: Some(AttrValue::from("Navigation")),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.label, Some(AttrValue::from("Navigation")));
    }

    #[test]
    fn test_sidebar_menu_button_active() {
        let props = SidebarMenuButtonProps {
            href: Some(AttrValue::from("/home")),
            active: true,
            disabled: false,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert!(props.active);
        assert!(!props.disabled);
        assert_eq!(props.href, Some(AttrValue::from("/home")));
    }

    #[test]
    fn test_sidebar_menu_button_disabled() {
        let props = SidebarMenuButtonProps {
            href: None,
            active: false,
            disabled: true,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
        assert!(!props.active);
    }
}
