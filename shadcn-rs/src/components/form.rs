//! Form component
//!
//! A form container with validation and submission handling.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Form, FormItem, FormLabel, FormControl, FormDescription, FormMessage, FormMessageType, Input, Button};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let has_error = use_state(|| false);
//!
//!     let onsubmit = Callback::from(|e: SubmitEvent| {
//!         e.prevent_default();
//!         web_sys::console::log_1(&"Form submitted!".into());
//!     });
//!
//!     html! {
//!         <Form {onsubmit}>
//!             <FormItem>
//!                 <FormLabel html_for="email" required=true>{ "Email" }</FormLabel>
//!                 <FormControl error={*has_error}>
//!                     <Input id="email" r#type="email" required=true />
//!                 </FormControl>
//!                 <FormDescription id="email-desc">
//!                     { "We'll never share your email with anyone." }
//!                 </FormDescription>
//!                 if *has_error {
//!                     <FormMessage message_type={FormMessageType::Error}>
//!                         { "Please enter a valid email address." }
//!                     </FormMessage>
//!                 }
//!             </FormItem>
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

/// Form item properties
#[derive(Properties, PartialEq, Clone)]
pub struct FormItemProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Children elements
    pub children: Children,
}

/// Form item component
///
/// A wrapper for form field elements providing consistent spacing and layout.
///
/// # Usage
/// Use FormItem to wrap a complete form field including label, control, description, and message.
#[function_component(FormItem)]
pub fn form_item(props: &FormItemProps) -> Html {
    let FormItemProps {
        class,
        style,
        id,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("form-item"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} style={style} id={id}>
            { children }
        </div>
    }
}

/// Form label properties
#[derive(Properties, PartialEq, Clone)]
pub struct FormLabelProps {
    /// ID of the form control this label is for
    #[prop_or_default]
    pub html_for: Option<AttrValue>,

    /// Whether the associated field is required
    #[prop_or(false)]
    pub required: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Children elements
    pub children: Children,
}

/// Form label component
///
/// A label specifically designed for form fields with consistent styling.
///
/// # Usage
/// Use FormLabel instead of regular Label for form fields to ensure consistent styling.
#[function_component(FormLabel)]
pub fn form_label(props: &FormLabelProps) -> Html {
    let FormLabelProps {
        html_for,
        required,
        class,
        id,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("form-label"),
        if required {
            Classes::from("form-label-required")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <label
            class={classes}
            for={html_for}
            id={id}
        >
            { children }
            if required {
                <span class="form-label-required-indicator" aria-hidden="true">{ " *" }</span>
            }
        </label>
    }
}

/// Form control properties
#[derive(Properties, PartialEq, Clone)]
pub struct FormControlProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// Whether the control has an error
    #[prop_or(false)]
    pub error: bool,

    /// Children elements
    pub children: Children,
}

/// Form control component
///
/// A wrapper for form input elements that provides consistent styling and error states.
///
/// # Usage
/// Wrap your input components with FormControl to apply form-specific styling.
#[function_component(FormControl)]
pub fn form_control(props: &FormControlProps) -> Html {
    let FormControlProps {
        class,
        style,
        error,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("form-control"),
        if error {
            Classes::from("form-control-error")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div class={classes} style={style}>
            { children }
        </div>
    }
}

/// Form description properties
#[derive(Properties, PartialEq, Clone)]
pub struct FormDescriptionProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// ID attribute (used for aria-describedby)
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Children elements
    pub children: Children,
}

/// Form description component
///
/// Displays helpful text or instructions for a form field.
///
/// # Usage
/// Use FormDescription to provide additional context or instructions for form fields.
#[function_component(FormDescription)]
pub fn form_description(props: &FormDescriptionProps) -> Html {
    let FormDescriptionProps {
        class,
        id,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("form-description"), class]
        .into_iter()
        .collect();

    html! {
        <p class={classes} id={id}>
            { children }
        </p>
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

    #[test]
    fn test_form_item_props() {
        let props = FormItemProps {
            class: Classes::new(),
            style: None,
            id: Some(AttrValue::from("item-1")),
            children: Children::new(vec![]),
        };

        assert_eq!(props.id, Some(AttrValue::from("item-1")));
    }

    #[test]
    fn test_form_label_required() {
        let props = FormLabelProps {
            html_for: Some(AttrValue::from("input-1")),
            required: true,
            class: Classes::new(),
            id: None,
            children: Children::new(vec![]),
        };

        assert!(props.required);
        assert_eq!(props.html_for, Some(AttrValue::from("input-1")));
    }

    #[test]
    fn test_form_label_optional() {
        let props = FormLabelProps {
            html_for: None,
            required: false,
            class: Classes::new(),
            id: None,
            children: Children::new(vec![]),
        };

        assert!(!props.required);
    }

    #[test]
    fn test_form_control_error_state() {
        let props = FormControlProps {
            class: Classes::new(),
            style: None,
            error: true,
            children: Children::new(vec![]),
        };

        assert!(props.error);
    }

    #[test]
    fn test_form_control_normal_state() {
        let props = FormControlProps {
            class: Classes::new(),
            style: None,
            error: false,
            children: Children::new(vec![]),
        };

        assert!(!props.error);
    }

    #[test]
    fn test_form_description_with_id() {
        let props = FormDescriptionProps {
            class: Classes::new(),
            id: Some(AttrValue::from("desc-1")),
            children: Children::new(vec![]),
        };

        assert_eq!(props.id, Some(AttrValue::from("desc-1")));
    }
}
