//! Badge component
//!
//! Displays a small label or status indicator.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Badge, Variant};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <>
//!             <Badge variant={Variant::Default}>{ "Default" }</Badge>
//!             <Badge variant={Variant::Primary}>{ "Primary" }</Badge>
//!             <Badge variant={Variant::Destructive}>{ "Destructive" }</Badge>
//!             <Badge variant={Variant::Outline}>{ "Outline" }</Badge>
//!         </>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::types::Variant;
use crate::utils::class_names;

/// Badge component properties
#[derive(Properties, PartialEq, Clone)]
pub struct BadgeProps {
    /// Badge variant style
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

    /// Children elements
    pub children: Children,
}

/// Badge component
///
/// A small label component for displaying status, categories, or counts.
///
/// # Variants
/// - `Default`: Standard badge
/// - `Primary`: Primary colored badge
/// - `Secondary`: Secondary colored badge
/// - `Destructive`: Destructive/error badge
/// - `Outline`: Outlined badge
///
/// # Usage
/// Badges are commonly used for:
/// - Status indicators
/// - Category labels
/// - Notification counts
/// - Tags
#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let BadgeProps {
        variant,
        class,
        style,
        id,
        children,
    } = props.clone();

    // Build class names
    let classes = class_names(&[
        Some("badge"),
        Some(variant.to_class()),
    ]);

    // Merge with custom classes
    let final_classes: Classes = vec![classes, class].into_iter().collect();

    html! {
        <span class={final_classes} style={style} id={id}>
            { children }
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_props_default() {
        let props = BadgeProps {
            variant: Variant::Default,
            class: Classes::new(),
            style: None,
            id: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.variant, Variant::Default);
        assert_eq!(props.class, Classes::new());
    }

    #[test]
    fn test_badge_variants() {
        let variants = vec![
            Variant::Default,
            Variant::Primary,
            Variant::Secondary,
            Variant::Destructive,
            Variant::Outline,
        ];

        for variant in variants {
            let props = BadgeProps {
                variant: variant.clone(),
                class: Classes::new(),
                style: None,
                id: None,
                children: Children::new(vec![]),
            };
            assert_eq!(props.variant, variant);
        }
    }

    #[test]
    fn test_badge_with_custom_class() {
        let custom_class = Classes::from("custom-badge");
        let props = BadgeProps {
            variant: Variant::Primary,
            class: custom_class.clone(),
            style: None,
            id: None,
            children: Children::new(vec![]),
        };

        assert!(props.class.contains("custom-badge"));
    }
}
