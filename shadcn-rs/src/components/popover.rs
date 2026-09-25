//! Popover component
//!
//! Displays rich content next to a trigger. Clicking the trigger toggles it;
//! clicking outside or pressing Escape closes it.
//!
//! # Examples
//!
//! Uncontrolled:
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
//!
//! Controlled:
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Popover, PopoverTrigger, PopoverContent, Button};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let open = use_state(|| false);
//!     let on_open_change = {
//!         let open = open.clone();
//!         Callback::from(move |value: bool| open.set(value))
//!     };
//!
//!     html! {
//!         <Popover open={*open} {on_open_change}>
//!             <PopoverTrigger>
//!                 <Button>{ "Toggle" }</Button>
//!             </PopoverTrigger>
//!             <PopoverContent>{ "Content" }</PopoverContent>
//!         </Popover>
//!     }
//! }
//! ```

use crate::hooks::{
    use_click_outside_conditional, use_controllable_bool, use_escape_key_conditional,
};
use crate::types::Position;
use crate::utils::{Portal, active_element, collect_focusable, focus_element, focus_first_within};
use web_sys::Element;
use yew::prelude::*;

/// Context shared by a [`Popover`] root with its trigger and content.
#[derive(Clone, PartialEq)]
pub struct PopoverContext {
    /// Whether the popover is currently open
    pub is_open: bool,
    /// Requests a new open state (reported through `on_open_change`)
    pub set_open: Callback<bool>,
    /// Flips the effective open state
    pub toggle: Callback<()>,
    /// The root element wrapping trigger and content (used for outside clicks)
    pub root_ref: NodeRef,
    /// The trigger element (focus returns here on close)
    pub trigger_ref: NodeRef,
}

/// Popover component properties
#[derive(Properties, PartialEq, Clone)]
pub struct PopoverProps {
    /// Whether the popover is open
    ///
    /// `Some(_)` makes the popover controlled: the value is always honored and
    /// user interaction only reports the requested state through `on_open_change`.
    /// `None` (the default) leaves it uncontrolled, starting from `default_open`.
    #[prop_or_default]
    pub open: Option<bool>,

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
/// Owns the open state and provides it to [`PopoverTrigger`] and
/// [`PopoverContent`]. Content is rendered inside the root wrapper (which is
/// `position: relative`) so it is placed next to the trigger.
///
/// # Accessibility
/// - Trigger exposes `aria-expanded` and `aria-haspopup="dialog"`
/// - Moves focus into the content on open
/// - Closes on Escape and on click outside
/// - Returns focus to the trigger on close
#[function_component(Popover)]
pub fn popover(props: &PopoverProps) -> Html {
    let PopoverProps {
        open,
        default_open,
        on_open_change,
        children,
    } = props.clone();

    let root_ref = use_node_ref();
    let trigger_ref = use_node_ref();
    let (is_open, set_open) = use_controllable_bool(open, default_open, on_open_change);

    let toggle = {
        let set_open = set_open.clone();
        Callback::from(move |_: ()| set_open.emit(!is_open))
    };

    let context = PopoverContext {
        is_open,
        set_open,
        toggle,
        root_ref: root_ref.clone(),
        trigger_ref,
    };

    html! {
        <ContextProvider<PopoverContext> {context}>
            <div
                ref={root_ref}
                class="popover-root popover"
                data-state={if is_open { "open" } else { "closed" }}
            >
                { children }
            </div>
        </ContextProvider<PopoverContext>>
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
/// Toggles the popover when clicked (or activated from the keyboard).
#[function_component(PopoverTrigger)]
pub fn popover_trigger(props: &PopoverTriggerProps) -> Html {
    let PopoverTriggerProps { class, children } = props.clone();

    let context = use_context::<PopoverContext>();
    let is_open = context.as_ref().is_some_and(|ctx| ctx.is_open);
    let trigger_ref = context
        .as_ref()
        .map(|ctx| ctx.trigger_ref.clone())
        .unwrap_or_default();

    let onclick = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ctx) = context.as_ref() {
                ctx.toggle.emit(());
            }
        })
    };

    let classes: Classes = vec![Classes::from("popover-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <div
            ref={trigger_ref}
            class={classes}
            aria-haspopup="dialog"
            aria-expanded={is_open.to_string()}
            data-state={if is_open { "open" } else { "closed" }}
            {onclick}
        >
            { children }
        </div>
    }
}

/// Popover content properties
#[derive(Properties, PartialEq, Clone)]
pub struct PopoverContentProps {
    /// Whether the popover is open.
    ///
    /// Only used when the content is rendered outside a [`Popover`] root;
    /// inside one, the root's state wins.
    #[prop_or(false)]
    pub open: bool,

    /// Callback to close the popover (standalone use only; inside a
    /// [`Popover`] root closing goes through `on_open_change`)
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    /// Popover position relative to trigger
    #[prop_or(Position::Bottom)]
    pub position: Position,

    /// Alignment along the trigger edge: `"start"`, `"center"` (default) or `"end"`
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
/// The content that appears in the popover. Inside a [`Popover`] root it is
/// rendered next to the trigger; used standalone (with `open`/`on_close`) it
/// is rendered through a [`Portal`].
#[function_component(PopoverContent)]
pub fn popover_content(props: &PopoverContentProps) -> Html {
    let PopoverContentProps {
        open: prop_open,
        on_close,
        position,
        align,
        close_on_outside_click,
        close_on_escape,
        class,
        children,
    } = props.clone();

    let context = use_context::<PopoverContext>();
    let content_ref = use_node_ref();
    let is_open = context.as_ref().map_or(prop_open, |ctx| ctx.is_open);

    let close = {
        let context = context.clone();
        let on_close = on_close.clone();
        move || {
            if let Some(ctx) = context.as_ref() {
                ctx.set_open.emit(false);
            } else if let Some(callback) = on_close.as_ref() {
                callback.emit(());
            }
        }
    };

    // Escape closes; focus then returns to the trigger via the effect below.
    use_escape_key_conditional(close.clone(), is_open && close_on_escape);

    // Inside a root, "outside" means outside trigger + content, so a click on
    // the trigger toggles instead of closing and immediately reopening.
    let outside_ref = context
        .as_ref()
        .map_or_else(|| content_ref.clone(), |ctx| ctx.root_ref.clone());
    use_click_outside_conditional(outside_ref, close, is_open && close_on_outside_click);

    // Move focus into the content on open and restore it on close.
    {
        let content_ref = content_ref.clone();
        let trigger_ref = context.as_ref().map(|ctx| ctx.trigger_ref.clone());
        use_effect_with(is_open, move |&is_open| {
            let restore_to = if is_open {
                let previous = active_element();
                if let Some(element) = content_ref.cast::<Element>() {
                    focus_first_within(&element);
                }
                trigger_ref
                    .and_then(|trigger| trigger.cast::<Element>())
                    .map(|trigger| {
                        collect_focusable(&trigger)
                            .into_iter()
                            .next()
                            .unwrap_or(trigger)
                    })
                    .or(previous)
            } else {
                None
            };

            move || {
                // Only reclaim focus if it was lost with the content (it now
                // sits on <body>); never steal it from something the user
                // clicked outside the popover.
                let focus_lost = active_element().is_none_or(|active| {
                    active.tag_name().eq_ignore_ascii_case("body")
                        || content_ref
                            .cast::<Element>()
                            .is_some_and(|content| content.contains(Some(&active)))
                });
                if focus_lost && let Some(element) = restore_to.as_ref() {
                    focus_element(element);
                }
            }
        });
    }

    if !is_open {
        return html! {};
    }

    let align = match align.as_deref() {
        Some("start") => "start",
        Some("end") => "end",
        _ => "center",
    };

    let classes: Classes = vec![
        Classes::from("popover-content"),
        Classes::from(position.to_class()),
        class,
    ]
    .into_iter()
    .collect();

    let content = html! {
        <div
            ref={content_ref}
            class={classes}
            role="dialog"
            aria-modal="false"
            tabindex="-1"
            data-state="open"
            data-align={align}
        >
            { children }
        </div>
    };

    if context.is_some() {
        content
    } else {
        html! { <Portal>{ content }</Portal> }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popover_props_default() {
        let props = PopoverProps {
            open: None,
            default_open: false,
            on_open_change: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.open, None);
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
