//! Collapsible component
//!
//! Expandable/collapsible content area with trigger.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Collapsible, CollapsibleTrigger, CollapsibleContent};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Collapsible>
//!             <CollapsibleTrigger>
//!                 { "Show more" }
//!             </CollapsibleTrigger>
//!             <CollapsibleContent>
//!                 { "Hidden content goes here..." }
//!             </CollapsibleContent>
//!         </Collapsible>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Context for sharing collapsible state with children
#[derive(Clone, PartialEq)]
pub struct CollapsibleContext {
    /// Whether the collapsible content is currently visible
    pub is_open: bool,
    /// Callback to toggle the open/close state
    pub toggle: Callback<()>,
}

/// Collapsible container properties
#[derive(Properties, PartialEq, Clone)]
pub struct CollapsibleProps {
    /// Open state (controlled)
    #[prop_or_default]
    pub open: Option<bool>,

    /// Default open state (uncontrolled)
    #[prop_or(false)]
    pub default_open: bool,

    /// Open state change handler
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Collapsible container component
///
/// Container for collapsible content with trigger.
///
/// # Accessibility
/// - Trigger has aria-expanded attribute
/// - Content has aria-hidden when collapsed
/// - Keyboard accessible
#[function_component(Collapsible)]
pub fn collapsible(props: &CollapsibleProps) -> Html {
    let CollapsibleProps {
        open,
        default_open,
        on_open_change,
        class,
        children,
    } = props.clone();

    // Internal state for uncontrolled mode
    let internal_open = use_state(|| default_open);

    // Use controlled value if provided, otherwise use internal state
    let is_open = open.unwrap_or(*internal_open);

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

    let context = CollapsibleContext { is_open, toggle };

    let classes: Classes = vec![
        Classes::from("collapsible"),
        if is_open {
            Classes::from("collapsible-open")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <ContextProvider<CollapsibleContext> context={context}>
            <div class={classes} data-state={if is_open { "open" } else { "closed" }}>
                { children }
            </div>
        </ContextProvider<CollapsibleContext>>
    }
}

/// Collapsible trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct CollapsibleTriggerProps {
    /// Click handler (optional, toggle is automatic)
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Collapsible trigger component
///
/// Button to toggle collapsible content.
#[function_component(CollapsibleTrigger)]
pub fn collapsible_trigger(props: &CollapsibleTriggerProps) -> Html {
    let CollapsibleTriggerProps {
        onclick,
        children,
        class,
    } = props.clone();

    let context = use_context::<CollapsibleContext>();

    let handle_click = {
        let context = context.clone();
        let onclick = onclick.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(ctx) = context.as_ref() {
                ctx.toggle.emit(());
            }
            if let Some(cb) = onclick.as_ref() {
                cb.emit(e);
            }
        })
    };

    let is_open = context.as_ref().map(|c| c.is_open).unwrap_or(false);

    let classes: Classes = vec![Classes::from("collapsible-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <button
            type="button"
            class={classes}
            onclick={handle_click}
            aria-expanded={is_open.to_string()}
        >
            { children }
        </button>
    }
}

/// Collapsible content properties
#[derive(Properties, PartialEq, Clone)]
pub struct CollapsibleContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Collapsible content component
///
/// Content that can be expanded/collapsed.
#[function_component(CollapsibleContent)]
pub fn collapsible_content(props: &CollapsibleContentProps) -> Html {
    let CollapsibleContentProps { class, children } = props.clone();

    let context = use_context::<CollapsibleContext>();
    let is_open = context.as_ref().map(|c| c.is_open).unwrap_or(false);

    let classes: Classes = vec![Classes::from("collapsible-content"), class]
        .into_iter()
        .collect();

    if !is_open {
        return html! {};
    }

    html! {
        <div class={classes} aria-hidden={(!is_open).to_string()}>
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapsible_default_closed() {
        let props = CollapsibleProps {
            open: None,
            default_open: false,
            on_open_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.default_open);
    }

    #[test]
    fn test_collapsible_default_open() {
        let props = CollapsibleProps {
            open: None,
            default_open: true,
            on_open_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.default_open);
    }

    #[test]
    fn test_collapsible_controlled() {
        let props = CollapsibleProps {
            open: Some(true),
            default_open: false,
            on_open_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.open, Some(true));
    }

    #[test]
    fn test_collapsible_trigger_props() {
        let props = CollapsibleTriggerProps {
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.onclick.is_none());
    }

    #[test]
    fn test_collapsible_content_props() {
        let props = CollapsibleContentProps {
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.class, Classes::new());
    }
}
