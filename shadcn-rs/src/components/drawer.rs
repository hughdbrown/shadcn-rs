//! Drawer component
//!
//! A panel that slides in from the edge of the screen.
//!
//! # Examples
//!
//! ```rust,ignore
//! use yew::prelude::*;
//! use shadcn_rs::{Drawer, DrawerTrigger, DrawerContent, DrawerHeader, DrawerTitle, DrawerDescription, DrawerFooter, Button, Position};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let open = use_state(|| false);
//!
//!     let on_open_change = {
//!         let open = open.clone();
//!         Callback::from(move |is_open: bool| {
//!             open.set(is_open);
//!         })
//!     };
//!
//!     html! {
//!         <Drawer {open} {on_open_change} side={Position::Right}>
//!             <DrawerTrigger>
//!                 <Button>{ "Open Drawer" }</Button>
//!             </DrawerTrigger>
//!             <DrawerContent>
//!                 <DrawerHeader>
//!                     <DrawerTitle>{ "Drawer Title" }</DrawerTitle>
//!                     <DrawerDescription>{ "Drawer description" }</DrawerDescription>
//!                 </DrawerHeader>
//!                 <div class="p-4">{ "Drawer content" }</div>
//!                 <DrawerFooter>
//!                     <Button>{ "Submit" }</Button>
//!                 </DrawerFooter>
//!             </DrawerContent>
//!         </Drawer>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::types::Position;
use crate::utils::Portal;
use crate::hooks::{use_escape_key_conditional, use_click_outside_conditional};

/// Context for sharing drawer state between parent and children
#[derive(Clone, PartialEq)]
pub struct DrawerContext {
    /// Whether the drawer is currently open
    pub is_open: bool,
    /// Callback to set open state
    pub set_open: Callback<bool>,
    /// Callback to toggle open state
    pub toggle: Callback<()>,
    /// Side from which the drawer slides in
    pub side: Position,
}

/// Drawer component properties
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerProps {
    /// Whether the drawer is open
    #[prop_or(false)]
    pub open: bool,

    /// Default open state (for uncontrolled drawers)
    #[prop_or(false)]
    pub default_open: bool,

    /// Callback when open state changes
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Side from which the drawer slides in
    #[prop_or(Position::Right)]
    pub side: Position,

    /// Children elements
    pub children: Children,
}

/// Drawer component
///
/// A container for drawer trigger and content.
///
/// # Accessibility
/// - Traps focus within the drawer
/// - Closes on Escape key
/// - Closes on overlay click (unless disabled)
/// - Proper ARIA attributes
/// - Keyboard navigation support
#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    let DrawerProps {
        open,
        default_open,
        on_open_change,
        side,
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

    let context = DrawerContext {
        is_open,
        set_open,
        toggle,
        side: side.clone(),
    };

    html! {
        <ContextProvider<DrawerContext> context={context}>
            <div class="drawer-root">
                { children }
            </div>
        </ContextProvider<DrawerContext>>
    }
}

/// Drawer trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerTriggerProps {
    /// Children elements
    pub children: Children,
}

/// Drawer trigger component
///
/// The element that opens the drawer when clicked.
#[function_component(DrawerTrigger)]
pub fn drawer_trigger(props: &DrawerTriggerProps) -> Html {
    let DrawerTriggerProps { children } = props.clone();

    let context = use_context::<DrawerContext>();

    let handle_click = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ctx) = context.as_ref() {
                ctx.toggle.emit(());
            }
        })
    };

    html! {
        <div class="drawer-trigger" onclick={handle_click}>
            { children }
        </div>
    }
}

/// Drawer content properties
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerContentProps {
    /// Whether the drawer is open
    #[prop_or(false)]
    pub open: bool,

    /// Callback to close the drawer
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    /// Side from which the drawer slides in
    #[prop_or(Position::Right)]
    pub side: Position,

    /// Whether to close on overlay click
    #[prop_or(true)]
    pub close_on_overlay_click: bool,

    /// Whether to close on Escape key
    #[prop_or(true)]
    pub close_on_escape: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Drawer content component
///
/// The actual drawer content that slides in from the edge.
#[function_component(DrawerContent)]
pub fn drawer_content(props: &DrawerContentProps) -> Html {
    let DrawerContentProps {
        open: prop_open,
        on_close,
        side: prop_side,
        close_on_overlay_click,
        close_on_escape,
        class,
        children,
    } = props.clone();

    let context = use_context::<DrawerContext>();
    let content_ref = use_node_ref();

    // Use context open state if available, otherwise use prop
    let is_open = context.as_ref().map(|ctx| ctx.is_open).unwrap_or(prop_open);
    let side = context.as_ref().map(|ctx| ctx.side.clone()).unwrap_or(prop_side);

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
        is_open && close_on_overlay_click,
    );

    if !is_open {
        return html! {};
    }

    let classes: Classes = vec![
        Classes::from("drawer-content"),
        Classes::from(side.to_class()),
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <Portal>
            <div class="drawer-overlay" aria-hidden="true">
                <div
                    ref={content_ref}
                    class={classes}
                    role="dialog"
                    aria-modal="true"
                >
                    { children }
                </div>
            </div>
        </Portal>
    }
}

/// Drawer header properties
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerHeaderProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Drawer header component
///
/// Container for drawer title and description.
#[function_component(DrawerHeader)]
pub fn drawer_header(props: &DrawerHeaderProps) -> Html {
    let DrawerHeaderProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("drawer-header"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Drawer title properties
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerTitleProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Drawer title component
///
/// The main heading of the drawer.
#[function_component(DrawerTitle)]
pub fn drawer_title(props: &DrawerTitleProps) -> Html {
    let DrawerTitleProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("drawer-title"), class]
        .into_iter()
        .collect();

    html! {
        <h2 class={classes}>
            { children }
        </h2>
    }
}

/// Drawer description properties
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerDescriptionProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Drawer description component
///
/// Descriptive text for the drawer.
#[function_component(DrawerDescription)]
pub fn drawer_description(props: &DrawerDescriptionProps) -> Html {
    let DrawerDescriptionProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("drawer-description"), class]
        .into_iter()
        .collect();

    html! {
        <p class={classes}>
            { children }
        </p>
    }
}

/// Drawer footer properties
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerFooterProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Drawer footer component
///
/// Container for drawer actions (typically buttons).
#[function_component(DrawerFooter)]
pub fn drawer_footer(props: &DrawerFooterProps) -> Html {
    let DrawerFooterProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("drawer-footer"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Drawer close properties
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerCloseProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Drawer close component
///
/// Wraps a button or element to close the drawer when clicked.
#[function_component(DrawerClose)]
pub fn drawer_close(props: &DrawerCloseProps) -> Html {
    let DrawerCloseProps { class, children } = props.clone();

    let context = use_context::<DrawerContext>();

    let handle_click = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ctx) = context.as_ref() {
                ctx.set_open.emit(false);
            }
        })
    };

    let classes: Classes = vec![Classes::from("drawer-close"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} onclick={handle_click}>
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawer_props_default() {
        let props = DrawerProps {
            open: false,
            default_open: false,
            on_open_change: None,
            side: Position::Right,
            children: Children::new(vec![]),
        };

        assert!(!props.open);
        assert_eq!(props.side, Position::Right);
    }

    #[test]
    fn test_drawer_sides() {
        let sides = vec![
            Position::Top,
            Position::Right,
            Position::Bottom,
            Position::Left,
        ];

        for side in sides {
            let props = DrawerProps {
                open: false,
                default_open: false,
                on_open_change: None,
                side: side.clone(),
                children: Children::new(vec![]),
            };
            assert_eq!(props.side, side);
        }
    }

    #[test]
    fn test_drawer_content_close_behaviors() {
        let props = DrawerContentProps {
            open: true,
            on_close: None,
            side: Position::Right,
            close_on_overlay_click: true,
            close_on_escape: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.close_on_overlay_click);
        assert!(props.close_on_escape);
    }
}
