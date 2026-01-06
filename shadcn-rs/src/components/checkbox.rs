//! Checkbox component
//!
//! A control that allows the user to toggle between checked and not checked.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Checkbox, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let checked = use_state(|| false);
//!
//!     let onchange = {
//!         let checked = checked.clone();
//!         Callback::from(move |e: Event| {
//!             let input: web_sys::HtmlInputElement = e.target_unchecked_into();
//!             checked.set(input.checked());
//!         })
//!     };
//!
//!     html! {
//!         <div class="flex items-center space-x-2">
//!             <Checkbox id="terms" checked={*checked} {onchange} />
//!             <Label html_for="terms">{ "Accept terms and conditions" }</Label>
//!         </div>
//!     }
//! }
//! ```

use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::types::Size;
use crate::utils::class_names;

/// Checkbox component properties
#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    /// Checked state
    #[prop_or(false)]
    pub checked: bool,

    /// Default checked state (for uncontrolled checkboxes)
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

    /// Change event handler
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,

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
/// # Accessibility
/// - Supports ARIA attributes
/// - Keyboard navigation (Space to toggle)
/// - Screen reader friendly
/// - Indeterminate state announced properly
#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        checked,
        default_checked: _,
        indeterminate,
        size,
        disabled,
        required,
        error,
        name,
        value,
        id,
        onchange,
        onfocus,
        onblur,
        aria_label,
        aria_describedby,
        aria_invalid,
        class,
        style,
        node_ref,
    } = props.clone();

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

    // Build class names
    let classes = class_names(&[
        Some("checkbox"),
        Some(size.to_class()),
        if error { Some("checkbox-error") } else { None },
        if disabled { Some("checkbox-disabled") } else { None },
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

    // Determine aria-checked
    let aria_checked = if indeterminate {
        Some(AttrValue::from("mixed"))
    } else {
        Some(AttrValue::from(checked.to_string()))
    };

    html! {
        <input
            ref={node_ref}
            type="checkbox"
            class={final_classes}
            checked={checked}
            disabled={disabled}
            required={required}
            name={name}
            value={value}
            id={id}
            onchange={onchange}
            onfocus={onfocus}
            onblur={onblur}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            aria-invalid={aria_invalid_value}
            aria-checked={aria_checked}
            style={style}
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_props_default() {
        let props = CheckboxProps {
            checked: false,
            default_checked: false,
            indeterminate: false,
            size: Size::Md,
            disabled: false,
            required: false,
            error: false,
            name: None,
            value: None,
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert!(!props.checked);
        assert!(!props.indeterminate);
        assert_eq!(props.size, Size::Md);
        assert!(!props.disabled);
    }

    #[test]
    fn test_checkbox_checked() {
        let props = CheckboxProps {
            checked: true,
            default_checked: false,
            indeterminate: false,
            size: Size::Md,
            disabled: false,
            required: false,
            error: false,
            name: None,
            value: None,
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert!(props.checked);
    }

    #[test]
    fn test_checkbox_indeterminate() {
        let props = CheckboxProps {
            checked: false,
            default_checked: false,
            indeterminate: true,
            size: Size::Md,
            disabled: false,
            required: false,
            error: false,
            name: None,
            value: None,
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert!(props.indeterminate);
    }

    #[test]
    fn test_checkbox_states() {
        let props = CheckboxProps {
            checked: true,
            default_checked: false,
            indeterminate: false,
            size: Size::Md,
            disabled: true,
            required: true,
            error: true,
            name: None,
            value: None,
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert!(props.disabled);
        assert!(props.required);
        assert!(props.error);
    }
}
