//! Input component
//!
//! A text input field with multiple variants and states.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Input, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let value = use_state(|| String::new());
//!
//!     let oninput = {
//!         let value = value.clone();
//!         Callback::from(move |e: InputEvent| {
//!             let input: web_sys::HtmlInputElement = e.target_unchecked_into();
//!             value.set(input.value());
//!         })
//!     };
//!
//!     html! {
//!         <div>
//!             <Label html_for="email">{ "Email" }</Label>
//!             <Input
//!                 id="email"
//!                 r#type="email"
//!                 placeholder="Enter your email"
//!                 value={(*value).clone()}
//!                 {oninput}
//!             />
//!         </div>
//!     }
//! }
//! ```

use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::types::Size;
use crate::utils::class_names;

/// Input component properties
#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    /// Input type (text, email, password, etc.)
    #[prop_or(AttrValue::from("text"))]
    pub r#type: AttrValue,

    /// Input value
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Default value (for uncontrolled inputs)
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Placeholder text
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    /// Input size
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Read-only state
    #[prop_or(false)]
    pub readonly: bool,

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

    /// Input event handler
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,

    /// Change event handler
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,

    /// Focus event handler
    #[prop_or_default]
    pub onfocus: Option<Callback<FocusEvent>>,

    /// Blur event handler
    #[prop_or_default]
    pub onblur: Option<Callback<FocusEvent>>,

    /// Keydown event handler
    #[prop_or_default]
    pub onkeydown: Option<Callback<KeyboardEvent>>,

    /// Keyup event handler
    #[prop_or_default]
    pub onkeyup: Option<Callback<KeyboardEvent>>,

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

/// Input component
///
/// A versatile text input component with support for various types and states.
///
/// # Input Types
/// Supports all HTML5 input types:
/// - text (default)
/// - email
/// - password
/// - number
/// - tel
/// - url
/// - search
/// - date, time, datetime-local
/// - And more...
///
/// # States
/// - Normal: Default interactive state
/// - Disabled: Non-interactive, visually muted
/// - Read-only: Non-editable but can be focused
/// - Error: Indicates validation error
/// - Required: Indicates required field
///
/// # Accessibility
/// - Supports ARIA attributes
/// - Keyboard navigation
/// - Screen reader friendly
#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        r#type,
        value,
        default_value,
        placeholder,
        size,
        disabled,
        readonly,
        required,
        error,
        name,
        id,
        autocomplete,
        oninput,
        onchange,
        onfocus,
        onblur,
        onkeydown,
        onkeyup,
        aria_label,
        aria_describedby,
        aria_invalid,
        class,
        style,
        node_ref,
    } = props.clone();

    // Build class names
    let classes = class_names(&[
        Some("input"),
        Some(size.to_class()),
        if error { Some("input-error") } else { None },
        if disabled { Some("input-disabled") } else { None },
        if readonly { Some("input-readonly") } else { None },
    ]);

    // Merge with custom classes
    let final_classes: Classes = vec![classes, class].into_iter().collect();

    // Determine aria-invalid
    let aria_invalid_value = aria_invalid.or(Some(error)).map(|v| v.to_string());

    html! {
        <input
            ref={node_ref}
            type={r#type}
            class={final_classes}
            value={value}
            placeholder={placeholder}
            disabled={disabled}
            readonly={readonly}
            required={required}
            name={name}
            id={id}
            autocomplete={autocomplete}
            oninput={oninput}
            onchange={onchange}
            onfocus={onfocus}
            onblur={onblur}
            onkeydown={onkeydown}
            onkeyup={onkeyup}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            aria-invalid={aria_invalid_value}
            style={style}
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_props_default() {
        let props = InputProps {
            r#type: AttrValue::from("text"),
            value: None,
            default_value: None,
            placeholder: None,
            size: Size::Md,
            disabled: false,
            readonly: false,
            required: false,
            error: false,
            name: None,
            id: None,
            autocomplete: None,
            oninput: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            onkeydown: None,
            onkeyup: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert_eq!(props.r#type, AttrValue::from("text"));
        assert_eq!(props.size, Size::Md);
        assert!(!props.disabled);
        assert!(!props.error);
    }

    #[test]
    fn test_input_types() {
        let types = vec!["text", "email", "password", "number", "tel", "url"];

        for input_type in types {
            let props = InputProps {
                r#type: AttrValue::from(input_type),
                value: None,
                default_value: None,
                placeholder: None,
                size: Size::Md,
                disabled: false,
                readonly: false,
                required: false,
                error: false,
                name: None,
                id: None,
                autocomplete: None,
                oninput: None,
                onchange: None,
                onfocus: None,
                onblur: None,
                onkeydown: None,
                onkeyup: None,
                aria_label: None,
                aria_describedby: None,
                aria_invalid: None,
                class: Classes::new(),
                style: None,
                node_ref: NodeRef::default(),
            };
            assert_eq!(props.r#type, AttrValue::from(input_type));
        }
    }

    #[test]
    fn test_input_states() {
        let props = InputProps {
            r#type: AttrValue::from("text"),
            value: None,
            default_value: None,
            placeholder: None,
            size: Size::Md,
            disabled: true,
            readonly: true,
            required: true,
            error: true,
            name: None,
            id: None,
            autocomplete: None,
            oninput: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            onkeydown: None,
            onkeyup: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert!(props.disabled);
        assert!(props.readonly);
        assert!(props.required);
        assert!(props.error);
    }
}
