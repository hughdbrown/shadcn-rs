//! Switch component
//!
//! A toggle switch control.
//!
//! # Examples
//!
//! Controlled: the parent owns the state and `on_checked_change` reports the
//! new value.
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Switch, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let enabled = use_state(|| false);
//!
//!     let on_checked_change = {
//!         let enabled = enabled.clone();
//!         Callback::from(move |value: bool| enabled.set(value))
//!     };
//!
//!     html! {
//!         <div class="flex items-center space-x-2">
//!             <Switch id="airplane-mode" checked={*enabled} {on_checked_change} />
//!             <Label html_for="airplane-mode">{ "Airplane Mode" }</Label>
//!         </div>
//!     }
//! }
//! ```
//!
//! Uncontrolled: leave `checked` unset and give an initial `default_checked`.
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::Switch;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! { <Switch id="wifi" default_checked={true} /> }
//! }
//! ```

use crate::hooks::use_controllable_bool;
use crate::types::Size;
use crate::utils::class_names;
use yew::prelude::*;

/// Switch component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SwitchProps {
    /// Checked state. `Some` makes the switch controlled: it only changes when
    /// the parent passes a new value. `None` leaves it uncontrolled.
    #[prop_or_default]
    pub checked: Option<bool>,

    /// Initial checked state for an uncontrolled switch (`checked` unset)
    #[prop_or(false)]
    pub default_checked: bool,

    /// Switch size
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Required field
    #[prop_or(false)]
    pub required: bool,

    /// Name attribute
    #[prop_or_default]
    pub name: Option<AttrValue>,

    /// Value attribute
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Raw event handler (the click or key event that toggled the switch)
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,

    /// Called with the new checked value when the user toggles the switch
    #[prop_or_default]
    pub on_checked_change: Option<Callback<bool>>,

    /// Focus event handler
    #[prop_or_default]
    pub onfocus: Option<Callback<FocusEvent>>,

    /// Blur event handler
    #[prop_or_default]
    pub onblur: Option<Callback<FocusEvent>>,

    /// ARIA label
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// ARIA described by
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// Node ref (for the button element)
    #[prop_or_default]
    pub node_ref: NodeRef,
}

/// Switch component
///
/// A toggle switch component that acts like a checkbox but with a switch UI.
///
/// # States
/// - Off: Not enabled (unchecked)
/// - On: Enabled (checked)
/// - Disabled: Non-interactive
///
/// # Controlled and uncontrolled
/// Pass `checked` to control the switch from the parent, or leave it unset and
/// use `default_checked` for the initial value. Both modes report changes
/// through `on_checked_change` (the new value) and `onchange` (the raw event).
///
/// # Accessibility
/// - Uses `role="switch"`
/// - Supports ARIA attributes
/// - Keyboard navigation (Space/Enter to toggle)
/// - Screen reader friendly
/// - Announces checked state changes
///
/// # Implementation
/// Unlike a checkbox, the Switch is implemented as a button with role="switch"
/// for better semantic meaning and accessibility.
#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let SwitchProps {
        checked,
        default_checked,
        size,
        disabled,
        required,
        name,
        value,
        id,
        onchange,
        on_checked_change,
        onfocus,
        onblur,
        aria_label,
        aria_describedby,
        class,
        style,
        node_ref,
    } = props.clone();

    let (is_checked, set_checked) =
        use_controllable_bool(checked, default_checked, on_checked_change);

    // Shared by click and keyboard activation
    let toggle = Callback::from(move |event: Event| {
        if disabled {
            return;
        }
        if let Some(callback) = onchange.as_ref() {
            callback.emit(event);
        }
        set_checked.emit(!is_checked);
    });

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |e: MouseEvent| toggle.emit(e.into()))
    };

    let onkeydown = Callback::from(move |e: KeyboardEvent| {
        let key = e.key();
        if key == " " || key == "Enter" {
            e.prevent_default();
            toggle.emit(e.into());
        }
    });

    // Build class names
    let classes = class_names(&[
        Some("switch"),
        Some(size.to_class()),
        if is_checked {
            Some("switch-checked")
        } else {
            None
        },
        if disabled {
            Some("switch-disabled")
        } else {
            None
        },
    ]);

    // Merge with custom classes
    let final_classes: Classes = vec![classes, class].into_iter().collect();

    html! {
        <button
            ref={node_ref}
            type="button"
            role="switch"
            class={final_classes}
            aria-checked={is_checked.to_string()}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            disabled={disabled}
            onclick={onclick}
            onkeydown={onkeydown}
            onfocus={onfocus}
            onblur={onblur}
            style={style}
            id={id}
        >
            <span class="switch-thumb" aria-hidden="true"></span>
            // Hidden input for form submission
            if let Some(name_value) = name {
                <input
                    type="checkbox"
                    name={name_value}
                    value={value}
                    checked={is_checked}
                    required={required}
                    tabindex="-1"
                    style="position: absolute; pointer-events: none; opacity: 0; margin: 0;"
                    aria-hidden="true"
                />
            }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_props_default() {
        let props = yew::props!(SwitchProps {});
        assert_eq!(props.checked, None);
        assert!(!props.default_checked);
        assert_eq!(props.size, Size::Md);
        assert!(!props.disabled);
        assert!(props.on_checked_change.is_none());
    }

    #[test]
    fn test_switch_checked_is_controlled() {
        let on = yew::props!(SwitchProps { checked: true });
        assert_eq!(on.checked, Some(true));

        // A parent can force the switch off, which `checked: bool` could not express.
        let off = yew::props!(SwitchProps {
            checked: false,
            default_checked: true
        });
        assert_eq!(off.checked, Some(false));
    }

    #[test]
    fn test_switch_disabled() {
        let props = yew::props!(SwitchProps { disabled: true });
        assert!(props.disabled);
    }

    #[test]
    fn test_switch_with_name() {
        let props = yew::props!(SwitchProps {
            checked: true,
            name: AttrValue::from("setting"),
            value: AttrValue::from("on"),
        });
        assert_eq!(props.name, Some(AttrValue::from("setting")));
        assert_eq!(props.value, Some(AttrValue::from("on")));
    }
}
