//! Label component
//!
//! Renders an accessible label associated with form controls.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Label, Input};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <div>
//!             <Label html_for="email">{ "Email" }</Label>
//!             <Input id="email" r#type="email" />
//!         </div>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Label component properties
#[derive(Properties, PartialEq, Clone)]
pub struct LabelProps {
    /// ID of the form control this label is for
    #[prop_or_default]
    pub html_for: Option<AttrValue>,

    /// Whether the associated field is required
    #[prop_or(false)]
    pub required: bool,

    /// Whether the associated field is disabled
    #[prop_or(false)]
    pub disabled: bool,

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

/// Label component
///
/// Renders an accessible label for form controls.
///
/// # Accessibility
/// - Associates with form controls via `html_for` (maps to `for` attribute)
/// - Supports required and disabled states
/// - Screen reader friendly
///
/// # Usage
/// Labels should be associated with form controls for accessibility.
/// Use the `html_for` prop to link the label to a control's ID.
#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    let LabelProps {
        html_for,
        required,
        disabled,
        class,
        style,
        id,
        children,
    } = props.clone();

    // Build class names
    let classes: Classes = vec![
        Classes::from("label"),
        if disabled {
            Classes::from("label-disabled")
        } else {
            Classes::new()
        },
        if required {
            Classes::from("label-required")
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
            style={style}
            id={id}
        >
            { children }
            if required {
                <span class="label-required-indicator" aria-hidden="true">{ " *" }</span>
            }
        </label>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_props_default() {
        let props = LabelProps {
            html_for: None,
            required: false,
            disabled: false,
            class: Classes::new(),
            style: None,
            id: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.html_for, None);
        assert!(!props.required);
        assert!(!props.disabled);
    }

    #[test]
    fn test_label_with_for() {
        let props = LabelProps {
            html_for: Some(AttrValue::from("input-id")),
            required: false,
            disabled: false,
            class: Classes::new(),
            style: None,
            id: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.html_for, Some(AttrValue::from("input-id")));
    }

    #[test]
    fn test_label_required() {
        let props = LabelProps {
            html_for: None,
            required: true,
            disabled: false,
            class: Classes::new(),
            style: None,
            id: None,
            children: Children::new(vec![]),
        };

        assert!(props.required);
    }

    #[test]
    fn test_label_disabled() {
        let props = LabelProps {
            html_for: None,
            required: false,
            disabled: true,
            class: Classes::new(),
            style: None,
            id: None,
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }
}
