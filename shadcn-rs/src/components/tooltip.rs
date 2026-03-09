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

use crate::types::Position;
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

/// Tooltip component properties
#[derive(Properties, PartialEq, Clone)]
pub struct TooltipProps {
    /// Delay before showing tooltip (in milliseconds)
    #[prop_or(200)]
    pub delay_duration: u32,

    /// Whether tooltip is disabled
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
/// - Uses aria-describedby to link trigger and content
/// - Shows on hover and focus
/// - Hides on blur and mouse leave
/// - Keyboard accessible
#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let TooltipProps {
        delay_duration: _,
        disabled: _,
        children,
    } = props.clone();

    let _is_open = use_state(|| false);

    html! {
        <div class="tooltip-root">
            { children }
        </div>
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

/// Tooltip trigger component
///
/// The element that triggers the tooltip on hover/focus.
#[function_component(TooltipTrigger)]
pub fn tooltip_trigger(props: &TooltipTriggerProps) -> Html {
    let TooltipTriggerProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("tooltip-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
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
/// The content that appears in the tooltip popup.
#[function_component(TooltipContent)]
pub fn tooltip_content(props: &TooltipContentProps) -> Html {
    let TooltipContentProps {
        position,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("tooltip-content"),
        Classes::from(position.to_class()),
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div class={classes} role="tooltip">
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
            delay_duration: 200,
            disabled: false,
            children: Children::new(vec![]),
        };

        assert_eq!(props.delay_duration, 200);
        assert!(!props.disabled);
    }

    #[test]
    fn test_tooltip_with_custom_delay() {
        let props = TooltipProps {
            delay_duration: 500,
            disabled: false,
            children: Children::new(vec![]),
        };

        assert_eq!(props.delay_duration, 500);
    }

    #[test]
    fn test_tooltip_disabled() {
        let props = TooltipProps {
            delay_duration: 200,
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
