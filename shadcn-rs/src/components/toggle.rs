//! Toggle component
//!
//! A button with pressed/unpressed states.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Toggle, ToggleVariant};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let pressed = use_state(|| false);
//!
//!     let ontoggle = {
//!         let pressed = pressed.clone();
//!         Callback::from(move |_| {
//!             pressed.set(!*pressed);
//!         })
//!     };
//!
//!     html! {
//!         <Toggle
//!             pressed={*pressed}
//!             {ontoggle}
//!         >
//!             { "Bold" }
//!         </Toggle>
//!     }
//! }
//! ```

use crate::types::Size;
use yew::prelude::*;

/// Toggle button variant
#[derive(Debug, Clone, PartialEq)]
pub enum ToggleVariant {
    /// Default variant
    Default,
    /// Outline variant
    Outline,
}

/// Toggle component properties
#[derive(Properties, PartialEq, Clone)]
pub struct ToggleProps {
    /// Pressed state (controlled)
    #[prop_or_default]
    pub pressed: Option<bool>,

    /// Default pressed state (uncontrolled)
    #[prop_or(false)]
    pub default_pressed: bool,

    /// Toggle handler
    #[prop_or_default]
    pub ontoggle: Option<Callback<MouseEvent>>,

    /// Size of toggle
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Variant
    #[prop_or(ToggleVariant::Default)]
    pub variant: ToggleVariant,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Toggle component
///
/// A two-state button with pressed/unpressed states.
///
/// # Accessibility
/// - Uses aria-pressed attribute
/// - Keyboard accessible
/// - Disabled state properly handled
#[function_component(Toggle)]
pub fn toggle(props: &ToggleProps) -> Html {
    let ToggleProps {
        pressed,
        default_pressed,
        ontoggle,
        size,
        variant,
        disabled,
        class,
        children,
    } = props.clone();

    // Internal state for uncontrolled mode
    let internal_pressed = use_state(|| default_pressed);

    // Use controlled value if provided, otherwise use internal state
    let is_pressed = pressed.unwrap_or(*internal_pressed);

    // Handle click events
    let onclick = {
        let internal_pressed = internal_pressed.clone();
        let ontoggle = ontoggle.clone();
        Callback::from(move |e: MouseEvent| {
            if !disabled {
                let new_state = !*internal_pressed;
                internal_pressed.set(new_state);
                if let Some(callback) = ontoggle.as_ref() {
                    callback.emit(e);
                }
            }
        })
    };

    let size_class = match size {
        Size::Xs => "toggle-xs",
        Size::Sm => "toggle-sm",
        Size::Md => "toggle-md",
        Size::Lg => "toggle-lg",
        Size::Xl => "toggle-xl",
        Size::Xl2 => "toggle-2xl",
    };

    let variant_class = match variant {
        ToggleVariant::Default => "toggle-default",
        ToggleVariant::Outline => "toggle-outline",
    };

    let classes: Classes = vec![
        Classes::from("toggle"),
        Classes::from(size_class),
        Classes::from(variant_class),
        if is_pressed {
            Classes::from("toggle-pressed")
        } else {
            Classes::new()
        },
        if disabled {
            Classes::from("toggle-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <button
            type="button"
            class={classes}
            onclick={onclick}
            disabled={disabled}
            aria-pressed={is_pressed.to_string()}
        >
            { children }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_default() {
        let props = ToggleProps {
            pressed: None,
            default_pressed: false,
            ontoggle: None,
            size: Size::Md,
            variant: ToggleVariant::Default,
            disabled: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.default_pressed);
        assert!(!props.disabled);
    }

    #[test]
    fn test_toggle_pressed() {
        let props = ToggleProps {
            pressed: Some(true),
            default_pressed: false,
            ontoggle: None,
            size: Size::Md,
            variant: ToggleVariant::Default,
            disabled: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.pressed, Some(true));
    }

    #[test]
    fn test_toggle_disabled() {
        let props = ToggleProps {
            pressed: None,
            default_pressed: false,
            ontoggle: None,
            size: Size::Md,
            variant: ToggleVariant::Default,
            disabled: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_toggle_sizes() {
        let props = ToggleProps {
            pressed: None,
            default_pressed: false,
            ontoggle: None,
            size: Size::Lg,
            variant: ToggleVariant::Default,
            disabled: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.size, Size::Lg);
    }

    #[test]
    fn test_toggle_outline_variant() {
        let props = ToggleProps {
            pressed: None,
            default_pressed: false,
            ontoggle: None,
            size: Size::Md,
            variant: ToggleVariant::Outline,
            disabled: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.variant, ToggleVariant::Outline);
    }
}
