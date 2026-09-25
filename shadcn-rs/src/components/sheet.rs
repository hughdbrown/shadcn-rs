//! Sheet component
//!
//! Extends the Dialog component to display content that complements the main content of the screen.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Sheet, SheetTrigger, SheetContent, SheetHeader, SheetTitle, SheetDescription, SheetFooter, Button, Position};
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
//!         <Sheet open={*open} {on_open_change} side={Position::Right}>
//!             <SheetTrigger>
//!                 <Button>{ "Open Sheet" }</Button>
//!             </SheetTrigger>
//!             <SheetContent>
//!                 <SheetHeader>
//!                     <SheetTitle>{ "Edit Profile" }</SheetTitle>
//!                     <SheetDescription>
//!                         { "Make changes to your profile here. Click save when you're done." }
//!                     </SheetDescription>
//!                 </SheetHeader>
//!                 <div class="py-4">
//!                     { "Sheet content goes here..." }
//!                 </div>
//!                 <SheetFooter>
//!                     <Button>{ "Save changes" }</Button>
//!                 </SheetFooter>
//!             </SheetContent>
//!         </Sheet>
//!     }
//! }
//! ```

use crate::hooks::{
    use_click_outside_conditional, use_controllable_bool, use_escape_key_conditional,
    use_focus_trap,
};
use crate::types::Position;
use crate::utils::Portal;
use yew::prelude::*;

/// Context for sharing sheet state between parent and children
#[derive(Clone, PartialEq)]
pub struct SheetContext {
    /// Whether the sheet is currently open
    pub is_open: bool,
    /// Callback to set open state
    pub set_open: Callback<bool>,
    /// Callback to toggle open state
    pub toggle: Callback<()>,
    /// Side from which the sheet slides in
    pub side: Position,
}

/// Sheet component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SheetProps {
    /// Whether the sheet is open
    ///
    /// `Some(_)` makes the component controlled: the value is always honored and
    /// user interaction only reports the requested state through `on_open_change`.
    /// `None` (the default) leaves it uncontrolled, starting from `default_open`.
    #[prop_or_default]
    pub open: Option<bool>,

    /// Default open state (for uncontrolled sheets)
    #[prop_or(false)]
    pub default_open: bool,

    /// Callback when open state changes
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Side from which the sheet slides in
    #[prop_or(Position::Right)]
    pub side: Position,

    /// Children elements
    pub children: Children,
}

/// Sheet component
///
/// A container for sheet trigger and content.
///
/// # Accessibility
/// - Traps focus within the sheet
/// - Closes on Escape key
/// - Closes on overlay click (unless disabled)
/// - Proper ARIA attributes (role="dialog")
/// - Keyboard navigation support
#[function_component(Sheet)]
pub fn sheet(props: &SheetProps) -> Html {
    let SheetProps {
        open,
        default_open,
        on_open_change,
        side,
        children,
    } = props.clone();

    let (is_open, set_open) = use_controllable_bool(open, default_open, on_open_change);

    // Toggle from the effective state so controlled and uncontrolled modes agree.
    let toggle = {
        let set_open = set_open.clone();
        Callback::from(move |_: ()| set_open.emit(!is_open))
    };

    let context = SheetContext {
        is_open,
        set_open,
        toggle,
        side,
    };

    html! {
        <ContextProvider<SheetContext> context={context}>
            <div class="sheet-root">
                { children }
            </div>
        </ContextProvider<SheetContext>>
    }
}

/// Sheet trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct SheetTriggerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sheet trigger component
///
/// The element that opens the sheet when clicked.
#[function_component(SheetTrigger)]
pub fn sheet_trigger(props: &SheetTriggerProps) -> Html {
    let SheetTriggerProps { class, children } = props.clone();

    let context = use_context::<SheetContext>();

    let handle_click = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ctx) = context.as_ref() {
                ctx.toggle.emit(());
            }
        })
    };

    let classes: Classes = vec![Classes::from("sheet-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} onclick={handle_click}>
            { children }
        </div>
    }
}

/// Sheet content properties
#[derive(Properties, PartialEq, Clone)]
pub struct SheetContentProps {
    /// Whether the sheet is open
    #[prop_or(false)]
    pub open: bool,

    /// Callback to close the sheet
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    /// Side from which the sheet slides in
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

/// Sheet content component
///
/// The actual sheet content that slides in from the edge.
#[function_component(SheetContent)]
pub fn sheet_content(props: &SheetContentProps) -> Html {
    let SheetContentProps {
        open: prop_open,
        on_close,
        side: prop_side,
        close_on_overlay_click,
        close_on_escape,
        class,
        children,
    } = props.clone();

    let context = use_context::<SheetContext>();
    let content_ref = use_node_ref();

    // Use context open state if available, otherwise use prop
    let is_open = context.as_ref().map(|ctx| ctx.is_open).unwrap_or(prop_open);
    let side = context.as_ref().map(|ctx| ctx.side).unwrap_or(prop_side);

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

    let onkeydown = use_focus_trap(content_ref.clone(), is_open);

    if !is_open {
        return html! {};
    }

    let classes: Classes = vec![
        Classes::from("sheet-content"),
        Classes::from(side.to_class()),
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <Portal>
            <div class="sheet-overlay">
                <div
                    ref={content_ref}
                    class={classes}
                    role="dialog"
                    aria-modal="true"
                    tabindex="-1"
                    onkeydown={onkeydown}
                >
                    { children }
                </div>
            </div>
        </Portal>
    }
}

/// Sheet close properties
#[derive(Properties, PartialEq, Clone)]
pub struct SheetCloseProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sheet close component
///
/// Wraps a button or element to close the sheet when clicked.
#[function_component(SheetClose)]
pub fn sheet_close(props: &SheetCloseProps) -> Html {
    let SheetCloseProps { class, children } = props.clone();

    let context = use_context::<SheetContext>();

    let handle_click = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ctx) = context.as_ref() {
                ctx.set_open.emit(false);
            }
        })
    };

    let classes: Classes = vec![Classes::from("sheet-close"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} onclick={handle_click}>
            { children }
        </div>
    }
}

/// Sheet header properties
#[derive(Properties, PartialEq, Clone)]
pub struct SheetHeaderProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sheet header component
///
/// Container for sheet title and description.
#[function_component(SheetHeader)]
pub fn sheet_header(props: &SheetHeaderProps) -> Html {
    let SheetHeaderProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("sheet-header"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Sheet title properties
#[derive(Properties, PartialEq, Clone)]
pub struct SheetTitleProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sheet title component
///
/// The main heading of the sheet.
#[function_component(SheetTitle)]
pub fn sheet_title(props: &SheetTitleProps) -> Html {
    let SheetTitleProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("sheet-title"), class]
        .into_iter()
        .collect();

    html! {
        <h2 class={classes}>
            { children }
        </h2>
    }
}

/// Sheet description properties
#[derive(Properties, PartialEq, Clone)]
pub struct SheetDescriptionProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sheet description component
///
/// Descriptive text for the sheet.
#[function_component(SheetDescription)]
pub fn sheet_description(props: &SheetDescriptionProps) -> Html {
    let SheetDescriptionProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("sheet-description"), class]
        .into_iter()
        .collect();

    html! {
        <p class={classes}>
            { children }
        </p>
    }
}

/// Sheet footer properties
#[derive(Properties, PartialEq, Clone)]
pub struct SheetFooterProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Sheet footer component
///
/// Container for sheet actions (typically buttons).
#[function_component(SheetFooter)]
pub fn sheet_footer(props: &SheetFooterProps) -> Html {
    let SheetFooterProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("sheet-footer"), class]
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
    fn test_sheet_props_default() {
        let props = SheetProps {
            open: None,
            default_open: false,
            on_open_change: None,
            side: Position::Right,
            children: Children::new(vec![]),
        };

        assert_eq!(props.open, None);
        assert!(!props.default_open);
        assert_eq!(props.side, Position::Right);
    }

    #[test]
    fn test_sheet_props_different_sides() {
        let sides = vec![
            Position::Top,
            Position::Right,
            Position::Bottom,
            Position::Left,
        ];

        for side in sides {
            let props = SheetProps {
                open: Some(true),
                default_open: false,
                on_open_change: None,
                side: side.clone(),
                children: Children::new(vec![]),
            };

            assert_eq!(props.side, side);
        }
    }

    #[test]
    fn test_sheet_content_close_behaviors() {
        let props = SheetContentProps {
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

    #[test]
    fn test_sheet_content_disabled_close() {
        let props = SheetContentProps {
            open: true,
            on_close: None,
            side: Position::Left,
            close_on_overlay_click: false,
            close_on_escape: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.close_on_overlay_click);
        assert!(!props.close_on_escape);
        assert_eq!(props.side, Position::Left);
    }

    #[test]
    fn test_sheet_content_all_sides() {
        let sides = vec![
            Position::Top,
            Position::Right,
            Position::Bottom,
            Position::Left,
        ];

        for side in sides {
            let props = SheetContentProps {
                open: true,
                on_close: None,
                side: side.clone(),
                close_on_overlay_click: true,
                close_on_escape: true,
                class: Classes::new(),
                children: Children::new(vec![]),
            };

            assert_eq!(props.side, side);
        }
    }
}
