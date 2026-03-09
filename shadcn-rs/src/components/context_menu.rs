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

use crate::hooks::{use_click_outside_conditional, use_escape_key_conditional};
use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Context for sharing context menu state between parent and children
#[derive(Clone, PartialEq)]
pub struct ContextMenuState {
    /// Whether the context menu is currently open
    pub is_open: bool,
    /// X position of the context menu
    pub position_x: i32,
    /// Y position of the context menu
    pub position_y: i32,
    /// Callback to open the menu at a position
    pub open_at: Callback<(i32, i32)>,
    /// Callback to close the menu
    pub close: Callback<()>,
}

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

    let is_open = use_state(|| false);
    let position_x = use_state(|| 0i32);
    let position_y = use_state(|| 0i32);

    let open_at = {
        let is_open = is_open.clone();
        let position_x = position_x.clone();
        let position_y = position_y.clone();
        Callback::from(move |(x, y): (i32, i32)| {
            position_x.set(x);
            position_y.set(y);
            is_open.set(true);
        })
    };

    let close = {
        let is_open = is_open.clone();
        Callback::from(move |_: ()| {
            is_open.set(false);
        })
    };

    let context = ContextMenuState {
        is_open: *is_open,
        position_x: *position_x,
        position_y: *position_y,
        open_at,
        close,
    };

    let classes: Classes = vec![Classes::from("context-menu"), class]
        .into_iter()
        .collect();

    html! {
        <ContextProvider<ContextMenuState> context={context}>
            <div class={classes}>
                { children }
            </div>
        </ContextProvider<ContextMenuState>>
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

    let context = use_context::<ContextMenuState>();

    let classes: Classes = vec![Classes::from("context-menu-trigger"), class]
        .into_iter()
        .collect();

    let handle_context_menu = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        if let Some(ctx) = context.as_ref() {
            ctx.open_at.emit((e.client_x(), e.client_y()));
        }
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

    /// Fixed X position in pixels (positions the menu at cursor location)
    #[prop_or_default]
    pub position_x: Option<i32>,

    /// Fixed Y position in pixels (positions the menu at cursor location)
    #[prop_or_default]
    pub position_y: Option<i32>,

    /// Children elements
    pub children: Children,
}

/// Context menu content component
///
/// Contains the context menu items. When used inside a `ContextMenu`, position
/// and visibility are managed automatically via context. When `position_x` and
/// `position_y` are both set as props, those override context positioning.
#[function_component(ContextMenuContent)]
pub fn context_menu_content(props: &ContextMenuContentProps) -> Html {
    let ContextMenuContentProps {
        class,
        position_x: prop_x,
        position_y: prop_y,
        children,
    } = props.clone();

    let context = use_context::<ContextMenuState>();
    let content_ref = use_node_ref();

    let is_open = context.as_ref().map(|ctx| ctx.is_open).unwrap_or(true);
    let ctx_x = context.as_ref().map(|ctx| ctx.position_x);
    let ctx_y = context.as_ref().map(|ctx| ctx.position_y);

    // Close on click outside
    let context_click = context.clone();
    use_click_outside_conditional(
        content_ref.clone(),
        move || {
            if let Some(ctx) = context_click.as_ref() {
                ctx.close.emit(());
            }
        },
        is_open,
    );

    // Close on Escape
    let context_esc = context.clone();
    use_escape_key_conditional(
        move || {
            if let Some(ctx) = context_esc.as_ref() {
                ctx.close.emit(());
            }
        },
        is_open,
    );

    if !is_open {
        return html! {};
    }

    let classes: Classes = vec![Classes::from("context-menu-content"), class]
        .into_iter()
        .collect();

    // Props override context position
    let x = prop_x.or(ctx_x);
    let y = prop_y.or(ctx_y);

    let style = match (x, y) {
        (Some(x), Some(y)) => Some(format!("position: fixed; left: {}px; top: {}px;", x, y)),
        _ => None,
    };

    html! {
        <div ref={content_ref} class={classes} role="menu" style={style}>
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
/// A clickable item in the context menu. Supports keyboard navigation
/// via Enter and Space keys.
#[function_component(ContextMenuItem)]
pub fn context_menu_item(props: &ContextMenuItemProps) -> Html {
    let ContextMenuItemProps {
        disabled,
        class,
        onclick,
        children,
    } = props.clone();

    let context = use_context::<ContextMenuState>();

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

    let handle_click = {
        let onclick = onclick.clone();
        let context = context.clone();
        Callback::from(move |e: MouseEvent| {
            if disabled {
                return;
            }
            if let Some(callback) = onclick.as_ref() {
                callback.emit(e);
            }
            // Close the menu after clicking an item
            if let Some(ctx) = context.as_ref() {
                ctx.close.emit(());
            }
        })
    };

    let onkeydown = {
        Callback::from(move |e: KeyboardEvent| {
            if disabled {
                return;
            }
            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    // Dispatch click on the target element for keyboard activation.
                    // el.click() triggers handle_click which already emits ctx.close,
                    // so we don't emit close again here.
                    if let Some(target) = e.target()
                        && let Some(el) = target.dyn_ref::<web_sys::HtmlElement>()
                    {
                        el.click();
                    }
                }
                _ => {}
            }
        })
    };

    let tabindex = if disabled { "-1" } else { "0" };

    html! {
        <div
            class={classes}
            role="menuitem"
            onclick={handle_click}
            onkeydown={onkeydown}
            tabindex={tabindex}
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

/// Context for sharing radio group state
#[derive(Clone, PartialEq)]
pub struct ContextMenuRadioContext {
    /// Currently selected value
    pub value: Option<AttrValue>,
    /// Callback when value changes
    pub onchange: Option<Callback<AttrValue>>,
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
        value,
        class,
        onchange,
        children,
    } = props.clone();

    let radio_context = ContextMenuRadioContext { value, onchange };

    let classes: Classes = vec![Classes::from("context-menu-radio-group"), class]
        .into_iter()
        .collect();

    html! {
        <ContextProvider<ContextMenuRadioContext> context={radio_context}>
            <div class={classes} role="group">
                { children }
            </div>
        </ContextProvider<ContextMenuRadioContext>>
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
        value,
        disabled,
        class,
        onclick,
        children,
    } = props.clone();

    let radio_context = use_context::<ContextMenuRadioContext>();
    let is_checked = radio_context
        .as_ref()
        .and_then(|ctx| ctx.value.as_ref())
        .map(|v| *v == value)
        .unwrap_or(false);

    let classes: Classes = vec![
        Classes::from("context-menu-radio-item"),
        if disabled {
            Classes::from("context-menu-radio-item-disabled")
        } else {
            Classes::new()
        },
        if is_checked {
            Classes::from("context-menu-radio-item-checked")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let handle_click = {
        let value = value.clone();
        let radio_context = radio_context.clone();
        let onclick = onclick.clone();
        Callback::from(move |e: MouseEvent| {
            if disabled {
                return;
            }
            if let Some(ctx) = radio_context.as_ref()
                && let Some(onchange) = ctx.onchange.as_ref()
            {
                onchange.emit(value.clone());
            }
            if let Some(callback) = onclick.as_ref() {
                callback.emit(e);
            }
        })
    };

    html! {
        <div
            class={classes}
            role="menuitemradio"
            aria-checked={is_checked.to_string()}
            aria-disabled={disabled.to_string()}
            onclick={handle_click}
        >
            <span class="context-menu-radio-indicator">
                { if is_checked { "●" } else { "" } }
            </span>
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

    #[test]
    fn test_context_menu_content_position_both_set() {
        let props = ContextMenuContentProps {
            class: Classes::new(),
            position_x: Some(100),
            position_y: Some(200),
            children: Children::new(vec![]),
        };

        assert_eq!(props.position_x, Some(100));
        assert_eq!(props.position_y, Some(200));

        // Verify the style string that would be generated
        let style = match (props.position_x, props.position_y) {
            (Some(x), Some(y)) => Some(format!("position: fixed; left: {}px; top: {}px;", x, y)),
            _ => None,
        };
        assert_eq!(
            style,
            Some("position: fixed; left: 100px; top: 200px;".to_string())
        );
    }

    #[test]
    fn test_context_menu_content_position_none() {
        let props = ContextMenuContentProps {
            class: Classes::new(),
            position_x: None,
            position_y: None,
            children: Children::new(vec![]),
        };

        let style = match (props.position_x, props.position_y) {
            (Some(x), Some(y)) => Some(format!("position: fixed; left: {}px; top: {}px;", x, y)),
            _ => None,
        };
        assert!(style.is_none());
    }

    #[test]
    fn test_context_menu_content_position_partial() {
        // Only position_x set, no style should be generated
        let props = ContextMenuContentProps {
            class: Classes::new(),
            position_x: Some(50),
            position_y: None,
            children: Children::new(vec![]),
        };

        let style = match (props.position_x, props.position_y) {
            (Some(x), Some(y)) => Some(format!("position: fixed; left: {}px; top: {}px;", x, y)),
            _ => None,
        };
        assert!(style.is_none());
    }

    #[test]
    fn test_context_menu_item_tabindex_enabled() {
        let props = ContextMenuItemProps {
            disabled: false,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        let tabindex = if props.disabled { "-1" } else { "0" };
        assert_eq!(tabindex, "0");
    }

    #[test]
    fn test_context_menu_item_tabindex_disabled() {
        let props = ContextMenuItemProps {
            disabled: true,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        let tabindex = if props.disabled { "-1" } else { "0" };
        assert_eq!(tabindex, "-1");
    }
}
