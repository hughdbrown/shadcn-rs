//! Separator component
//!
//! Visually or semantically separates content.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::Separator;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <>
//!             <div>{ "Content above" }</div>
//!             <Separator />
//!             <div>{ "Content below" }</div>
//!         </>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Separator orientation
#[derive(Debug, Clone, PartialEq)]
pub enum SeparatorOrientation {
    /// Horizontal separator (default)
    Horizontal,
    /// Vertical separator
    Vertical,
}

impl SeparatorOrientation {
    /// Convert to CSS class
    pub fn to_class(&self) -> &'static str {
        match self {
            SeparatorOrientation::Horizontal => "separator-horizontal",
            SeparatorOrientation::Vertical => "separator-vertical",
        }
    }

    /// Get ARIA orientation value
    pub fn to_aria(&self) -> &'static str {
        match self {
            SeparatorOrientation::Horizontal => "horizontal",
            SeparatorOrientation::Vertical => "vertical",
        }
    }
}

/// Separator component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SeparatorProps {
    /// Separator orientation
    #[prop_or(SeparatorOrientation::Horizontal)]
    pub orientation: SeparatorOrientation,

    /// Whether the separator is decorative (no semantic meaning)
    /// If decorative, will use role="none", otherwise role="separator"
    #[prop_or(true)]
    pub decorative: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

/// Separator component
///
/// A thin line that groups or divides content.
///
/// # Orientation
/// - `Horizontal`: Divides content vertically (default)
/// - `Vertical`: Divides content horizontally
///
/// # Accessibility
/// - Decorative separators use `role="none"` and are hidden from screen readers
/// - Semantic separators use `role="separator"` with appropriate ARIA attributes
#[function_component(Separator)]
pub fn separator(props: &SeparatorProps) -> Html {
    let SeparatorProps {
        orientation,
        decorative,
        class,
        style,
        id,
    } = props.clone();

    // Build class names
    let classes: Classes = vec![
        Classes::from("separator"),
        Classes::from(orientation.to_class()),
        class,
    ]
    .into_iter()
    .collect();

    let role = if decorative {
        Some(AttrValue::from("none"))
    } else {
        Some(AttrValue::from("separator"))
    };

    let aria_orientation = if !decorative {
        Some(AttrValue::from(orientation.to_aria()))
    } else {
        None
    };

    html! {
        <div
            class={classes}
            style={style}
            id={id}
            role={role}
            aria-orientation={aria_orientation}
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separator_orientation_class() {
        assert_eq!(
            SeparatorOrientation::Horizontal.to_class(),
            "separator-horizontal"
        );
        assert_eq!(
            SeparatorOrientation::Vertical.to_class(),
            "separator-vertical"
        );
    }

    #[test]
    fn test_separator_orientation_aria() {
        assert_eq!(SeparatorOrientation::Horizontal.to_aria(), "horizontal");
        assert_eq!(SeparatorOrientation::Vertical.to_aria(), "vertical");
    }

    #[test]
    fn test_separator_props_default() {
        let props = SeparatorProps {
            orientation: SeparatorOrientation::Horizontal,
            decorative: true,
            class: Classes::new(),
            style: None,
            id: None,
        };

        assert_eq!(props.orientation, SeparatorOrientation::Horizontal);
        assert!(props.decorative);
    }

    #[test]
    fn test_separator_vertical() {
        let props = SeparatorProps {
            orientation: SeparatorOrientation::Vertical,
            decorative: false,
            class: Classes::new(),
            style: None,
            id: None,
        };

        assert_eq!(props.orientation, SeparatorOrientation::Vertical);
        assert!(!props.decorative);
    }

    #[test]
    fn test_separator_decorative_vs_semantic() {
        let decorative = SeparatorProps {
            orientation: SeparatorOrientation::Horizontal,
            decorative: true,
            class: Classes::new(),
            style: None,
            id: None,
        };

        let semantic = SeparatorProps {
            orientation: SeparatorOrientation::Horizontal,
            decorative: false,
            class: Classes::new(),
            style: None,
            id: None,
        };

        assert!(decorative.decorative);
        assert!(!semantic.decorative);
    }
}
