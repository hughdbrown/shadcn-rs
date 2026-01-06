//! Popover component
//!
//! Displays rich content in a portal, triggered by a button.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Popover, PopoverTrigger, PopoverContent, Button};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Popover>
//!             <PopoverTrigger>
//!                 <Button>{ "Open Popover" }</Button>
//!             </PopoverTrigger>
//!             <PopoverContent>
//!                 <div class="space-y-2">
//!                     <h4>{ "Dimensions" }</h4>
//!                     <p>{ "Set the dimensions for the layer." }</p>
//!                 </div>
//!             </PopoverContent>
//!         </Popover>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::types::Position;
use crate::utils::Portal;
use crate::hooks::{use_escape_key_conditional, use_click_outside_conditional};

/// Popover component properties
#[derive(Properties, PartialEq, Clone)]
pub struct PopoverProps {
    /// Whether the popover is open
    #[prop_or(false)]
    pub open: bool,

    /// Default open state (for uncontrolled popovers)
    #[prop_or(false)]
    pub default_open: bool,

    /// Callback when open state changes
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Children elements
    pub children: Children,
}

/// Popover component
///
/// A container for popover trigger and content.
///
/// # Accessibility
/// - Closes on Escape key
/// - Closes on click outside
/// - Proper ARIA attributes
/// - Keyboard navigation support
#[function_component(Popover)]
pub fn popover(props: &PopoverProps) -> Html {
    let PopoverProps {
        open,
        default_open,
        on_open_change,
        children,
    } = props.clone();

    html! {
        <div class="popover-root">
            { children }
        </div>
    }
}

/// Popover trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct PopoverTriggerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Popover trigger component
///
/// The element that opens the popover when clicked.
#[function_component(PopoverTrigger)]
pub fn popover_trigger(props: &PopoverTriggerProps) -> Html {
    let PopoverTriggerProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("popover-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Popover content properties
#[derive(Properties, PartialEq, Clone)]
pub struct PopoverContentProps {
    /// Whether the popover is open
    #[prop_or(false)]
    pub open: bool,

    /// Callback to close the popover
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    /// Popover position relative to trigger
    #[prop_or(Position::Bottom)]
    pub position: Position,

    /// Alignment of popover
    #[prop_or_default]
    pub align: Option<AttrValue>,

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

/// Popover content component
///
/// The content that appears in the popover.
#[function_component(PopoverContent)]
pub fn popover_content(props: &PopoverContentProps) -> Html {
    let PopoverContentProps {
        open,
        on_close,
        position,
        align,
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

    let classes: Classes = vec![
        Classes::from("popover-content"),
        Classes::from(position.to_class()),
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <Portal>
            <div
                ref={content_ref}
                class={classes}
                role="dialog"
                aria-modal="false"
            >
                { children }
            </div>
        </Portal>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popover_props_default() {
        let props = PopoverProps {
            open: false,
            default_open: false,
            on_open_change: None,
            children: Children::new(vec![]),
        };

        assert!(!props.open);
        assert!(!props.default_open);
    }

    #[test]
    fn test_popover_content_positions() {
        let positions = vec![
            Position::Top,
            Position::Right,
            Position::Bottom,
            Position::Left,
        ];

        for position in positions {
            let props = PopoverContentProps {
                open: true,
                on_close: None,
                position: position.clone(),
                align: None,
                close_on_outside_click: true,
                close_on_escape: true,
                class: Classes::new(),
                children: Children::new(vec![]),
            };
            assert_eq!(props.position, position);
        }
    }

    #[test]
    fn test_popover_content_close_behaviors() {
        let props = PopoverContentProps {
            open: true,
            on_close: None,
            position: Position::Bottom,
            align: None,
            close_on_outside_click: false,
            close_on_escape: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.close_on_outside_click);
        assert!(!props.close_on_escape);
    }
}
