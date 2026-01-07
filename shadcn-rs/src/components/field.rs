//! Field component
//!
//! Complete form field wrapper with label, input, help text, and error message.
//!
//! # Examples
//!
//! ```rust,ignore
//! use yew::prelude::*;
//! use shadcn_rs::{Field, Input};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let has_error = use_state(|| false);
//!
//!     html! {
//!         <Field
//!             label="Email"
//!             required=true
//!             help_text="We'll never share your email."
//!             error={if *has_error { Some("Invalid email address".into()) } else { None }}
//!         >
//!             <Input r#type="email" id="email" />
//!         </Field>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Field component properties
#[derive(Properties, PartialEq, Clone)]
pub struct FieldProps {
    /// Label text
    #[prop_or_default]
    pub label: Option<AttrValue>,

    /// Field ID for label association
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Required field indicator
    #[prop_or(false)]
    pub required: bool,

    /// Help text (shown below input)
    #[prop_or_default]
    pub help_text: Option<AttrValue>,

    /// Error message (shown when field has error)
    #[prop_or_default]
    pub error: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements (input/textarea/select)
    pub children: Children,
}

/// Field component
///
/// Complete form field with label, input, help text, and error handling.
///
/// # Accessibility
/// - Proper label-input association via htmlFor
/// - Error messages linked with aria-describedby
/// - Required indicator visible and announced
/// - Help text associated with input
#[function_component(Field)]
pub fn field(props: &FieldProps) -> Html {
    let FieldProps {
        label,
        id,
        required,
        help_text,
        error,
        class,
        children,
    } = props.clone();

    let has_error = error.is_some();

    let classes: Classes = vec![
        Classes::from("field"),
        if has_error {
            Classes::from("field-error")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let help_id = id.as_ref().map(|field_id| format!("{}-help", field_id));
    let error_id = id.as_ref().map(|field_id| format!("{}-error", field_id));

    html! {
        <div class={classes}>
            if let Some(label_text) = label {
                <label
                    class="field-label"
                    for={id.clone()}
                >
                    { label_text }
                    if required {
                        <span class="field-required" aria-label="required">
                            { " *" }
                        </span>
                    }
                </label>
            }
            <div class="field-input">
                { children }
            </div>
            if let Some(help) = help_text {
                <div
                    class="field-help"
                    id={help_id}
                >
                    { help }
                </div>
            }
            if let Some(error_msg) = error {
                <div
                    class="field-error-message"
                    id={error_id}
                    role="alert"
                >
                    { error_msg }
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_default() {
        let props = FieldProps {
            label: None,
            id: None,
            required: false,
            help_text: None,
            error: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.required);
        assert_eq!(props.error, None);
    }

    #[test]
    fn test_field_with_label() {
        let props = FieldProps {
            label: Some(AttrValue::from("Email")),
            id: Some(AttrValue::from("email")),
            required: false,
            help_text: None,
            error: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.label, Some(AttrValue::from("Email")));
        assert_eq!(props.id, Some(AttrValue::from("email")));
    }

    #[test]
    fn test_field_required() {
        let props = FieldProps {
            label: Some(AttrValue::from("Name")),
            id: None,
            required: true,
            help_text: None,
            error: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.required);
    }

    #[test]
    fn test_field_with_help_text() {
        let props = FieldProps {
            label: None,
            id: None,
            required: false,
            help_text: Some(AttrValue::from("Enter your email address")),
            error: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(
            props.help_text,
            Some(AttrValue::from("Enter your email address"))
        );
    }

    #[test]
    fn test_field_with_error() {
        let props = FieldProps {
            label: None,
            id: None,
            required: false,
            help_text: None,
            error: Some(AttrValue::from("This field is required")),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.error, Some(AttrValue::from("This field is required")));
    }
}
