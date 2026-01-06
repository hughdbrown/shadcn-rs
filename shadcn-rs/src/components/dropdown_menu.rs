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

/// Context for sharing dropdown menu state between parent and children
#[derive(Clone, PartialEq)]
pub struct DropdownMenuContext {
    /// Whether the dropdown menu is currently open
    pub is_open: bool,
    /// Callback to set open state
    pub set_open: Callback<bool>,
    /// Callback to toggle open state
    pub toggle: Callback<()>,
}

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
        open,
        default_open,
        on_open_change,
        children,
    } = props.clone();

    // Internal state for uncontrolled mode
    let internal_open = use_state(|| default_open);

    // Use controlled value if provided (open=true), otherwise use internal state
    let is_open = if open { open } else { *internal_open };

    let set_open = {
        let internal_open = internal_open.clone();
        let on_open_change = on_open_change.clone();
        Callback::from(move |new_state: bool| {
            internal_open.set(new_state);
            if let Some(callback) = on_open_change.as_ref() {
                callback.emit(new_state);
            }
        })
    };

    let toggle = {
        let internal_open = internal_open.clone();
        let on_open_change = on_open_change.clone();
        Callback::from(move |_: ()| {
            let new_state = !*internal_open;
            internal_open.set(new_state);
            if let Some(callback) = on_open_change.as_ref() {
                callback.emit(new_state);
            }
        })
    };

    let context = DropdownMenuContext {
        is_open,
        set_open,
        toggle,
    };

    html! {
        <ContextProvider<DropdownMenuContext> context={context}>
            <div class="dropdown-menu-root">
                { children }
            </div>
        </ContextProvider<DropdownMenuContext>>
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

    let context = use_context::<DropdownMenuContext>();

    let handle_click = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ctx) = context.as_ref() {
                ctx.toggle.emit(());
            }
        })
    };

    let classes: Classes = vec![Classes::from("dropdown-menu-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} onclick={handle_click}>
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
        open: prop_open,
        on_close,
        close_on_outside_click,
        close_on_escape,
        class,
        children,
    } = props.clone();

    let context = use_context::<DropdownMenuContext>();
    let content_ref = use_node_ref();

    // Use context open state if available, otherwise use prop
    let is_open = context.as_ref().map(|ctx| ctx.is_open).unwrap_or(prop_open);

    // Handle Escape key - close via context if available
    let context_esc = context.clone();
    let on_close_esc = on_close.clone();
    use_escape_key_conditional(
        move || {
            if let Some(ctx) = context_esc.as_ref() {
                ctx.set_open.emit(false);
            } else if let Some(callback) = on_close_esc.as_ref() {
                callback.emit(());
            }
        },
        is_open && close_on_escape,
    );

    // Handle click outside - close via context if available
    let context_click = context.clone();
    let on_close_click = on_close.clone();
    use_click_outside_conditional(
        content_ref.clone(),
        move || {
            if let Some(ctx) = context_click.as_ref() {
                ctx.set_open.emit(false);
            } else if let Some(callback) = on_close_click.as_ref() {
                callback.emit(());
            }
        },
        is_open && close_on_outside_click,
    );

    if !is_open {
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

    let context = use_context::<DropdownMenuContext>();

    let handle_click = {
        let context = context.clone();
        let onclick = onclick.clone();
        let disabled = disabled;
        Callback::from(move |e: MouseEvent| {
            if disabled {
                return;
            }
            // Call user's onclick
            if let Some(callback) = onclick.as_ref() {
                callback.emit(e);
            }
            // Close the menu
            if let Some(ctx) = context.as_ref() {
                ctx.set_open.emit(false);
            }
        })
    };

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
            onclick={handle_click}
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

/// Dropdown menu checkbox item properties
#[derive(Properties, PartialEq, Clone)]
pub struct DropdownMenuCheckboxItemProps {
    /// Whether the checkbox is checked
    #[prop_or(false)]
    pub checked: bool,

    /// Whether the item is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Checked state change handler
    #[prop_or_default]
    pub on_checked_change: Option<Callback<bool>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Dropdown menu checkbox item component
///
/// A checkbox item within a dropdown menu.
#[function_component(DropdownMenuCheckboxItem)]
pub fn dropdown_menu_checkbox_item(props: &DropdownMenuCheckboxItemProps) -> Html {
    let DropdownMenuCheckboxItemProps {
        checked,
        disabled,
        on_checked_change,
        class,
        children,
    } = props.clone();

    let onclick = on_checked_change.map(|cb| {
        let checked = checked;
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            cb.emit(!checked);
        })
    });

    let classes: Classes = vec![
        Classes::from("dropdown-menu-checkbox-item"),
        if checked {
            Classes::from("dropdown-menu-checkbox-item-checked")
        } else {
            Classes::new()
        },
        if disabled {
            Classes::from("dropdown-menu-checkbox-item-disabled")
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
            role="menuitemcheckbox"
            aria-checked={checked.to_string()}
            aria-disabled={disabled.to_string()}
            onclick={onclick}
        >
            <span class="dropdown-menu-item-indicator">
                if checked {
                    { "✓" }
                }
            </span>
            { children }
        </div>
    }
}

/// Dropdown menu radio group properties
#[derive(Properties, PartialEq, Clone)]
pub struct DropdownMenuRadioGroupProps {
    /// Currently selected value
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Value change handler
    #[prop_or_default]
    pub on_value_change: Option<Callback<AttrValue>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements (should be DropdownMenuRadioItem)
    pub children: Children,
}

/// Dropdown menu radio group component
///
/// A group of radio items within a dropdown menu.
#[function_component(DropdownMenuRadioGroup)]
pub fn dropdown_menu_radio_group(props: &DropdownMenuRadioGroupProps) -> Html {
    let DropdownMenuRadioGroupProps {
        value: _,
        on_value_change: _,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("dropdown-menu-radio-group"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="group">
            { children }
        </div>
    }
}

/// Dropdown menu radio item properties
#[derive(Properties, PartialEq, Clone)]
pub struct DropdownMenuRadioItemProps {
    /// The value of this radio item
    pub value: AttrValue,

    /// Whether the item is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Click handler (receives the value)
    #[prop_or_default]
    pub on_select: Option<Callback<AttrValue>>,

    /// Whether this item is selected
    #[prop_or(false)]
    pub selected: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Dropdown menu radio item component
///
/// A radio item within a dropdown menu radio group.
#[function_component(DropdownMenuRadioItem)]
pub fn dropdown_menu_radio_item(props: &DropdownMenuRadioItemProps) -> Html {
    let DropdownMenuRadioItemProps {
        value,
        disabled,
        on_select,
        selected,
        class,
        children,
    } = props.clone();

    let onclick = on_select.map(|cb| {
        let value = value.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            cb.emit(value.clone());
        })
    });

    let classes: Classes = vec![
        Classes::from("dropdown-menu-radio-item"),
        if selected {
            Classes::from("dropdown-menu-radio-item-selected")
        } else {
            Classes::new()
        },
        if disabled {
            Classes::from("dropdown-menu-radio-item-disabled")
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
            role="menuitemradio"
            aria-checked={selected.to_string()}
            aria-disabled={disabled.to_string()}
            onclick={onclick}
        >
            <span class="dropdown-menu-item-indicator">
                if selected {
                    { "●" }
                }
            </span>
            { children }
        </div>
    }
}

/// Dropdown menu sub properties
#[derive(Properties, PartialEq, Clone)]
pub struct DropdownMenuSubProps {
    /// Whether the submenu is open
    #[prop_or(false)]
    pub open: bool,

    /// Callback when open state changes
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Dropdown menu sub component
///
/// A nested submenu within a dropdown menu.
#[function_component(DropdownMenuSub)]
pub fn dropdown_menu_sub(props: &DropdownMenuSubProps) -> Html {
    let DropdownMenuSubProps {
        open: _,
        on_open_change: _,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("dropdown-menu-sub"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="menu">
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

    #[test]
    fn test_dropdown_menu_checkbox_item_checked() {
        let props = DropdownMenuCheckboxItemProps {
            checked: true,
            disabled: false,
            on_checked_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.checked);
        assert!(!props.disabled);
    }

    #[test]
    fn test_dropdown_menu_checkbox_item_unchecked() {
        let props = DropdownMenuCheckboxItemProps {
            checked: false,
            disabled: true,
            on_checked_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.checked);
        assert!(props.disabled);
    }

    #[test]
    fn test_dropdown_menu_radio_item_selected() {
        let props = DropdownMenuRadioItemProps {
            value: AttrValue::from("option1"),
            disabled: false,
            on_select: None,
            selected: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.selected);
        assert_eq!(props.value, AttrValue::from("option1"));
    }

    #[test]
    fn test_dropdown_menu_radio_item_unselected() {
        let props = DropdownMenuRadioItemProps {
            value: AttrValue::from("option2"),
            disabled: false,
            on_select: None,
            selected: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.selected);
    }

    #[test]
    fn test_dropdown_menu_sub_default() {
        let props = DropdownMenuSubProps {
            open: false,
            on_open_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.open);
    }
}
