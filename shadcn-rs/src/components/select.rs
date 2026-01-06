//! Select component
//!
//! A dropdown select input for choosing from a list of options.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Select, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let selected = use_state(|| String::from(""));
//!
//!     let onchange = {
//!         let selected = selected.clone();
//!         Callback::from(move |e: Event| {
//!             let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
//!             selected.set(select.value());
//!         })
//!     };
//!
//!     html! {
//!         <div>
//!             <Label html_for="fruit">{ "Select a fruit" }</Label>
//!             <Select id="fruit" value={(*selected).clone()} {onchange}>
//!                 <option value="">{ "Choose..." }</option>
//!                 <option value="apple">{ "Apple" }</option>
//!                 <option value="banana">{ "Banana" }</option>
//!                 <option value="orange">{ "Orange" }</option>
//!             </Select>
//!         </div>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::types::Size;
use crate::utils::class_names;

/// Select component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps {
    /// Selected value
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Default value (for uncontrolled selects)
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Placeholder option (shown when no value selected)
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    /// Select size
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Allow multiple selections
    #[prop_or(false)]
    pub multiple: bool,

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

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Autocomplete attribute
    #[prop_or_default]
    pub autocomplete: Option<AttrValue>,

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

    /// Children elements (option and optgroup elements)
    pub children: Children,
}

/// Select component
///
/// A dropdown select component for choosing from a list of options.
///
/// # Features
/// - Single and multiple selection modes
/// - Grouped options (via optgroup)
/// - Keyboard navigation
/// - Accessibility support
///
/// # States
/// - Normal: Default interactive state
/// - Disabled: Non-interactive
/// - Error: Indicates validation error
/// - Required: Indicates required field
///
/// # Accessibility
/// - Supports ARIA attributes
/// - Keyboard navigation (Arrow keys, Enter, Escape)
/// - Screen reader friendly
#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let SelectProps {
        value,
        default_value: _,
        placeholder,
        size,
        multiple,
        disabled,
        required,
        error,
        name,
        id,
        autocomplete,
        onchange,
        onfocus,
        onblur,
        aria_label,
        aria_describedby,
        aria_invalid,
        class,
        style,
        node_ref,
        children,
    } = props.clone();

    // Build class names
    let classes = class_names(&[
        Some("select"),
        Some(size.to_class()),
        if error { Some("select-error") } else { None },
        if disabled { Some("select-disabled") } else { None },
    ]);

    // Merge with custom classes
    let final_classes: Classes = vec![classes, class].into_iter().collect();

    // Determine aria-invalid
    let aria_invalid_value = aria_invalid.or(Some(error)).map(|v| v.to_string());

    let has_value = value.is_some();

    html! {
        <select
            ref={node_ref}
            class={final_classes}
            value={value}
            multiple={multiple}
            disabled={disabled}
            required={required}
            name={name}
            id={id}
            autocomplete={autocomplete}
            onchange={onchange}
            onfocus={onfocus}
            onblur={onblur}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            aria-invalid={aria_invalid_value}
            style={style}
        >
            if let Some(placeholder_text) = placeholder {
                <option value="" disabled=true selected={!has_value}>
                    { placeholder_text }
                </option>
            }
            { children }
        </select>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_props_default() {
        let props = SelectProps {
            value: None,
            default_value: None,
            placeholder: None,
            size: Size::Md,
            multiple: false,
            disabled: false,
            required: false,
            error: false,
            name: None,
            id: None,
            autocomplete: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.size, Size::Md);
        assert!(!props.multiple);
        assert!(!props.disabled);
        assert!(!props.error);
    }

    #[test]
    fn test_select_with_value() {
        let props = SelectProps {
            value: Some(AttrValue::from("option1")),
            default_value: None,
            placeholder: None,
            size: Size::Md,
            multiple: false,
            disabled: false,
            required: false,
            error: false,
            name: None,
            id: None,
            autocomplete: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.value, Some(AttrValue::from("option1")));
    }

    #[test]
    fn test_select_multiple() {
        let props = SelectProps {
            value: None,
            default_value: None,
            placeholder: None,
            size: Size::Md,
            multiple: true,
            disabled: false,
            required: false,
            error: false,
            name: None,
            id: None,
            autocomplete: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert!(props.multiple);
    }

    #[test]
    fn test_select_with_placeholder() {
        let props = SelectProps {
            value: None,
            default_value: None,
            placeholder: Some(AttrValue::from("Choose an option")),
            size: Size::Md,
            multiple: false,
            disabled: false,
            required: false,
            error: false,
            name: None,
            id: None,
            autocomplete: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert_eq!(
            props.placeholder,
            Some(AttrValue::from("Choose an option"))
        );
    }

    #[test]
    fn test_select_states() {
        let props = SelectProps {
            value: None,
            default_value: None,
            placeholder: None,
            size: Size::Md,
            multiple: false,
            disabled: true,
            required: true,
            error: true,
            name: None,
            id: None,
            autocomplete: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
        assert!(props.required);
        assert!(props.error);
    }
}
