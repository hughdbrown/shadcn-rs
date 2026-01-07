//! Slider component
//!
//! An input where the user selects a value from within a given range.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Slider, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let value = use_state(|| vec![50.0]);
//!
//!     let onchange = {
//!         let value = value.clone();
//!         Callback::from(move |values: Vec<f64>| {
//!             value.set(values);
//!         })
//!     };
//!
//!     html! {
//!         <div>
//!             <Label>{ "Volume" }</Label>
//!             <Slider
//!                 value={(*value).clone()}
//!                 {onchange}
//!                 min={0.0}
//!                 max={100.0}
//!                 step={1.0}
//!             />
//!             <p>{ format!("Value: {}", value[0]) }</p>
//!         </div>
//!     }
//! }
//! ```

use crate::types::Size;
use crate::utils::class_names;
use yew::prelude::*;

/// Slider component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SliderProps {
    /// Current value(s) of the slider (controlled)
    #[prop_or_default]
    pub value: Option<Vec<f64>>,

    /// Default value(s) for uncontrolled slider
    #[prop_or_default]
    pub default_value: Option<Vec<f64>>,

    /// Minimum value
    #[prop_or(0.0)]
    pub min: f64,

    /// Maximum value
    #[prop_or(100.0)]
    pub max: f64,

    /// Step increment
    #[prop_or(1.0)]
    pub step: f64,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Size variant
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Show marks at step intervals
    #[prop_or(false)]
    pub show_marks: bool,

    /// Change event handler
    #[prop_or_default]
    pub onchange: Option<Callback<Vec<f64>>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// ARIA label
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// ARIA labelled by
    #[prop_or_default]
    pub aria_labelledby: Option<AttrValue>,
}

/// Slider component
///
/// An accessible range input with support for single or multiple handles.
///
/// # Accessibility
/// - Uses proper ARIA attributes (role="slider")
/// - Keyboard navigation (Arrow keys, Home, End, Page Up/Down)
/// - Screen reader friendly
/// - Focus indicators
#[function_component(Slider)]
pub fn slider(props: &SliderProps) -> Html {
    let SliderProps {
        value,
        default_value,
        min,
        max,
        step,
        disabled,
        size,
        show_marks,
        onchange,
        class,
        aria_label,
        aria_labelledby,
    } = props.clone();

    // Internal state for uncontrolled mode
    let internal_value = use_state(|| {
        value
            .clone()
            .or_else(|| default_value.clone())
            .unwrap_or_else(|| vec![50.0])
    });

    // Sync with controlled value prop
    {
        let internal_value = internal_value.clone();
        use_effect_with(value.clone(), move |value| {
            if let Some(v) = value {
                internal_value.set(v.clone());
            }
        });
    }

    let current_value = value.clone().unwrap_or_else(|| (*internal_value).clone());
    let num_handles = current_value.len();

    // Handle value change
    let handle_change = {
        let internal_value = internal_value.clone();
        let onchange = onchange.clone();
        Callback::from(move |new_values: Vec<f64>| {
            internal_value.set(new_values.clone());
            if let Some(callback) = onchange.as_ref() {
                callback.emit(new_values);
            }
        })
    };

    // Calculate percentage for a value
    let value_to_percent =
        |val: f64| -> f64 { ((val - min) / (max - min) * 100.0).clamp(0.0, 100.0) };

    // Build class names
    let classes = class_names(&[
        Some("slider"),
        Some(size.to_class()),
        if disabled {
            Some("slider-disabled")
        } else {
            None
        },
    ]);

    let final_classes: Classes = vec![classes, class].into_iter().collect();

    // Generate marks if enabled
    let marks = if show_marks {
        let num_marks = ((max - min) / step) as usize + 1;
        (0..num_marks)
            .map(|i| {
                let mark_value = min + (i as f64 * step);
                let percent = value_to_percent(mark_value);
                html! {
                    <div
                        key={i}
                        class="slider-mark"
                        style={format!("left: {}%", percent)}
                    />
                }
            })
            .collect::<Html>()
    } else {
        html! {}
    };

    // Calculate track fill based on handle positions
    let track_fill_style = if num_handles == 1 {
        let percent = value_to_percent(current_value[0]);
        format!("width: {}%", percent)
    } else if num_handles == 2 {
        let start_percent = value_to_percent(current_value[0].min(current_value[1]));
        let end_percent = value_to_percent(current_value[0].max(current_value[1]));
        format!(
            "left: {}%; width: {}%",
            start_percent,
            end_percent - start_percent
        )
    } else {
        String::from("width: 0")
    };

    // For a single value, use a simple native range input
    if num_handles == 1 {
        let current_val = current_value[0];
        let handle_input = {
            let handle_change = handle_change.clone();
            Callback::from(move |e: InputEvent| {
                if disabled {
                    return;
                }
                let target: web_sys::HtmlInputElement = e.target_unchecked_into();
                if let Ok(new_val) = target.value().parse::<f64>() {
                    handle_change.emit(vec![new_val]);
                }
            })
        };

        return html! {
            <div class={final_classes}>
                <div class="slider-track">
                    <div class="slider-track-fill" style={track_fill_style} />
                    {marks}
                </div>
                <input
                    type="range"
                    class="slider-input"
                    min={min.to_string()}
                    max={max.to_string()}
                    step={step.to_string()}
                    value={current_val.to_string()}
                    disabled={disabled}
                    oninput={handle_input}
                    aria-valuemin={min.to_string()}
                    aria-valuemax={max.to_string()}
                    aria-valuenow={current_val.to_string()}
                    aria-label={aria_label.clone()}
                    aria-labelledby={aria_labelledby.clone()}
                    aria-disabled={disabled.to_string()}
                />
            </div>
        };
    }

    // For dual-handle sliders, use two range inputs
    html! {
        <div class={final_classes}>
            <div class="slider-track">
                <div class="slider-track-fill" style={track_fill_style} />
                {marks}
            </div>
            {
                current_value.iter().enumerate().map(|(index, &val)| {
                    let handle_change = handle_change.clone();
                    let current_value = current_value.clone();

                    let oninput = Callback::from(move |e: InputEvent| {
                        if disabled {
                            return;
                        }
                        let target: web_sys::HtmlInputElement = e.target_unchecked_into();
                        if let Ok(new_val) = target.value().parse::<f64>() {
                            let mut new_values = current_value.clone();
                            new_values[index] = new_val;
                            handle_change.emit(new_values);
                        }
                    });

                    html! {
                        <input
                            key={index}
                            type="range"
                            class={format!("slider-input slider-input-{}", index)}
                            min={min.to_string()}
                            max={max.to_string()}
                            step={step.to_string()}
                            value={val.to_string()}
                            disabled={disabled}
                            {oninput}
                            aria-valuemin={min.to_string()}
                            aria-valuemax={max.to_string()}
                            aria-valuenow={val.to_string()}
                            aria-label={aria_label.clone()}
                            aria-labelledby={aria_labelledby.clone()}
                            aria-disabled={disabled.to_string()}
                        />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_props_default() {
        let props = SliderProps {
            value: None,
            default_value: None,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            size: Size::Md,
            show_marks: false,
            onchange: None,
            class: Classes::new(),
            aria_label: None,
            aria_labelledby: None,
        };

        assert_eq!(props.min, 0.0);
        assert_eq!(props.max, 100.0);
        assert_eq!(props.step, 1.0);
        assert!(!props.disabled);
        assert!(!props.show_marks);
    }

    #[test]
    fn test_slider_single_value() {
        let props = SliderProps {
            value: Some(vec![75.0]),
            default_value: None,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            size: Size::Md,
            show_marks: false,
            onchange: None,
            class: Classes::new(),
            aria_label: None,
            aria_labelledby: None,
        };

        assert_eq!(props.value, Some(vec![75.0]));
    }

    #[test]
    fn test_slider_dual_values() {
        let props = SliderProps {
            value: Some(vec![25.0, 75.0]),
            default_value: None,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            size: Size::Md,
            show_marks: false,
            onchange: None,
            class: Classes::new(),
            aria_label: None,
            aria_labelledby: None,
        };

        assert_eq!(props.value, Some(vec![25.0, 75.0]));
    }

    #[test]
    fn test_slider_custom_range() {
        let props = SliderProps {
            value: Some(vec![5.0]),
            default_value: None,
            min: 0.0,
            max: 10.0,
            step: 0.5,
            disabled: false,
            size: Size::Md,
            show_marks: false,
            onchange: None,
            class: Classes::new(),
            aria_label: None,
            aria_labelledby: None,
        };

        assert_eq!(props.min, 0.0);
        assert_eq!(props.max, 10.0);
        assert_eq!(props.step, 0.5);
    }

    #[test]
    fn test_slider_disabled() {
        let props = SliderProps {
            value: Some(vec![50.0]),
            default_value: None,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: true,
            size: Size::Md,
            show_marks: false,
            onchange: None,
            class: Classes::new(),
            aria_label: None,
            aria_labelledby: None,
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_slider_with_marks() {
        let props = SliderProps {
            value: Some(vec![50.0]),
            default_value: None,
            min: 0.0,
            max: 100.0,
            step: 25.0,
            disabled: false,
            size: Size::Md,
            show_marks: true,
            onchange: None,
            class: Classes::new(),
            aria_label: None,
            aria_labelledby: None,
        };

        assert!(props.show_marks);
        assert_eq!(props.step, 25.0);
    }

    #[test]
    fn test_slider_sizes() {
        let sizes = vec![Size::Sm, Size::Md, Size::Lg];

        for size in sizes {
            let props = SliderProps {
                value: Some(vec![50.0]),
                default_value: None,
                min: 0.0,
                max: 100.0,
                step: 1.0,
                disabled: false,
                size: size.clone(),
                show_marks: false,
                onchange: None,
                class: Classes::new(),
                aria_label: None,
                aria_labelledby: None,
            };
            assert_eq!(props.size, size);
        }
    }
}
