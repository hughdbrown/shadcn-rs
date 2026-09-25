//! Hover Card component
//!
//! For sighted users to preview content available behind a link.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{HoverCard, HoverCardTrigger, HoverCardContent};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <HoverCard>
//!             <HoverCardTrigger>
//!                 <a href="/user/alice">{ "@alice" }</a>
//!             </HoverCardTrigger>
//!             <HoverCardContent>
//!                 <div class="space-y-1">
//!                     <h4 class="text-sm font-semibold">{ "Alice Smith" }</h4>
//!                     <p class="text-sm">{ "Software engineer at ACME Corp" }</p>
//!                     <div class="flex items-center pt-2">
//!                         <span class="text-xs text-muted">{ "Joined December 2023" }</span>
//!                     </div>
//!                 </div>
//!             </HoverCardContent>
//!         </HoverCard>
//!     }
//! }
//! ```

use crate::hooks::{use_controllable_bool, use_escape_key_conditional};
use crate::types::Position;
use crate::utils::Portal;
use gloo::timers::callback::Timeout;
use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Context for sharing hover card state between parent and children
#[derive(Clone, PartialEq)]
pub struct HoverCardContext {
    /// Whether the hover card is currently open
    pub is_open: bool,
    /// Requests a new open state immediately (cancels any pending delay)
    pub set_open: Callback<bool>,
    /// Pointer or focus arrived (trigger or card): open after `open_delay`,
    /// or cancel a pending close if already open
    pub schedule_open: Callback<()>,
    /// Pointer or focus left (trigger or card): close after `close_delay`,
    /// or cancel a pending open if still closed
    pub schedule_close: Callback<()>,
}

/// Hover Card component properties
#[derive(Properties, PartialEq, Clone)]
pub struct HoverCardProps {
    /// Whether the hover card is open
    ///
    /// `Some(_)` makes the card controlled: the value is always honored and
    /// hover/focus only report the requested state through `on_open_change`.
    /// `None` (the default) leaves it uncontrolled, starting from `default_open`.
    #[prop_or_default]
    pub open: Option<bool>,

    /// Default open state (for uncontrolled hover cards)
    #[prop_or(false)]
    pub default_open: bool,

    /// Callback when open state changes
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Open delay in milliseconds
    #[prop_or(200)]
    pub open_delay: u32,

    /// Close delay in milliseconds. Moving the pointer from the trigger into
    /// the card within this time keeps it open.
    #[prop_or(300)]
    pub close_delay: u32,

    /// Children elements
    pub children: Children,
}

/// Hover Card component
///
/// A container for hover card trigger and content.
///
/// # Accessibility
/// - Closes on Escape key
/// - Proper ARIA attributes
/// - Keyboard accessible (opens on focus)
/// - Not announced to screen readers (visual only)
#[function_component(HoverCard)]
pub fn hover_card(props: &HoverCardProps) -> Html {
    let HoverCardProps {
        open,
        default_open,
        on_open_change,
        open_delay,
        close_delay,
        children,
    } = props.clone();

    let (is_open, request_open) = use_controllable_bool(open, default_open, on_open_change);
    // One pending open-or-close at a time; dropping the handle cancels it.
    let pending = use_mut_ref(|| None::<Timeout>);

    let set_open = {
        let request_open = request_open.clone();
        let pending = pending.clone();
        Callback::from(move |value: bool| {
            pending.borrow_mut().take();
            if value != is_open {
                request_open.emit(value);
            }
        })
    };

    let schedule = {
        let pending = pending.clone();
        move |target: bool, delay: u32| {
            let request_open = request_open.clone();
            let pending = pending.clone();
            Callback::from(move |_: ()| {
                pending.borrow_mut().take();
                if is_open == target {
                    // Already there: cancelling the opposite transition is enough.
                    return;
                }
                if delay == 0 {
                    request_open.emit(target);
                    return;
                }
                let request_open = request_open.clone();
                *pending.borrow_mut() =
                    Some(Timeout::new(delay, move || request_open.emit(target)));
            })
        }
    };

    let context = HoverCardContext {
        is_open,
        set_open,
        schedule_open: schedule(true, open_delay),
        schedule_close: schedule(false, close_delay),
    };

    html! {
        <ContextProvider<HoverCardContext> context={context}>
            <div
                class="hover-card-root hover-card"
                data-state={if is_open { "open" } else { "closed" }}
            >
                { children }
            </div>
        </ContextProvider<HoverCardContext>>
    }
}

/// Hover Card trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct HoverCardTriggerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Hover Card trigger component
///
/// The element that triggers the hover card on mouse enter and focus.
#[function_component(HoverCardTrigger)]
pub fn hover_card_trigger(props: &HoverCardTriggerProps) -> Html {
    let HoverCardTriggerProps { class, children } = props.clone();

    let context = use_context::<HoverCardContext>();

    let on_mouse_enter = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ctx) = context.as_ref() {
                ctx.schedule_open.emit(());
            }
        })
    };

    let on_mouse_leave = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ctx) = context.as_ref() {
                ctx.schedule_close.emit(());
            }
        })
    };

    let on_focus = {
        let context = context.clone();
        Callback::from(move |_: FocusEvent| {
            if let Some(ctx) = context.as_ref() {
                ctx.schedule_open.emit(());
            }
        })
    };

    let on_blur = {
        let context = context.clone();
        Callback::from(move |e: FocusEvent| {
            // Check if focus moved to a descendant; if so, keep card open
            if let Some(related) = e.related_target()
                && let Some(current) = e.current_target()
                && let (Ok(parent), Ok(child)) = (
                    current.dyn_into::<web_sys::Node>(),
                    related.dyn_into::<web_sys::Node>(),
                )
                && parent.contains(Some(&child))
            {
                return;
            }
            if let Some(ctx) = context.as_ref() {
                ctx.schedule_close.emit(());
            }
        })
    };

    let classes: Classes = vec![Classes::from("hover-card-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <div
            class={classes}
            tabindex="0"
            onmouseenter={on_mouse_enter}
            onmouseleave={on_mouse_leave}
            onfocusin={on_focus}
            onfocusout={on_blur}
        >
            { children }
        </div>
    }
}

/// Hover Card content properties
#[derive(Properties, PartialEq, Clone)]
pub struct HoverCardContentProps {
    /// Whether the hover card is open
    #[prop_or(false)]
    pub open: bool,

    /// Callback to close the hover card
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    /// Hover card position relative to trigger
    #[prop_or(Position::Bottom)]
    pub position: Position,

    /// Alignment of hover card
    #[prop_or_default]
    pub align: Option<AttrValue>,

    /// Whether to show an arrow pointing to the trigger
    #[prop_or(true)]
    pub show_arrow: bool,

    /// Whether to close on Escape key
    #[prop_or(true)]
    pub close_on_escape: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Hover Card content component
///
/// The rich content that appears in the hover card.
#[function_component(HoverCardContent)]
pub fn hover_card_content(props: &HoverCardContentProps) -> Html {
    let HoverCardContentProps {
        open: prop_open,
        on_close,
        position,
        align,
        show_arrow,
        close_on_escape,
        class,
        children,
    } = props.clone();

    let context = use_context::<HoverCardContext>();

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

    if !is_open {
        return html! {};
    }

    let position_class = match position {
        Position::Top => "hover-card-top",
        Position::Right => "hover-card-right",
        Position::Bottom => "hover-card-bottom",
        Position::Left => "hover-card-left",
        Position::TopLeft => "hover-card-top-left",
        Position::TopRight => "hover-card-top-right",
        Position::BottomLeft => "hover-card-bottom-left",
        Position::BottomRight => "hover-card-bottom-right",
        Position::Center => "hover-card-center",
    };

    let align_class = align
        .as_ref()
        .map(|a| format!("hover-card-align-{}", a))
        .unwrap_or_default();

    let classes: Classes = vec![
        Classes::from("hover-card-content"),
        Classes::from(position_class),
        if !align_class.is_empty() {
            Classes::from(align_class)
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    // Pointer bridge: entering the card cancels the close scheduled when the
    // pointer left the trigger; leaving the card schedules a close again.
    let onmouseenter = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ctx) = context.as_ref() {
                ctx.schedule_open.emit(());
            }
        })
    };
    let onmouseleave = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ctx) = context.as_ref() {
                ctx.schedule_close.emit(());
            }
        })
    };

    let content = html! {
        <div
            class={classes}
            role="region"
            aria-label="Additional information"
            data-state="open"
            {onmouseenter}
            {onmouseleave}
        >
            if show_arrow {
                <div class="hover-card-arrow" aria-hidden="true" />
            }
            { children }
        </div>
    };

    // Inside a HoverCard root the card is anchored next to the trigger (the
    // root is position: relative); standalone use keeps the Portal.
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
    fn test_hover_card_props_default() {
        let props = HoverCardProps {
            open: None,
            default_open: false,
            on_open_change: None,
            open_delay: 200,
            close_delay: 300,
            children: Children::new(vec![]),
        };

        assert_eq!(props.open, None);
        assert!(!props.default_open);
        assert_eq!(props.open_delay, 200);
        assert_eq!(props.close_delay, 300);
    }

    #[test]
    fn test_hover_card_custom_delays() {
        let props = HoverCardProps {
            open: None,
            default_open: false,
            on_open_change: None,
            open_delay: 500,
            close_delay: 100,
            children: Children::new(vec![]),
        };

        assert_eq!(props.open_delay, 500);
        assert_eq!(props.close_delay, 100);
    }

    #[test]
    fn test_hover_card_content_default() {
        let props = HoverCardContentProps {
            open: true,
            on_close: None,
            position: Position::Bottom,
            align: None,
            show_arrow: true,
            close_on_escape: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.show_arrow);
        assert!(props.close_on_escape);
        assert_eq!(props.position, Position::Bottom);
    }

    #[test]
    fn test_hover_card_content_no_arrow() {
        let props = HoverCardContentProps {
            open: true,
            on_close: None,
            position: Position::Top,
            align: Some(AttrValue::from("start")),
            show_arrow: false,
            close_on_escape: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.show_arrow);
        assert_eq!(props.position, Position::Top);
        assert_eq!(props.align, Some(AttrValue::from("start")));
    }

    #[test]
    fn test_hover_card_positions() {
        let positions = vec![
            Position::Top,
            Position::Right,
            Position::Bottom,
            Position::Left,
        ];

        for pos in positions {
            let props = HoverCardContentProps {
                open: true,
                on_close: None,
                position: pos.clone(),
                align: None,
                show_arrow: true,
                close_on_escape: true,
                class: Classes::new(),
                children: Children::new(vec![]),
            };

            assert_eq!(props.position, pos);
        }
    }
}
