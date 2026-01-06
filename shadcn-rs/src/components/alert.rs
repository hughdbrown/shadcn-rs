//! Alert component
//!
//! Displays a callout for user attention.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Alert, AlertTitle, AlertDescription, Variant};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Alert variant={Variant::Default}>
//!             <AlertTitle>{ "Heads up!" }</AlertTitle>
//!             <AlertDescription>
//!                 { "You can add components to your app using the cli." }
//!             </AlertDescription>
//!         </Alert>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::types::Variant;
use crate::utils::class_names;

/// Alert component properties
#[derive(Properties, PartialEq, Clone)]
pub struct AlertProps {
    /// Alert variant style
    #[prop_or(Variant::Default)]
    pub variant: Variant,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// ARIA role (default: "alert")
    #[prop_or(AttrValue::from("alert"))]
    pub role: AttrValue,

    /// Children elements
    pub children: Children,
}

/// Alert component
///
/// A prominent message container for important information, warnings, or errors.
///
/// # Variants
/// - `Default`: Standard informational alert
/// - `Destructive`: Error or critical alert
/// - `Primary`: Primary information alert
/// - `Secondary`: Secondary information alert
///
/// # Accessibility
/// - Uses `role="alert"` by default for screen readers
/// - Supports custom ARIA roles
#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let AlertProps {
        variant,
        class,
        style,
        id,
        role,
        children,
    } = props.clone();

    // Build class names
    let classes = class_names(&[
        Some("alert"),
        Some(variant.to_class()),
    ]);

    // Merge with custom classes
    let final_classes: Classes = vec![classes, class].into_iter().collect();

    html! {
        <div class={final_classes} style={style} id={id} role={role}>
            { children }
        </div>
    }
}

/// Alert title properties
#[derive(Properties, PartialEq, Clone)]
pub struct AlertTitleProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Alert title component
///
/// Displays the main heading for an alert.
#[function_component(AlertTitle)]
pub fn alert_title(props: &AlertTitleProps) -> Html {
    let AlertTitleProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("alert-title"), class]
        .into_iter()
        .collect();

    html! {
        <h5 class={classes}>
            { children }
        </h5>
    }
}

/// Alert description properties
#[derive(Properties, PartialEq, Clone)]
pub struct AlertDescriptionProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Alert description component
///
/// Displays the descriptive content for an alert.
#[function_component(AlertDescription)]
pub fn alert_description(props: &AlertDescriptionProps) -> Html {
    let AlertDescriptionProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("alert-description"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_props_default() {
        let props = AlertProps {
            variant: Variant::Default,
            class: Classes::new(),
            style: None,
            id: None,
            role: AttrValue::from("alert"),
            children: Children::new(vec![]),
        };

        assert_eq!(props.variant, Variant::Default);
        assert_eq!(props.role, AttrValue::from("alert"));
    }

    #[test]
    fn test_alert_variants() {
        let variants = vec![
            Variant::Default,
            Variant::Primary,
            Variant::Secondary,
            Variant::Destructive,
        ];

        for variant in variants {
            let props = AlertProps {
                variant: variant.clone(),
                class: Classes::new(),
                style: None,
                id: None,
                role: AttrValue::from("alert"),
                children: Children::new(vec![]),
            };
            assert_eq!(props.variant, variant);
        }
    }

    #[test]
    fn test_alert_custom_role() {
        let props = AlertProps {
            variant: Variant::Default,
            class: Classes::new(),
            style: None,
            id: None,
            role: AttrValue::from("status"),
            children: Children::new(vec![]),
        };

        assert_eq!(props.role, AttrValue::from("status"));
    }

    #[test]
    fn test_alert_title_props() {
        let props = AlertTitleProps {
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.class, Classes::new());
    }

    #[test]
    fn test_alert_description_props() {
        let props = AlertDescriptionProps {
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.class, Classes::new());
    }
}
