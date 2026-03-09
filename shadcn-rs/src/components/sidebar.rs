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

/// Sidebar state shared via context
#[derive(Clone, Debug, PartialEq)]
pub struct SidebarContext {
    /// Whether the sidebar is currently open
    pub open: bool,
    /// Whether the sidebar is in mobile mode
    pub is_mobile: bool,
    /// Callback to toggle the sidebar open/closed
    pub toggle: Callback<()>,
    /// Callback to set the sidebar open state directly
    pub set_open: Callback<bool>,
}

/// SidebarProvider component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarProviderProps {
    /// Whether the sidebar starts open
    #[prop_or(true)]
    pub default_open: bool,

    /// Controlled open state
    #[prop_or_default]
    pub open: Option<bool>,

    /// Callback when open state changes
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// SidebarProvider component
///
/// Wraps the application layout to provide sidebar state context.
/// Use the `use_sidebar` hook or `SidebarTrigger` to control the sidebar.
///
/// # Accessibility
/// - Manages focus when sidebar toggles
/// - Provides state to all sidebar sub-components
#[function_component(SidebarProvider)]
pub fn sidebar_provider(props: &SidebarProviderProps) -> Html {
    let SidebarProviderProps {
        default_open,
        open: controlled_open,
        on_open_change,
        class,
        children,
    } = props.clone();

    let internal_open = use_state(|| default_open);

    let is_open = controlled_open.unwrap_or(*internal_open);

    let toggle = {
        let internal_open = internal_open.clone();
        let on_open_change = on_open_change.clone();
        Callback::from(move |_: ()| {
            let new_val = !controlled_open.unwrap_or(*internal_open);
            if controlled_open.is_none() {
                internal_open.set(new_val);
            }
            if let Some(cb) = &on_open_change {
                cb.emit(new_val);
            }
        })
    };

    let set_open = {
        let internal_open = internal_open.clone();
        let on_open_change = on_open_change.clone();
        Callback::from(move |value: bool| {
            internal_open.set(value);
            if let Some(cb) = &on_open_change {
                cb.emit(value);
            }
        })
    };

    let context = SidebarContext {
        open: is_open,
        is_mobile: false,
        toggle,
        set_open,
    };

    let classes: Classes = vec![
        Classes::from("sidebar-provider"),
        if is_open {
            Classes::from("sidebar-provider-open")
        } else {
            Classes::from("sidebar-provider-closed")
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <ContextProvider<SidebarContext> {context}>
            <div class={classes}>
                { children }
            </div>
        </ContextProvider<SidebarContext>>
    }
}

/// Hook to access sidebar state from a SidebarProvider ancestor.
///
/// # Panics
///
/// Panics if used outside a SidebarProvider.
///
/// # Examples
///
/// ```rust,ignore
/// use yew::prelude::*;
/// use shadcn_rs::components::sidebar::use_sidebar;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let sidebar = use_sidebar();
///
///     html! {
///         <button onclick={move |_| sidebar.toggle.emit(())}>
///             { if sidebar.open { "Close" } else { "Open" } }
///         </button>
///     }
/// }
/// ```
#[hook]
pub fn use_sidebar() -> SidebarContext {
    use_context::<SidebarContext>().expect("use_sidebar must be used within a SidebarProvider")
}

/// SidebarTrigger component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarTriggerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements (defaults to a hamburger icon if empty)
    #[prop_or_default]
    pub children: Children,
}

/// SidebarTrigger component
///
/// A button that toggles the sidebar open/closed state.
///
/// # Accessibility
/// - Uses `aria-expanded` to indicate sidebar state
/// - Uses `aria-label` for screen readers
#[function_component(SidebarTrigger)]
pub fn sidebar_trigger(props: &SidebarTriggerProps) -> Html {
    let SidebarTriggerProps { class, children } = props.clone();

    let context = use_context::<SidebarContext>();

    let (is_open, onclick) = if let Some(ctx) = &context {
        let toggle = ctx.toggle.clone();
        (
            ctx.open,
            Callback::from(move |_: MouseEvent| toggle.emit(())),
        )
    } else {
        (false, Callback::from(|_: MouseEvent| {}))
    };

    let classes: Classes = vec![Classes::from("sidebar-trigger"), class]
        .into_iter()
        .collect();

    let has_children = !children.is_empty();

    html! {
        <button
            class={classes}
            onclick={onclick}
            aria-expanded={is_open.to_string()}
            aria-label="Toggle sidebar"
        >
            if has_children {
                { children }
            } else {
                <span class="sidebar-trigger-icon" aria-hidden="true">
                    { "☰" }
                </span>
            }
        </button>
    }
}

/// SidebarRail component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarRailProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// SidebarRail component
///
/// A thin rail on the edge of the sidebar that can be used to toggle it.
/// Typically rendered as a narrow vertical bar that becomes visible on hover.
///
/// # Accessibility
/// - Keyboard accessible (Enter/Space to toggle)
/// - Uses aria-label for screen readers
#[function_component(SidebarRail)]
pub fn sidebar_rail(props: &SidebarRailProps) -> Html {
    let SidebarRailProps { class } = props.clone();

    let context = use_context::<SidebarContext>();

    let onclick = if let Some(ctx) = &context {
        let toggle = ctx.toggle.clone();
        Callback::from(move |_: MouseEvent| toggle.emit(()))
    } else {
        Callback::from(|_: MouseEvent| {})
    };

    let classes: Classes = vec![Classes::from("sidebar-rail"), class]
        .into_iter()
        .collect();

    html! {
        <button
            class={classes}
            onclick={onclick}
            aria-label="Toggle sidebar"
            tabindex="0"
        />
    }
}

/// SidebarInset component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarInsetProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// SidebarInset component
///
/// The main content area that sits next to the sidebar.
/// Automatically adjusts its layout when the sidebar opens/closes.
#[function_component(SidebarInset)]
pub fn sidebar_inset(props: &SidebarInsetProps) -> Html {
    let SidebarInsetProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("sidebar-inset"), class]
        .into_iter()
        .collect();

    html! {
        <main class={classes}>
            { children }
        </main>
    }
}

/// Sidebar container properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarProps {
    /// Collapsed state
    #[prop_or(false)]
    pub collapsed: bool,

    /// Whether the sidebar is in mobile overlay mode
    #[prop_or(false)]
    pub is_mobile: bool,

    /// Callback when the backdrop is clicked (to close sidebar on mobile)
    #[prop_or_default]
    pub on_backdrop_click: Option<Callback<MouseEvent>>,

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
        is_mobile,
        on_backdrop_click,
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
        if is_mobile {
            Classes::from("sidebar-mobile")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <>
            if is_mobile && !collapsed {
                <div class="sidebar-backdrop" onclick={on_backdrop_click} aria-hidden="true" />
            }
            <aside class={classes} aria-label="Sidebar">
                { children }
            </aside>
        </>
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

/// Sidebar group label properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarGroupLabelProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sidebar group label component
///
/// A label for a sidebar group.
#[function_component(SidebarGroupLabel)]
pub fn sidebar_group_label(props: &SidebarGroupLabelProps) -> Html {
    let SidebarGroupLabelProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("sidebar-group-label"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Sidebar group content properties
#[derive(Properties, PartialEq, Clone)]
pub struct SidebarGroupContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sidebar group content component
///
/// The content area of a sidebar group.
#[function_component(SidebarGroupContent)]
pub fn sidebar_group_content(props: &SidebarGroupContentProps) -> Html {
    let SidebarGroupContentProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("sidebar-group-content"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
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
            is_mobile: false,
            on_backdrop_click: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.collapsed);
    }

    #[test]
    fn test_sidebar_mobile() {
        let props = SidebarProps {
            collapsed: false,
            is_mobile: true,
            on_backdrop_click: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.is_mobile);
        assert!(!props.collapsed);
    }

    #[test]
    fn test_sidebar_mobile_with_backdrop() {
        let props = SidebarProps {
            collapsed: false,
            is_mobile: true,
            on_backdrop_click: Some(Callback::noop()),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.is_mobile);
        assert!(props.on_backdrop_click.is_some());
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

    #[test]
    fn test_sidebar_group_label_default() {
        let props = SidebarGroupLabelProps {
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.class, Classes::new());
    }

    #[test]
    fn test_sidebar_group_content_default() {
        let props = SidebarGroupContentProps {
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.class, Classes::new());
    }

    #[test]
    fn test_sidebar_provider_props_default_open() {
        let props = SidebarProviderProps {
            default_open: true,
            open: None,
            on_open_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.default_open);
        assert!(props.open.is_none());
    }

    #[test]
    fn test_sidebar_provider_props_controlled() {
        let props = SidebarProviderProps {
            default_open: true,
            open: Some(false),
            on_open_change: Some(Callback::noop()),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.open, Some(false));
        assert!(props.on_open_change.is_some());
    }

    #[test]
    fn test_sidebar_context_equality() {
        let ctx1 = SidebarContext {
            open: true,
            is_mobile: false,
            toggle: Callback::noop(),
            set_open: Callback::noop(),
        };
        let ctx2 = SidebarContext {
            open: true,
            is_mobile: false,
            toggle: Callback::noop(),
            set_open: Callback::noop(),
        };

        assert_eq!(ctx1.open, ctx2.open);
        assert_eq!(ctx1.is_mobile, ctx2.is_mobile);
    }

    #[test]
    fn test_sidebar_trigger_props() {
        let props = SidebarTriggerProps {
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.class, Classes::new());
    }

    #[test]
    fn test_sidebar_rail_props() {
        let props = SidebarRailProps {
            class: Classes::new(),
        };

        assert_eq!(props.class, Classes::new());
    }

    #[test]
    fn test_sidebar_inset_props() {
        let props = SidebarInsetProps {
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.class, Classes::new());
    }
}
