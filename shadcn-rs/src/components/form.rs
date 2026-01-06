//! Form component
//!
//! A form container with validation and submission handling.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Form, FormField, Label, Input, Button};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let onsubmit = Callback::from(|e: SubmitEvent| {
//!         e.prevent_default();
//!         web_sys::console::log_1(&"Form submitted!".into());
//!     });
//!
//!     html! {
//!         <Form {onsubmit}>
//!             <FormField>
//!                 <Label html_for="email">{ "Email" }</Label>
//!                 <Input id="email" r#type="email" required=true />
//!             </FormField>
//!             <Button r#type="submit">{ "Submit" }</Button>
//!         </Form>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Form component properties
#[derive(Properties, PartialEq, Clone)]
pub struct FormProps {
    /// Form submit event handler
    #[prop_or_default]
    pub onsubmit: Option<Callback<SubmitEvent>>,

    /// Form reset event handler
    #[prop_or_default]
    pub onreset: Option<Callback<Event>>,

    /// Form action URL
    #[prop_or_default]
    pub action: Option<AttrValue>,

    /// Form method (GET or POST)
    #[prop_or_default]
    pub method: Option<AttrValue>,

    /// Form encoding type
    #[prop_or_default]
    pub enctype: Option<AttrValue>,

    /// Form target
    #[prop_or_default]
    pub target: Option<AttrValue>,

    /// Disable native validation
    #[prop_or(false)]
    pub novalidate: bool,

    /// Autocomplete behavior
    #[prop_or_default]
    pub autocomplete: Option<AttrValue>,

    /// Name attribute
    #[prop_or_default]
    pub name: Option<AttrValue>,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// Node ref
    #[prop_or_default]
    pub node_ref: NodeRef,

    /// Children elements
    pub children: Children,
}

/// Form component
///
/// A form container for grouping and managing form inputs.
///
/// # Features
/// - Form submission handling
/// - Form reset handling
/// - Native validation support
/// - Customizable submission behavior
///
/// # Accessibility
/// - Semantic HTML form element
/// - Supports all standard form attributes
/// - Screen reader friendly
#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let FormProps {
        onsubmit,
        onreset,
        action,
        method,
        enctype,
        target,
        novalidate,
        autocomplete,
        name,
        id,
        class,
        style,
        node_ref,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("form"), class].into_iter().collect();

    html! {
        <form
            ref={node_ref}
            class={classes}
            onsubmit={onsubmit}
            onreset={onreset}
            action={action}
            method={method}
            enctype={enctype}
            target={target}
            novalidate={novalidate}
            autocomplete={autocomplete}
            name={name}
            id={id}
            style={style}
        >
            { children }
        </form>
    }
}

/// Form field properties
#[derive(Properties, PartialEq, Clone)]
pub struct FormFieldProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// Children elements
    pub children: Children,
}

/// Form field component
///
/// A container for grouping a label and input together.
///
/// # Usage
/// Use FormField to group related form elements (label + input + error message).
#[function_component(FormField)]
pub fn form_field(props: &FormFieldProps) -> Html {
    let FormFieldProps {
        class,
        style,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("form-field"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} style={style}>
            { children }
        </div>
    }
}

/// Form message properties (for errors, hints, etc.)
#[derive(Properties, PartialEq, Clone)]
pub struct FormMessageProps {
    /// Message type
    #[prop_or(FormMessageType::Info)]
    pub message_type: FormMessageType,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Children elements
    pub children: Children,
}

/// Form message type
#[derive(Debug, Clone, PartialEq)]
pub enum FormMessageType {
    /// Informational message
    Info,
    /// Error message
    Error,
    /// Warning message
    Warning,
    /// Success message
    Success,
}

impl FormMessageType {
    /// Convert to CSS class
    pub fn to_class(&self) -> &'static str {
        match self {
            FormMessageType::Info => "form-message-info",
            FormMessageType::Error => "form-message-error",
            FormMessageType::Warning => "form-message-warning",
            FormMessageType::Success => "form-message-success",
        }
    }

    /// Get ARIA role
    pub fn to_role(&self) -> &'static str {
        match self {
            FormMessageType::Error => "alert",
            FormMessageType::Warning => "alert",
            FormMessageType::Success => "status",
            FormMessageType::Info => "status",
        }
    }
}

/// Form message component
///
/// Displays validation messages, hints, or other form-related messages.
///
/// # Usage
/// Use FormMessage to display error messages, hints, or success feedback.
#[function_component(FormMessage)]
pub fn form_message(props: &FormMessageProps) -> Html {
    let FormMessageProps {
        message_type,
        class,
        id,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("form-message"),
        Classes::from(message_type.to_class()),
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <p
            class={classes}
            id={id}
            role={message_type.to_role()}
            aria-live="polite"
        >
            { children }
        </p>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_props_default() {
        let props = FormProps {
            onsubmit: None,
            onreset: None,
            action: None,
            method: None,
            enctype: None,
            target: None,
            novalidate: false,
            autocomplete: None,
            name: None,
            id: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert!(!props.novalidate);
        assert!(props.action.is_none());
    }

    #[test]
    fn test_form_with_action() {
        let props = FormProps {
            onsubmit: None,
            onreset: None,
            action: Some(AttrValue::from("/submit")),
            method: Some(AttrValue::from("POST")),
            enctype: None,
            target: None,
            novalidate: false,
            autocomplete: None,
            name: None,
            id: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.action, Some(AttrValue::from("/submit")));
        assert_eq!(props.method, Some(AttrValue::from("POST")));
    }

    #[test]
    fn test_form_novalidate() {
        let props = FormProps {
            onsubmit: None,
            onreset: None,
            action: None,
            method: None,
            enctype: None,
            target: None,
            novalidate: true,
            autocomplete: None,
            name: None,
            id: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert!(props.novalidate);
    }

    #[test]
    fn test_form_message_type_class() {
        assert_eq!(FormMessageType::Info.to_class(), "form-message-info");
        assert_eq!(FormMessageType::Error.to_class(), "form-message-error");
        assert_eq!(FormMessageType::Warning.to_class(), "form-message-warning");
        assert_eq!(FormMessageType::Success.to_class(), "form-message-success");
    }

    #[test]
    fn test_form_message_type_role() {
        assert_eq!(FormMessageType::Error.to_role(), "alert");
        assert_eq!(FormMessageType::Warning.to_role(), "alert");
        assert_eq!(FormMessageType::Success.to_role(), "status");
        assert_eq!(FormMessageType::Info.to_role(), "status");
    }

    #[test]
    fn test_form_field_props() {
        let props = FormFieldProps {
            class: Classes::new(),
            style: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.class, Classes::new());
    }

    #[test]
    fn test_form_message_props() {
        let props = FormMessageProps {
            message_type: FormMessageType::Error,
            class: Classes::new(),
            id: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.message_type, FormMessageType::Error);
    }
}
