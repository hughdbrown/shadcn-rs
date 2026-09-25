//! Tooltip component
//!
//! A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Tooltip, TooltipTrigger, TooltipContent, Button};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Tooltip>
//!             <TooltipTrigger>
//!                 <Button>{ "Hover me" }</Button>
//!             </TooltipTrigger>
//!             <TooltipContent>
//!                 { "This is a tooltip" }
//!             </TooltipContent>
//!         </Tooltip>
//!     }
//! }
//! ```

use crate::hooks::{use_controllable_bool, use_escape_key_conditional};
use crate::types::Position;
use crate::utils::generate_id;
use gloo::timers::callback::Timeout;
use yew::prelude::*;

/// Context for sharing tooltip configuration through the component tree
#[derive(Clone, Debug, PartialEq)]
pub struct TooltipContext {
    /// Default delay before showing tooltips (in milliseconds)
    pub delay_duration: u32,
    /// Duration to skip delay when moving between tooltips (in milliseconds)
    pub skip_delay_duration: u32,
}

/// TooltipProvider component properties
#[derive(Properties, PartialEq, Clone)]
pub struct TooltipProviderProps {
    /// Default delay before showing tooltips (in milliseconds)
    #[prop_or(200)]
    pub delay_duration: u32,

    /// Duration to skip delay when quickly moving between tooltips (in milliseconds)
    ///
    /// Not applied yet: each tooltip currently waits its full delay.
    #[prop_or(300)]
    pub skip_delay_duration: u32,

    /// Children elements
    pub children: Children,
}

/// TooltipProvider component
///
/// Wraps the application or a subtree to provide global tooltip configuration.
/// All `Tooltip` components within this provider will inherit the configured
/// delay settings unless overridden individually.
///
/// # Examples
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::{TooltipProvider, Tooltip, TooltipTrigger, TooltipContent, Button};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <TooltipProvider delay_duration={400}>
///             <Tooltip>
///                 <TooltipTrigger>
///                     <Button>{ "Hover me" }</Button>
///                 </TooltipTrigger>
///                 <TooltipContent>
///                     { "This tooltip has a 400ms delay" }
///                 </TooltipContent>
///             </Tooltip>
///         </TooltipProvider>
///     }
/// }
/// ```
#[function_component(TooltipProvider)]
pub fn tooltip_provider(props: &TooltipProviderProps) -> Html {
    let TooltipProviderProps {
        delay_duration,
        skip_delay_duration,
        children,
    } = props.clone();

    let context = TooltipContext {
        delay_duration,
        skip_delay_duration,
    };

    html! {
        <ContextProvider<TooltipContext> {context}>
            { children }
        </ContextProvider<TooltipContext>>
    }
}

/// Delay used when neither the tooltip nor a [`TooltipProvider`] sets one (ms).
pub const DEFAULT_TOOLTIP_DELAY: u32 = 200;

/// Resolves the open delay: the tooltip's own value wins, then the nearest
/// [`TooltipProvider`], then [`DEFAULT_TOOLTIP_DELAY`].
pub fn resolve_tooltip_delay(own: Option<u32>, provider: Option<&TooltipContext>) -> u32 {
    own.or_else(|| provider.map(|ctx| ctx.delay_duration))
        .unwrap_or(DEFAULT_TOOLTIP_DELAY)
}

/// State shared by a [`Tooltip`] root with its trigger and content.
#[derive(Clone, PartialEq)]
pub struct TooltipStateContext {
    /// Whether the tooltip is currently shown
    pub is_open: bool,
    /// Id of the content element, referenced by the trigger's `aria-describedby`
    pub content_id: AttrValue,
    /// Pointer entered the trigger: open after the delay
    pub on_pointer_enter: Callback<()>,
    /// Keyboard focus reached the trigger: open immediately
    pub on_focus: Callback<()>,
    /// Pointer left, focus left or Escape: close (and cancel a pending open)
    pub on_close: Callback<()>,
}

/// Tooltip component properties
#[derive(Properties, PartialEq, Clone)]
pub struct TooltipProps {
    /// Delay before showing the tooltip on hover (in milliseconds).
    ///
    /// `None` (the default) uses the nearest [`TooltipProvider`]'s
    /// `delay_duration`, or 200 ms without a provider. Keyboard focus opens
    /// immediately.
    #[prop_or_default]
    pub delay_duration: Option<u32>,

    /// Whether the tooltip is shown (controlled). `None` leaves it uncontrolled.
    #[prop_or_default]
    pub open: Option<bool>,

    /// Initial open state for an uncontrolled tooltip
    #[prop_or(false)]
    pub default_open: bool,

    /// Called with the requested open state on hover, focus, leave, blur and Escape
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Whether tooltip is disabled (never opens)
    #[prop_or(false)]
    pub disabled: bool,

    /// Children elements
    pub children: Children,
}

/// Tooltip component
///
/// A container for tooltip trigger and content.
///
/// # Accessibility
/// - The trigger references the open content with `aria-describedby`
/// - Shows on pointer hover (after the delay) and immediately on keyboard focus
/// - Hides on pointer leave, blur and Escape
#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let TooltipProps {
        delay_duration,
        open,
        default_open,
        on_open_change,
        disabled,
        children,
    } = props.clone();

    let provider = use_context::<TooltipContext>();
    let delay = resolve_tooltip_delay(delay_duration, provider.as_ref());
    let content_id = use_state(|| AttrValue::from(generate_id("tooltip")));
    let pending_open = use_mut_ref(|| None::<Timeout>);

    let (is_open, set_open) = use_controllable_bool(open, default_open, on_open_change);
    let is_open = is_open && !disabled;

    let on_pointer_enter = {
        let set_open = set_open.clone();
        let pending_open = pending_open.clone();
        Callback::from(move |_: ()| {
            if disabled || is_open {
                return;
            }
            if delay == 0 {
                set_open.emit(true);
                return;
            }
            let set_open = set_open.clone();
            // Replacing the handle drops (cancels) any earlier pending open.
            *pending_open.borrow_mut() = Some(Timeout::new(delay, move || set_open.emit(true)));
        })
    };

    let on_focus = {
        let set_open = set_open.clone();
        let pending_open = pending_open.clone();
        Callback::from(move |_: ()| {
            pending_open.borrow_mut().take();
            if !disabled && !is_open {
                set_open.emit(true);
            }
        })
    };

    let on_close = {
        let pending_open = pending_open.clone();
        Callback::from(move |_: ()| {
            pending_open.borrow_mut().take();
            if is_open {
                set_open.emit(false);
            }
        })
    };

    {
        let on_close = on_close.clone();
        use_escape_key_conditional(move || on_close.emit(()), is_open);
    }

    let context = TooltipStateContext {
        is_open,
        content_id: (*content_id).clone(),
        on_pointer_enter,
        on_focus,
        on_close,
    };

    html! {
        <ContextProvider<TooltipStateContext> {context}>
            <div
                class="tooltip-root tooltip"
                data-state={if is_open { "open" } else { "closed" }}
            >
                { children }
            </div>
        </ContextProvider<TooltipStateContext>>
    }
}

/// Tooltip trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct TooltipTriggerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Builds a DOM event handler that forwards to one of the root's callbacks.
fn forward<E: 'static>(
    context: &Option<TooltipStateContext>,
    pick: fn(&TooltipStateContext) -> &Callback<()>,
) -> Callback<E> {
    let context = context.clone();
    Callback::from(move |_: E| {
        if let Some(ctx) = context.as_ref() {
            pick(ctx).emit(());
        }
    })
}

/// Tooltip trigger component
///
/// The element that triggers the tooltip on hover/focus.
#[function_component(TooltipTrigger)]
pub fn tooltip_trigger(props: &TooltipTriggerProps) -> Html {
    let TooltipTriggerProps { class, children } = props.clone();

    let context = use_context::<TooltipStateContext>();

    let onmouseenter: Callback<MouseEvent> = forward(&context, |ctx| &ctx.on_pointer_enter);
    let onmouseleave: Callback<MouseEvent> = forward(&context, |ctx| &ctx.on_close);
    let onfocusin: Callback<FocusEvent> = forward(&context, |ctx| &ctx.on_focus);
    let onfocusout: Callback<FocusEvent> = forward(&context, |ctx| &ctx.on_close);

    let describedby = context
        .as_ref()
        .filter(|ctx| ctx.is_open)
        .map(|ctx| ctx.content_id.clone());

    let classes: Classes = vec![Classes::from("tooltip-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <div
            class={classes}
            aria-describedby={describedby}
            {onmouseenter}
            {onmouseleave}
            {onfocusin}
            {onfocusout}
        >
            { children }
        </div>
    }
}

/// Tooltip content properties
#[derive(Properties, PartialEq, Clone)]
pub struct TooltipContentProps {
    /// Tooltip position relative to trigger
    #[prop_or(Position::Top)]
    pub position: Position,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Tooltip content component
///
/// The content that appears in the tooltip popup. Rendered only while the
/// parent [`Tooltip`] is open; outside a `Tooltip` it is always shown.
#[function_component(TooltipContent)]
pub fn tooltip_content(props: &TooltipContentProps) -> Html {
    let TooltipContentProps {
        position,
        class,
        children,
    } = props.clone();

    let context = use_context::<TooltipStateContext>();
    if context.as_ref().is_some_and(|ctx| !ctx.is_open) {
        return html! {};
    }
    let id = context.map(|ctx| ctx.content_id);

    let classes: Classes = vec![
        Classes::from("tooltip-content"),
        Classes::from(position.to_class()),
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div {id} class={classes} role="tooltip" data-state="open">
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_props_default() {
        let props = TooltipProps {
            delay_duration: Some(200),
            open: None,
            default_open: false,
            on_open_change: None,
            disabled: false,
            children: Children::new(vec![]),
        };

        assert_eq!(props.delay_duration, Some(200));
        assert!(!props.disabled);
    }

    #[test]
    fn test_tooltip_with_custom_delay() {
        let props = TooltipProps {
            delay_duration: Some(500),
            open: None,
            default_open: false,
            on_open_change: None,
            disabled: false,
            children: Children::new(vec![]),
        };

        assert_eq!(props.delay_duration, Some(500));
    }

    #[test]
    fn test_tooltip_disabled() {
        let props = TooltipProps {
            delay_duration: None,
            open: None,
            default_open: false,
            on_open_change: None,
            disabled: true,
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_tooltip_provider_defaults() {
        let props = TooltipProviderProps {
            delay_duration: 200,
            skip_delay_duration: 300,
            children: Children::new(vec![]),
        };

        assert_eq!(props.delay_duration, 200);
        assert_eq!(props.skip_delay_duration, 300);
    }

    #[test]
    fn test_tooltip_provider_custom() {
        let props = TooltipProviderProps {
            delay_duration: 500,
            skip_delay_duration: 100,
            children: Children::new(vec![]),
        };

        assert_eq!(props.delay_duration, 500);
        assert_eq!(props.skip_delay_duration, 100);
    }

    #[test]
    fn test_resolve_tooltip_delay_precedence() {
        let provider = TooltipContext {
            delay_duration: 700,
            skip_delay_duration: 300,
        };

        assert_eq!(resolve_tooltip_delay(None, None), DEFAULT_TOOLTIP_DELAY);
        assert_eq!(resolve_tooltip_delay(None, Some(&provider)), 700);
        assert_eq!(resolve_tooltip_delay(Some(0), Some(&provider)), 0);
        assert_eq!(resolve_tooltip_delay(Some(50), None), 50);
    }

    #[test]
    fn test_tooltip_context_equality() {
        let ctx1 = TooltipContext {
            delay_duration: 200,
            skip_delay_duration: 300,
        };
        let ctx2 = TooltipContext {
            delay_duration: 200,
            skip_delay_duration: 300,
        };
        let ctx3 = TooltipContext {
            delay_duration: 400,
            skip_delay_duration: 300,
        };

        assert_eq!(ctx1, ctx2);
        assert_ne!(ctx1, ctx3);
    }

    #[test]
    fn test_tooltip_content_positions() {
        let positions = vec![
            Position::Top,
            Position::Right,
            Position::Bottom,
            Position::Left,
        ];

        for position in positions {
            let props = TooltipContentProps {
                position: position.clone(),
                class: Classes::new(),
                children: Children::new(vec![]),
            };
            assert_eq!(props.position, position);
        }
    }
}
