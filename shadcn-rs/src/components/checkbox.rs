//! Checkbox component
//!
//! A control that allows the user to toggle between checked and not checked.
//!
//! # Examples
//!
//! Controlled: the parent owns the state and `on_checked_change` reports the
//! value the user asked for.
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Checkbox, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let checked = use_state(|| false);
//!
//!     let on_checked_change = {
//!         let checked = checked.clone();
//!         Callback::from(move |value: bool| checked.set(value))
//!     };
//!
//!     html! {
//!         <div class="flex items-center space-x-2">
//!             <Checkbox id="terms" checked={*checked} {on_checked_change} />
//!             <Label html_for="terms">{ "Accept terms and conditions" }</Label>
//!         </div>
//!     }
//! }
//! ```
//!
//! Uncontrolled: leave `checked` unset and give an initial `default_checked`.
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::Checkbox;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! { <Checkbox id="newsletter" default_checked={true} /> }
//! }
//! ```

use crate::hooks::use_controllable_bool;
use crate::types::Size;
use crate::utils::class_names;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Checkbox component properties
#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    /// Checked state. `Some` makes the checkbox controlled: it only changes
    /// when the parent passes a new value. `None` leaves it uncontrolled.
    #[prop_or_default]
    pub checked: Option<bool>,

    /// Initial checked state for an uncontrolled checkbox (`checked` unset)
    #[prop_or(false)]
    pub default_checked: bool,

    /// Indeterminate state (partially checked)
    #[prop_or(false)]
    pub indeterminate: bool,

    /// Checkbox size
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Required field
    #[prop_or(false)]
    pub required: bool,

    /// Error state
    #[prop_or(false)]
    pub error: bool,

    /// Name attribute
    #[prop_or_default]
    pub name: Option<AttrValue>,

    /// Value attribute
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Raw change event handler
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,

    /// Called with the new checked value when the user toggles the checkbox
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

    /// ARIA invalid
    #[prop_or_default]
    pub aria_invalid: Option<bool>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// Node ref
    #[prop_or_default]
    pub node_ref: NodeRef,
}

/// Checkbox component
///
/// A checkbox control for binary or indeterminate selection.
///
/// # States
/// - Unchecked: Not selected
/// - Checked: Selected
/// - Indeterminate: Partially selected (e.g., some children selected)
/// - Disabled: Non-interactive
/// - Error: Invalid state
///
/// # Controlled and uncontrolled
/// Pass `checked` to control the checkbox from the parent, or leave it unset
/// and use `default_checked` for the initial value. Both modes report changes
/// through `on_checked_change` (the new value) and `onchange` (the raw event).
///
/// # Accessibility
/// - Supports ARIA attributes
/// - Keyboard navigation (Space to toggle)
/// - Screen reader friendly
/// - Indeterminate state announced properly
#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        checked,
        default_checked,
        indeterminate,
        size,
        disabled,
        required,
        error,
        name,
        value,
        id,
        onchange,
        on_checked_change,
        onfocus,
        onblur,
        aria_label,
        aria_describedby,
        aria_invalid,
        class,
        style,
        node_ref,
    } = props.clone();

    let (is_checked, set_checked) =
        use_controllable_bool(checked, default_checked, on_checked_change);

    // Set indeterminate state on mount and when it changes
    {
        let node_ref = node_ref.clone();
        use_effect_with(indeterminate, move |indeterminate| {
            if let Some(input) = node_ref.cast::<HtmlInputElement>() {
                input.set_indeterminate(*indeterminate);
            }
            || ()
        });
    }

    let handle_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let requested = input.checked();
        if let Some(callback) = onchange.as_ref() {
            callback.emit(e);
        }
        set_checked.emit(requested);
        // A controlled checkbox keeps showing the parent's value until the
        // parent re-renders with a new one, so undo the browser's own toggle.
        if let Some(controlled) = checked {
            input.set_checked(controlled);
        }
    });

    // Build class names
    let classes = class_names(&[
        Some("checkbox"),
        Some(size.to_class()),
        if error { Some("checkbox-error") } else { None },
        if disabled {
            Some("checkbox-disabled")
        } else {
            None
        },
        if indeterminate {
            Some("checkbox-indeterminate")
        } else {
            None
        },
    ]);

    // Merge with custom classes
    let final_classes: Classes = vec![classes, class].into_iter().collect();

    // Determine aria-invalid
    let aria_invalid_value = aria_invalid.or(Some(error)).map(|v| v.to_string());

    html! {
        <input
            ref={node_ref}
            type="checkbox"
            class={final_classes}
            checked={is_checked}
            disabled={disabled}
            required={required}
            name={name}
            value={value}
            id={id}
            onchange={handle_change}
            onfocus={onfocus}
            onblur={onblur}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            aria-invalid={aria_invalid_value}
            aria-checked={aria_checked_value(is_checked, indeterminate)}
            style={style}
        />
    }
}

/// Value for `aria-checked`: `"mixed"` while indeterminate, else the checked state.
fn aria_checked_value(checked: bool, indeterminate: bool) -> &'static str {
    match (indeterminate, checked) {
        (true, _) => "mixed",
        (false, true) => "true",
        (false, false) => "false",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aria_checked_value() {
        assert_eq!(aria_checked_value(false, false), "false");
        assert_eq!(aria_checked_value(true, false), "true");
        assert_eq!(aria_checked_value(false, true), "mixed");
        assert_eq!(aria_checked_value(true, true), "mixed");
    }

    #[test]
    fn test_checkbox_props_defaults() {
        let props = yew::props!(CheckboxProps {});
        assert_eq!(props.checked, None);
        assert!(!props.default_checked);
        assert!(!props.indeterminate);
        assert_eq!(props.size, Size::Md);
        assert!(!props.disabled);
        assert!(!props.required);
        assert!(!props.error);
        assert!(props.on_checked_change.is_none());
    }

    #[test]
    fn test_checkbox_controlled_vs_uncontrolled_props() {
        let uncontrolled = yew::props!(CheckboxProps {
            default_checked: true
        });
        assert_eq!(uncontrolled.checked, None);
        assert!(uncontrolled.default_checked);

        let controlled = yew::props!(CheckboxProps { checked: false });
        assert_eq!(controlled.checked, Some(false));
    }

    #[test]
    fn test_checkbox_states() {
        let props = yew::props!(CheckboxProps {
            checked: true,
            disabled: true,
            required: true,
            error: true,
            indeterminate: true,
        });
        assert_eq!(props.checked, Some(true));
        assert!(props.disabled);
        assert!(props.required);
        assert!(props.error);
        assert!(props.indeterminate);
    }
}
