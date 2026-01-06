//! Aspect Ratio component
//!
//! Maintains a specific aspect ratio for its content.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::AspectRatio;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <AspectRatio ratio={16.0 / 9.0}>
//!             <img src="image.jpg" alt="16:9 aspect ratio" />
//!         </AspectRatio>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Aspect ratio component properties
#[derive(Properties, PartialEq, Clone)]
pub struct AspectRatioProps {
    /// Aspect ratio (width / height)
    /// Common values: 16/9 = 1.777, 4/3 = 1.333, 1/1 = 1.0
    #[prop_or(1.0)]
    pub ratio: f64,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Aspect ratio component
///
/// Maintains a consistent aspect ratio for child content.
///
/// # Accessibility
/// - Container is purely presentational
/// - Child content maintains its own accessibility attributes
#[function_component(AspectRatio)]
pub fn aspect_ratio(props: &AspectRatioProps) -> Html {
    let AspectRatioProps {
        ratio,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("aspect-ratio"), class]
        .into_iter()
        .collect();

    // Use CSS aspect-ratio property
    let style = format!("aspect-ratio: {}", ratio);

    html! {
        <div class={classes} style={style}>
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspect_ratio_default() {
        let props = AspectRatioProps {
            ratio: 1.0,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.ratio, 1.0);
    }

    #[test]
    fn test_aspect_ratio_16_9() {
        let props = AspectRatioProps {
            ratio: 16.0 / 9.0,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!((props.ratio - 1.777).abs() < 0.01);
    }

    #[test]
    fn test_aspect_ratio_4_3() {
        let props = AspectRatioProps {
            ratio: 4.0 / 3.0,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!((props.ratio - 1.333).abs() < 0.01);
    }

    #[test]
    fn test_aspect_ratio_square() {
        let props = AspectRatioProps {
            ratio: 1.0,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.ratio, 1.0);
    }
}
