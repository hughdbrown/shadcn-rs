//! Skeleton component
//!
//! A placeholder for content that is loading.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::Skeleton;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <div class="space-y-2">
//!             <Skeleton width="250px" height="20px" />
//!             <Skeleton width="200px" height="20px" />
//!             <Skeleton width="150px" height="20px" />
//!         </div>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Skeleton shape variant
#[derive(Debug, Clone, PartialEq)]
pub enum SkeletonShape {
    /// Rectangle skeleton (default)
    Rectangle,
    /// Circle skeleton
    Circle,
    /// Text line skeleton
    Text,
}

impl SkeletonShape {
    /// Convert to CSS class
    pub fn to_class(&self) -> &'static str {
        match self {
            SkeletonShape::Rectangle => "skeleton-rectangle",
            SkeletonShape::Circle => "skeleton-circle",
            SkeletonShape::Text => "skeleton-text",
        }
    }
}

/// Skeleton component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SkeletonProps {
    /// Width of skeleton (CSS value)
    #[prop_or_default]
    pub width: Option<AttrValue>,

    /// Height of skeleton (CSS value)
    #[prop_or_default]
    pub height: Option<AttrValue>,

    /// Shape variant
    #[prop_or(SkeletonShape::Rectangle)]
    pub shape: SkeletonShape,

    /// Whether to animate the skeleton
    #[prop_or(true)]
    pub animate: bool,

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

/// Skeleton component
///
/// A loading placeholder that shows where content will eventually appear.
///
/// # Shape Variants
/// - `Rectangle`: Standard rectangular skeleton (default)
/// - `Circle`: Circular skeleton (useful for avatars)
/// - `Text`: Text line skeleton with appropriate height
///
/// # Animation
/// By default, skeletons pulse to indicate loading. This can be disabled
/// by setting `animate={false}`.
///
/// # Accessibility
/// - Uses `aria-busy="true"` to indicate loading state
/// - Uses `aria-live="polite"` for screen reader announcements
#[function_component(Skeleton)]
pub fn skeleton(props: &SkeletonProps) -> Html {
    let SkeletonProps {
        width,
        height,
        shape,
        animate,
        class,
        style,
        id,
    } = props.clone();

    // Build class names
    let classes: Classes = vec![
        Classes::from("skeleton"),
        Classes::from(shape.to_class()),
        if animate {
            Classes::from("skeleton-animate")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    // Build inline styles
    let inline_style = {
        let mut styles = vec![];
        if let Some(w) = width {
            styles.push(format!("width: {}", w));
        }
        if let Some(h) = height {
            styles.push(format!("height: {}", h));
        }
        if let Some(s) = style {
            styles.push(s.to_string());
        }

        if styles.is_empty() {
            None
        } else {
            Some(AttrValue::from(styles.join("; ")))
        }
    };

    html! {
        <div
            class={classes}
            style={inline_style}
            id={id}
            aria-busy="true"
            aria-live="polite"
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_shape_class() {
        assert_eq!(SkeletonShape::Rectangle.to_class(), "skeleton-rectangle");
        assert_eq!(SkeletonShape::Circle.to_class(), "skeleton-circle");
        assert_eq!(SkeletonShape::Text.to_class(), "skeleton-text");
    }

    #[test]
    fn test_skeleton_props_default() {
        let props = SkeletonProps {
            width: None,
            height: None,
            shape: SkeletonShape::Rectangle,
            animate: true,
            class: Classes::new(),
            style: None,
            id: None,
        };

        assert_eq!(props.shape, SkeletonShape::Rectangle);
        assert!(props.animate);
        assert!(props.width.is_none());
        assert!(props.height.is_none());
    }

    #[test]
    fn test_skeleton_with_dimensions() {
        let props = SkeletonProps {
            width: Some(AttrValue::from("100px")),
            height: Some(AttrValue::from("20px")),
            shape: SkeletonShape::Rectangle,
            animate: true,
            class: Classes::new(),
            style: None,
            id: None,
        };

        assert_eq!(props.width, Some(AttrValue::from("100px")));
        assert_eq!(props.height, Some(AttrValue::from("20px")));
    }

    #[test]
    fn test_skeleton_circle_shape() {
        let props = SkeletonProps {
            width: Some(AttrValue::from("40px")),
            height: Some(AttrValue::from("40px")),
            shape: SkeletonShape::Circle,
            animate: true,
            class: Classes::new(),
            style: None,
            id: None,
        };

        assert_eq!(props.shape, SkeletonShape::Circle);
    }

    #[test]
    fn test_skeleton_no_animation() {
        let props = SkeletonProps {
            width: None,
            height: None,
            shape: SkeletonShape::Rectangle,
            animate: false,
            class: Classes::new(),
            style: None,
            id: None,
        };

        assert!(!props.animate);
    }
}
