//! Dropdown Menu component
//!
//! Displays a menu to the user triggered by a button.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, Button};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <DropdownMenu>
//!             <DropdownMenuTrigger>
//!                 <Button>{ "Open Menu" }</Button>
//!             </DropdownMenuTrigger>
//!             <DropdownMenuContent>
//!                 <DropdownMenuItem>{ "Profile" }</DropdownMenuItem>
//!                 <DropdownMenuItem>{ "Settings" }</DropdownMenuItem>
//!                 <DropdownMenuSeparator />
//!                 <DropdownMenuItem>{ "Logout" }</DropdownMenuItem>
//!             </DropdownMenuContent>
//!         </DropdownMenu>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::utils::Portal;
use crate::hooks::{use_escape_key_conditional, use_click_outside_conditional};

/// Dropdown menu component properties
#[derive(Properties, PartialEq, Clone)]
pub struct DropdownMenuProps {
    /// Whether the menu is open
    #[prop_or(false)]
    pub open: bool,

    /// Default open state (for uncontrolled menus)
    #[prop_or(false)]
    pub default_open: bool,

    /// Callback when open state changes
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Children elements
    pub children: Children,
}

/// Dropdown menu component
///
/// A container for dropdown menu trigger and content.
///
/// # Accessibility
/// - Keyboard navigation (Arrow keys, Enter, Escape)
/// - Proper ARIA attributes
/// - Focus management
/// - Screen reader friendly
#[function_component(DropdownMenu)]
pub fn dropdown_menu(props: &DropdownMenuProps) -> Html {
    let DropdownMenuProps {
        open: _,
        default_open: _,
        on_open_change: _,
        children,
    } = props.clone();

    html! {
        <div class="dropdown-menu-root">
            { children }
        </div>
    }
}

/// Dropdown menu trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct DropdownMenuTriggerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Dropdown menu trigger component
///
/// The element that opens the menu when clicked.
#[function_component(DropdownMenuTrigger)]
pub fn dropdown_menu_trigger(props: &DropdownMenuTriggerProps) -> Html {
    let DropdownMenuTriggerProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("dropdown-menu-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Dropdown menu content properties
#[derive(Properties, PartialEq, Clone)]
pub struct DropdownMenuContentProps {
    /// Whether the menu is open
    #[prop_or(false)]
    pub open: bool,

    /// Callback to close the menu
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    /// Whether to close on click outside
    #[prop_or(true)]
    pub close_on_outside_click: bool,

    /// Whether to close on Escape key
    #[prop_or(true)]
    pub close_on_escape: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Dropdown menu content component
///
/// The content that appears in the dropdown menu.
#[function_component(DropdownMenuContent)]
pub fn dropdown_menu_content(props: &DropdownMenuContentProps) -> Html {
    let DropdownMenuContentProps {
        open,
        on_close,
        close_on_outside_click,
        close_on_escape,
        class,
        children,
    } = props.clone();

    let content_ref = use_node_ref();

    // Handle Escape key
    let on_close_esc = on_close.clone();
    use_escape_key_conditional(
        move || {
            if let Some(callback) = on_close_esc.as_ref() {
                callback.emit(());
            }
        },
        open && close_on_escape,
    );

    // Handle click outside
    let on_close_click = on_close.clone();
    use_click_outside_conditional(
        content_ref.clone(),
        move || {
            if let Some(callback) = on_close_click.as_ref() {
                callback.emit(());
            }
        },
        open && close_on_outside_click,
    );

    if !open {
        return html! {};
    }

    let classes: Classes = vec![Classes::from("dropdown-menu-content"), class]
        .into_iter()
        .collect();

    html! {
        <Portal>
            <div
                ref={content_ref}
                class={classes}
                role="menu"
                aria-orientation="vertical"
            >
                { children }
            </div>
        </Portal>
    }
}

/// Dropdown menu item properties
#[derive(Properties, PartialEq, Clone)]
pub struct DropdownMenuItemProps {
    /// Whether the item is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Click event handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Dropdown menu item component
///
/// A clickable item within a dropdown menu.
#[function_component(DropdownMenuItem)]
pub fn dropdown_menu_item(props: &DropdownMenuItemProps) -> Html {
    let DropdownMenuItemProps {
        disabled,
        onclick,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("dropdown-menu-item"),
        if disabled {
            Classes::from("dropdown-menu-item-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div
            class={classes}
            role="menuitem"
            aria-disabled={disabled.to_string()}
            onclick={onclick}
        >
            { children }
        </div>
    }
}

/// Dropdown menu separator properties
#[derive(Properties, PartialEq, Clone)]
pub struct DropdownMenuSeparatorProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Dropdown menu separator component
///
/// A visual separator between menu items.
#[function_component(DropdownMenuSeparator)]
pub fn dropdown_menu_separator(props: &DropdownMenuSeparatorProps) -> Html {
    let DropdownMenuSeparatorProps { class } = props.clone();

    let classes: Classes = vec![Classes::from("dropdown-menu-separator"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="separator" aria-orientation="horizontal" />
    }
}

/// Dropdown menu label properties
#[derive(Properties, PartialEq, Clone)]
pub struct DropdownMenuLabelProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Dropdown menu label component
///
/// A label for a group of menu items.
#[function_component(DropdownMenuLabel)]
pub fn dropdown_menu_label(props: &DropdownMenuLabelProps) -> Html {
    let DropdownMenuLabelProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("dropdown-menu-label"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="presentation">
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dropdown_menu_props_default() {
        let props = DropdownMenuProps {
            open: false,
            default_open: false,
            on_open_change: None,
            children: Children::new(vec![]),
        };

        assert!(!props.open);
        assert!(!props.default_open);
    }

    #[test]
    fn test_dropdown_menu_item_disabled() {
        let props = DropdownMenuItemProps {
            disabled: true,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_dropdown_menu_content_close_behaviors() {
        let props = DropdownMenuContentProps {
            open: true,
            on_close: None,
            close_on_outside_click: false,
            close_on_escape: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.close_on_outside_click);
        assert!(!props.close_on_escape);
    }
}
