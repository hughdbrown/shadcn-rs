//! Spinner component
//!
//! A loading spinner with customizable size and color.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::Spinner;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Spinner />
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::types::Size;

/// Spinner component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SpinnerProps {
    /// Size of the spinner
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Custom color (CSS color value)
    #[prop_or_default]
    pub color: Option<AttrValue>,

    /// ARIA label for screen readers
    #[prop_or(AttrValue::from("Loading"))]
    pub aria_label: AttrValue,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Spinner component
///
/// A loading indicator with a spinning animation.
///
/// # Accessibility
/// - Includes `role="status"` for screen readers
/// - Uses `aria-label` to describe loading state
/// - Hidden from screen readers with `aria-hidden` on decorative element
#[function_component(Spinner)]
pub fn spinner(props: &SpinnerProps) -> Html {
    let SpinnerProps {
        size,
        color,
        aria_label,
        class,
    } = props.clone();

    let size_class = match size {
        Size::Xs => "spinner-xs",
        Size::Sm => "spinner-sm",
        Size::Md => "spinner-md",
        Size::Lg => "spinner-lg",
        Size::Xl => "spinner-xl",
        Size::Xl2 => "spinner-2xl",
    };

    let classes: Classes = vec![
        Classes::from("spinner"),
        Classes::from(size_class),
        class,
    ]
    .into_iter()
    .collect();

    let style = color.map(|c| format!("color: {}", c));

    html! {
        <div
            class={classes}
            role="status"
            aria-label={aria_label}
            style={style}
        >
            <svg
                class="spinner-svg"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                aria-hidden="true"
            >
                <circle
                    class="spinner-track"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    stroke-width="4"
                />
                <path
                    class="spinner-path"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                />
            </svg>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_default() {
        let props = SpinnerProps {
            size: Size::Md,
            color: None,
            aria_label: AttrValue::from("Loading"),
            class: Classes::new(),
        };

        assert_eq!(props.size, Size::Md);
        assert_eq!(props.aria_label, AttrValue::from("Loading"));
    }

    #[test]
    fn test_spinner_custom_size() {
        let props = SpinnerProps {
            size: Size::Lg,
            color: None,
            aria_label: AttrValue::from("Loading"),
            class: Classes::new(),
        };

        assert_eq!(props.size, Size::Lg);
    }

    #[test]
    fn test_spinner_custom_color() {
        let props = SpinnerProps {
            size: Size::Md,
            color: Some(AttrValue::from("#ff0000")),
            aria_label: AttrValue::from("Loading"),
            class: Classes::new(),
        };

        assert_eq!(props.color, Some(AttrValue::from("#ff0000")));
    }

    #[test]
    fn test_spinner_custom_aria_label() {
        let props = SpinnerProps {
            size: Size::Md,
            color: None,
            aria_label: AttrValue::from("Processing request..."),
            class: Classes::new(),
        };

        assert_eq!(props.aria_label, AttrValue::from("Processing request..."));
    }
}
