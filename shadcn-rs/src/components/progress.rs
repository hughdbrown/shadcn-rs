//! Progress component
//!
//! Displays an indicator showing the completion progress of a task.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Progress, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let progress = use_state(|| 65.0);
//!
//!     html! {
//!         <div>
//!             <Label>{ "Loading..." }</Label>
//!             <Progress value={*progress} />
//!             <p>{ format!("{}% complete", *progress) }</p>
//!         </div>
//!     }
//! }
//! ```

use crate::types::{Size, Variant};
use crate::utils::class_names;
use yew::prelude::*;

/// Progress component properties
#[derive(Properties, PartialEq, Clone)]
pub struct ProgressProps {
    /// Current progress value (0-100)
    #[prop_or_default]
    pub value: Option<f64>,

    /// Maximum value (defaults to 100)
    #[prop_or(100.0)]
    pub max: f64,

    /// Size variant
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Color variant
    #[prop_or(Variant::Primary)]
    pub variant: Variant,

    /// Show as indeterminate (animated loading state)
    #[prop_or(false)]
    pub indeterminate: bool,

    /// Show label inside progress bar
    #[prop_or(false)]
    pub show_label: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Custom style
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// ARIA label
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// ARIA labelled by
    #[prop_or_default]
    pub aria_labelledby: Option<AttrValue>,
}

/// Progress component
///
/// An accessible progress indicator for tasks and loading states.
///
/// # Accessibility
/// - Uses proper ARIA attributes (role="progressbar")
/// - Announces current progress to screen readers
/// - Supports indeterminate state for unknown progress
/// - Respects color contrast requirements
#[function_component(Progress)]
pub fn progress(props: &ProgressProps) -> Html {
    let ProgressProps {
        value,
        max,
        size,
        variant,
        indeterminate,
        show_label,
        class,
        style,
        aria_label,
        aria_labelledby,
    } = props.clone();

    // Calculate percentage
    let percentage = if indeterminate {
        None
    } else {
        value.map(|v| (v / max * 100.0).clamp(0.0, 100.0))
    };

    // Build class names
    let classes = class_names(&[
        Some("progress"),
        Some(size.to_class()),
        Some(variant.to_class()),
        if indeterminate {
            Some("progress-indeterminate")
        } else {
            None
        },
    ]);

    let final_classes: Classes = vec![classes, class].into_iter().collect();

    // Progress bar fill style
    let bar_style = if let Some(pct) = percentage {
        format!("width: {}%", pct)
    } else {
        String::new()
    };

    // Determine aria values
    let aria_valuenow = if indeterminate {
        None
    } else {
        value.map(|v| v.to_string())
    };

    let aria_valuemin = if indeterminate {
        None
    } else {
        Some("0".to_string())
    };

    let aria_valuemax = if indeterminate {
        None
    } else {
        Some(max.to_string())
    };

    // Label content
    let label_content = if show_label && !indeterminate {
        percentage.map(|pct| {
            html! {
                <span class="progress-label">
                    { format!("{:.0}%", pct) }
                </span>
            }
        })
    } else {
        None
    };

    html! {
        <div
            class={final_classes}
            role="progressbar"
            aria-valuenow={aria_valuenow}
            aria-valuemin={aria_valuemin}
            aria-valuemax={aria_valuemax}
            aria-label={aria_label}
            aria-labelledby={aria_labelledby}
            style={style}
        >
            <div class="progress-bar" style={bar_style}>
                {label_content}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_props_default() {
        let props = ProgressProps {
            value: None,
            max: 100.0,
            size: Size::Md,
            variant: Variant::Primary,
            indeterminate: false,
            show_label: false,
            class: Classes::new(),
            style: None,
            aria_label: None,
            aria_labelledby: None,
        };

        assert_eq!(props.max, 100.0);
        assert!(!props.indeterminate);
        assert!(!props.show_label);
    }

    #[test]
    fn test_progress_with_value() {
        let props = ProgressProps {
            value: Some(75.0),
            max: 100.0,
            size: Size::Md,
            variant: Variant::Primary,
            indeterminate: false,
            show_label: false,
            class: Classes::new(),
            style: None,
            aria_label: None,
            aria_labelledby: None,
        };

        assert_eq!(props.value, Some(75.0));
        assert_eq!(props.max, 100.0);
    }

    #[test]
    fn test_progress_indeterminate() {
        let props = ProgressProps {
            value: None,
            max: 100.0,
            size: Size::Md,
            variant: Variant::Primary,
            indeterminate: true,
            show_label: false,
            class: Classes::new(),
            style: None,
            aria_label: None,
            aria_labelledby: None,
        };

        assert!(props.indeterminate);
        assert!(props.value.is_none());
    }

    #[test]
    fn test_progress_with_label() {
        let props = ProgressProps {
            value: Some(50.0),
            max: 100.0,
            size: Size::Md,
            variant: Variant::Primary,
            indeterminate: false,
            show_label: true,
            class: Classes::new(),
            style: None,
            aria_label: None,
            aria_labelledby: None,
        };

        assert!(props.show_label);
    }

    #[test]
    fn test_progress_custom_max() {
        let props = ProgressProps {
            value: Some(5.0),
            max: 10.0,
            size: Size::Md,
            variant: Variant::Primary,
            indeterminate: false,
            show_label: false,
            class: Classes::new(),
            style: None,
            aria_label: None,
            aria_labelledby: None,
        };

        assert_eq!(props.value, Some(5.0));
        assert_eq!(props.max, 10.0);
    }

    #[test]
    fn test_progress_sizes() {
        let sizes = vec![Size::Sm, Size::Md, Size::Lg];

        for size in sizes {
            let props = ProgressProps {
                value: Some(50.0),
                max: 100.0,
                size: size.clone(),
                variant: Variant::Primary,
                indeterminate: false,
                show_label: false,
                class: Classes::new(),
                style: None,
                aria_label: None,
                aria_labelledby: None,
            };
            assert_eq!(props.size, size);
        }
    }

    #[test]
    fn test_progress_variants() {
        let variants = vec![
            Variant::Default,
            Variant::Primary,
            Variant::Secondary,
            Variant::Destructive,
        ];

        for variant in variants {
            let props = ProgressProps {
                value: Some(50.0),
                max: 100.0,
                size: Size::Md,
                variant,
                indeterminate: false,
                show_label: false,
                class: Classes::new(),
                style: None,
                aria_label: None,
                aria_labelledby: None,
            };
            assert_eq!(props.variant, variant);
        }
    }

    #[test]
    fn test_progress_zero_value() {
        let props = ProgressProps {
            value: Some(0.0),
            max: 100.0,
            size: Size::Md,
            variant: Variant::Primary,
            indeterminate: false,
            show_label: false,
            class: Classes::new(),
            style: None,
            aria_label: None,
            aria_labelledby: None,
        };

        assert_eq!(props.value, Some(0.0));
    }

    #[test]
    fn test_progress_complete() {
        let props = ProgressProps {
            value: Some(100.0),
            max: 100.0,
            size: Size::Md,
            variant: Variant::Primary,
            indeterminate: false,
            show_label: false,
            class: Classes::new(),
            style: None,
            aria_label: None,
            aria_labelledby: None,
        };

        assert_eq!(props.value, Some(100.0));
    }
}
