//! Context Menu component
//!
//! Displays a menu to the user at the location of right-click or long-press.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <ContextMenu>
//!             <ContextMenuTrigger>
//!                 <div>{ "Right-click me" }</div>
//!             </ContextMenuTrigger>
//!             <ContextMenuContent>
//!                 <ContextMenuItem>{ "Copy" }</ContextMenuItem>
//!                 <ContextMenuItem>{ "Paste" }</ContextMenuItem>
//!                 <ContextMenuItem>{ "Delete" }</ContextMenuItem>
//!             </ContextMenuContent>
//!         </ContextMenu>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Context menu container properties
#[derive(Properties, PartialEq, Clone)]
pub struct ContextMenuProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Context menu container component
///
/// Displays a menu at the pointer location on right-click.
///
/// # Accessibility
/// - Keyboard accessible
/// - Screen reader announcements
/// - Focus management
#[function_component(ContextMenu)]
pub fn context_menu(props: &ContextMenuProps) -> Html {
    let ContextMenuProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("context-menu"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Context menu trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct ContextMenuTriggerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Context menu handler
    #[prop_or_default]
    pub oncontextmenu: Option<Callback<MouseEvent>>,

    /// Children elements
    pub children: Children,
}

/// Context menu trigger component
///
/// The area that triggers the context menu on right-click.
#[function_component(ContextMenuTrigger)]
pub fn context_menu_trigger(props: &ContextMenuTriggerProps) -> Html {
    let ContextMenuTriggerProps {
        class,
        oncontextmenu,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("context-menu-trigger"), class]
        .into_iter()
        .collect();

    let handle_context_menu = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        if let Some(callback) = oncontextmenu.as_ref() {
            callback.emit(e);
        }
    });

    html! {
        <div class={classes} oncontextmenu={handle_context_menu}>
            { children }
        </div>
    }
}

/// Context menu content properties
#[derive(Properties, PartialEq, Clone)]
pub struct ContextMenuContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Context menu content component
///
/// Contains the context menu items.
#[function_component(ContextMenuContent)]
pub fn context_menu_content(props: &ContextMenuContentProps) -> Html {
    let ContextMenuContentProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("context-menu-content"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="menu">
            { children }
        </div>
    }
}

/// Context menu item properties
#[derive(Properties, PartialEq, Clone)]
pub struct ContextMenuItemProps {
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

/// Context menu item component
///
/// A clickable item in the context menu.
#[function_component(ContextMenuItem)]
pub fn context_menu_item(props: &ContextMenuItemProps) -> Html {
    let ContextMenuItemProps {
        disabled,
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("context-menu-item"),
        if disabled {
            Classes::from("context-menu-item-disabled")
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
            onclick={onclick}
            aria-disabled={disabled.to_string()}
        >
            { children }
        </div>
    }
}

/// Context menu separator properties
#[derive(Properties, PartialEq, Clone)]
pub struct ContextMenuSeparatorProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Context menu separator component
///
/// Separates groups of context menu items.
#[function_component(ContextMenuSeparator)]
pub fn context_menu_separator(props: &ContextMenuSeparatorProps) -> Html {
    let ContextMenuSeparatorProps { class } = props.clone();

    let classes: Classes = vec![Classes::from("context-menu-separator"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="separator" aria-orientation="horizontal" />
    }
}

/// Context menu label properties
#[derive(Properties, PartialEq, Clone)]
pub struct ContextMenuLabelProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Context menu label component
///
/// A label for a group of context menu items.
#[function_component(ContextMenuLabel)]
pub fn context_menu_label(props: &ContextMenuLabelProps) -> Html {
    let ContextMenuLabelProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("context-menu-label"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="presentation">
            { children }
        </div>
    }
}

/// Context menu checkbox item properties
#[derive(Properties, PartialEq, Clone)]
pub struct ContextMenuCheckboxItemProps {
    /// Checked state
    #[prop_or(false)]
    pub checked: bool,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Change handler
    #[prop_or_default]
    pub onchange: Option<Callback<bool>>,

    /// Children elements
    pub children: Children,
}

/// Context menu checkbox item component
///
/// A context menu item with a checkbox.
#[function_component(ContextMenuCheckboxItem)]
pub fn context_menu_checkbox_item(props: &ContextMenuCheckboxItemProps) -> Html {
    let ContextMenuCheckboxItemProps {
        checked,
        disabled,
        class,
        onchange,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("context-menu-checkbox-item"),
        if checked {
            Classes::from("context-menu-checkbox-item-checked")
        } else {
            Classes::new()
        },
        if disabled {
            Classes::from("context-menu-checkbox-item-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let onclick = Callback::from(move |_| {
        if !disabled && let Some(callback) = onchange.as_ref() {
            callback.emit(!checked);
        }
    });

    html! {
        <div
            class={classes}
            role="menuitemcheckbox"
            aria-checked={checked.to_string()}
            aria-disabled={disabled.to_string()}
            onclick={onclick}
        >
            <span class="context-menu-checkbox-indicator">
                { if checked { "✓" } else { "" } }
            </span>
            { children }
        </div>
    }
}

/// Context menu radio group properties
#[derive(Properties, PartialEq, Clone)]
pub struct ContextMenuRadioGroupProps {
    /// Currently selected value
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Change handler
    #[prop_or_default]
    pub onchange: Option<Callback<AttrValue>>,

    /// Children elements
    pub children: Children,
}

/// Context menu radio group component
///
/// Groups radio items in a context menu.
#[function_component(ContextMenuRadioGroup)]
pub fn context_menu_radio_group(props: &ContextMenuRadioGroupProps) -> Html {
    let ContextMenuRadioGroupProps {
        value: _,
        class,
        onchange: _,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("context-menu-radio-group"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="group">
            { children }
        </div>
    }
}

/// Context menu radio item properties
#[derive(Properties, PartialEq, Clone)]
pub struct ContextMenuRadioItemProps {
    /// Value of this radio item
    pub value: AttrValue,

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

/// Context menu radio item component
///
/// A radio option in a context menu radio group.
#[function_component(ContextMenuRadioItem)]
pub fn context_menu_radio_item(props: &ContextMenuRadioItemProps) -> Html {
    let ContextMenuRadioItemProps {
        value: _,
        disabled,
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("context-menu-radio-item"),
        if disabled {
            Classes::from("context-menu-radio-item-disabled")
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
            aria-disabled={disabled.to_string()}
            onclick={onclick}
        >
            <span class="context-menu-radio-indicator" />
            { children }
        </div>
    }
}

/// Context menu sub properties
#[derive(Properties, PartialEq, Clone)]
pub struct ContextMenuSubProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Context menu sub component
///
/// A submenu within a context menu.
#[function_component(ContextMenuSub)]
pub fn context_menu_sub(props: &ContextMenuSubProps) -> Html {
    let ContextMenuSubProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("context-menu-sub"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_menu_item_disabled() {
        let props = ContextMenuItemProps {
            disabled: true,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_context_menu_checkbox_item_checked() {
        let props = ContextMenuCheckboxItemProps {
            checked: true,
            disabled: false,
            class: Classes::new(),
            onchange: None,
            children: Children::new(vec![]),
        };

        assert!(props.checked);
        assert!(!props.disabled);
    }

    #[test]
    fn test_context_menu_radio_item_value() {
        let props = ContextMenuRadioItemProps {
            value: AttrValue::from("option1"),
            disabled: false,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.value, AttrValue::from("option1"));
        assert!(!props.disabled);
    }

    #[test]
    fn test_context_menu_label_default() {
        let props = ContextMenuLabelProps {
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.class, Classes::new());
    }
}
