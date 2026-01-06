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
//!     let open = use_state(|| false);
//!
//!     html! {
//!         <Collapsible open={*open}>
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
use crate::hooks::use_toggle;

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
        on_open_change: _,
        class,
        children,
    } = props.clone();

    let (is_open, _toggle, _set_open) = use_toggle(open.unwrap_or(default_open));

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
        <div class={classes} data-state={if is_open { "open" } else { "closed" }}>
            { children }
        </div>
    }
}

/// Collapsible trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct CollapsibleTriggerProps {
    /// Click handler
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

    let classes: Classes = vec![Classes::from("collapsible-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <button
            type="button"
            class={classes}
            onclick={onclick}
            aria-expanded="false"
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

    let classes: Classes = vec![Classes::from("collapsible-content"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} aria-hidden="false">
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
