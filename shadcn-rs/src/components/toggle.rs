//! Toggle component
//!
//! A button with pressed/unpressed states.
//!
//! # Examples
//!
//! Controlled: the parent owns the state and `on_pressed_change` reports the
//! new value.
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Toggle, ToggleVariant};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let pressed = use_state(|| false);
//!
//!     let on_pressed_change = {
//!         let pressed = pressed.clone();
//!         Callback::from(move |value: bool| pressed.set(value))
//!     };
//!
//!     html! {
//!         <Toggle
//!             pressed={*pressed}
//!             {on_pressed_change}
//!             aria_label="Toggle bold"
//!         >
//!             <strong>{ "B" }</strong>
//!         </Toggle>
//!     }
//! }
//! ```
//!
//! Uncontrolled: leave `pressed` unset and use `default_pressed`.
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::Toggle;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! { <Toggle default_pressed={true} aria_label="Toggle italic"><em>{ "I" }</em></Toggle> }
//! }
//! ```

use crate::hooks::use_controllable_bool;
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
    /// Pressed state. `Some` makes the toggle controlled: it only changes when
    /// the parent passes a new value. `None` leaves it uncontrolled.
    #[prop_or_default]
    pub pressed: Option<bool>,

    /// Initial pressed state for an uncontrolled toggle (`pressed` unset)
    #[prop_or(false)]
    pub default_pressed: bool,

    /// Raw click handler, called whenever the toggle is activated
    #[prop_or_default]
    pub ontoggle: Option<Callback<MouseEvent>>,

    /// Called with the new pressed value when the user toggles the button
    #[prop_or_default]
    pub on_pressed_change: Option<Callback<bool>>,

    /// Accessible name, needed when the toggle only shows an icon
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

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
/// # Controlled and uncontrolled
/// Pass `pressed` to control the toggle from the parent, or leave it unset and
/// use `default_pressed`. Both modes report changes through
/// `on_pressed_change` (the new value) and `ontoggle` (the raw click).
///
/// # Accessibility
/// - Uses aria-pressed attribute
/// - `aria_label` names icon-only toggles
/// - Keyboard accessible
/// - Disabled state properly handled
#[function_component(Toggle)]
pub fn toggle(props: &ToggleProps) -> Html {
    let ToggleProps {
        pressed,
        default_pressed,
        ontoggle,
        on_pressed_change,
        aria_label,
        size,
        variant,
        disabled,
        class,
        children,
    } = props.clone();

    let (is_pressed, set_pressed) =
        use_controllable_bool(pressed, default_pressed, on_pressed_change);

    // Handle click events
    let onclick = Callback::from(move |e: MouseEvent| {
        if disabled {
            return;
        }
        if let Some(callback) = ontoggle.as_ref() {
            callback.emit(e);
        }
        set_pressed.emit(!is_pressed);
    });

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
            aria-label={aria_label}
            data-state={if is_pressed { "on" } else { "off" }}
        >
            { children }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn props_with(pressed: Option<bool>, variant: ToggleVariant, size: Size) -> ToggleProps {
        yew::props!(ToggleProps {
            pressed,
            variant,
            size,
            children: Children::new(vec![]),
        })
    }

    #[test]
    fn test_toggle_default() {
        let props = props_with(None, ToggleVariant::Default, Size::Md);
        assert_eq!(props.pressed, None);
        assert!(!props.default_pressed);
        assert!(!props.disabled);
        assert!(props.aria_label.is_none());
        assert!(props.on_pressed_change.is_none());
    }

    #[test]
    fn test_toggle_pressed() {
        let props = props_with(Some(true), ToggleVariant::Default, Size::Md);
        assert_eq!(props.pressed, Some(true));
    }

    #[test]
    fn test_toggle_disabled_and_labelled() {
        let props = yew::props!(ToggleProps {
            disabled: true,
            aria_label: "Toggle bold",
            children: Children::new(vec![]),
        });
        assert!(props.disabled);
        assert_eq!(props.aria_label, Some(AttrValue::from("Toggle bold")));
    }

    #[test]
    fn test_toggle_sizes() {
        let props = props_with(None, ToggleVariant::Default, Size::Lg);
        assert_eq!(props.size, Size::Lg);
    }

    #[test]
    fn test_toggle_outline_variant() {
        let props = props_with(None, ToggleVariant::Outline, Size::Md);
        assert_eq!(props.variant, ToggleVariant::Outline);
    }
}
