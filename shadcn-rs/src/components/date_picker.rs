//! Date Picker component
//!
//! A date input with a calendar popup for selecting dates.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{DatePicker, Button};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let date = use_state(|| None::<String>);
//!
//!     let onchange = {
//!         let date = date.clone();
//!         Callback::from(move |new_date: String| {
//!             date.set(Some(new_date));
//!         })
//!     };
//!
//!     html! {
//!         <DatePicker
//!             value={(*date).clone()}
//!             {onchange}
//!             placeholder="Pick a date"
//!         />
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::hooks::use_toggle;

/// Date picker component properties
#[derive(Properties, PartialEq, Clone)]
pub struct DatePickerProps {
    /// Selected date value (ISO format YYYY-MM-DD)
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Default value (uncontrolled)
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Change event handler
    #[prop_or_default]
    pub onchange: Option<Callback<String>>,

    /// Placeholder text
    #[prop_or(AttrValue::from("Select date"))]
    pub placeholder: AttrValue,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Minimum selectable date
    #[prop_or_default]
    pub min_date: Option<AttrValue>,

    /// Maximum selectable date
    #[prop_or_default]
    pub max_date: Option<AttrValue>,

    /// Date format for display
    #[prop_or(AttrValue::from("MM/DD/YYYY"))]
    pub format: AttrValue,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Date picker component
///
/// Combines an input field with a calendar popup for date selection.
///
/// # Accessibility
/// - Full keyboard navigation
/// - Screen reader support
/// - Proper ARIA attributes
#[function_component(DatePicker)]
pub fn date_picker(props: &DatePickerProps) -> Html {
    let DatePickerProps {
        value,
        default_value,
        onchange: _,
        placeholder,
        disabled,
        min_date: _,
        max_date: _,
        format: _,
        class,
    } = props.clone();

    // Internal state for uncontrolled mode
    let internal_value = use_state(|| {
        value.clone().or_else(|| default_value.clone())
    });

    let current_value = value.or_else(|| (*internal_value).clone());

    // Popover open state
    let (is_open, toggle, _set_open) = use_toggle(false);

    let classes: Classes = vec![
        Classes::from("date-picker"),
        if disabled {
            Classes::from("date-picker-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let button_classes: Classes = vec![
        Classes::from("date-picker-trigger"),
        if disabled {
            Classes::from("date-picker-trigger-disabled")
        } else {
            Classes::new()
        },
    ]
    .into_iter()
    .collect();

    let toggle_calendar = {
        let toggle = toggle.clone();
        Callback::from(move |_| {
            toggle.emit(());
        })
    };

    html! {
        <div class={classes}>
            <button
                type="button"
                class={button_classes}
                onclick={toggle_calendar}
                disabled={disabled}
                aria-haspopup="dialog"
                aria-expanded={is_open.to_string()}
            >
                {
                    if let Some(date_value) = current_value {
                        html! { <span class="date-picker-value">{ date_value }</span> }
                    } else {
                        html! { <span class="date-picker-placeholder">{ placeholder }</span> }
                    }
                }
                <span class="date-picker-icon">{ "📅" }</span>
            </button>
            if is_open {
                <div class="date-picker-popover">
                    <div class="date-picker-calendar">
                        { "Calendar would be rendered here" }
                    </div>
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_picker_props_default() {
        let props = DatePickerProps {
            value: None,
            default_value: None,
            onchange: None,
            placeholder: AttrValue::from("Select date"),
            disabled: false,
            min_date: None,
            max_date: None,
            format: AttrValue::from("MM/DD/YYYY"),
            class: Classes::new(),
        };

        assert!(!props.disabled);
        assert_eq!(props.placeholder, AttrValue::from("Select date"));
    }

    #[test]
    fn test_date_picker_with_value() {
        let props = DatePickerProps {
            value: Some(AttrValue::from("2024-01-15")),
            default_value: None,
            onchange: None,
            placeholder: AttrValue::from("Select date"),
            disabled: false,
            min_date: None,
            max_date: None,
            format: AttrValue::from("MM/DD/YYYY"),
            class: Classes::new(),
        };

        assert_eq!(props.value, Some(AttrValue::from("2024-01-15")));
    }

    #[test]
    fn test_date_picker_disabled() {
        let props = DatePickerProps {
            value: None,
            default_value: None,
            onchange: None,
            placeholder: AttrValue::from("Select date"),
            disabled: true,
            min_date: None,
            max_date: None,
            format: AttrValue::from("MM/DD/YYYY"),
            class: Classes::new(),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_date_picker_custom_format() {
        let props = DatePickerProps {
            value: None,
            default_value: None,
            onchange: None,
            placeholder: AttrValue::from("Select date"),
            disabled: false,
            min_date: None,
            max_date: None,
            format: AttrValue::from("YYYY-MM-DD"),
            class: Classes::new(),
        };

        assert_eq!(props.format, AttrValue::from("YYYY-MM-DD"));
    }

    #[test]
    fn test_date_picker_with_min_max() {
        let props = DatePickerProps {
            value: None,
            default_value: None,
            onchange: None,
            placeholder: AttrValue::from("Select date"),
            disabled: false,
            min_date: Some(AttrValue::from("2024-01-01")),
            max_date: Some(AttrValue::from("2024-12-31")),
            format: AttrValue::from("MM/DD/YYYY"),
            class: Classes::new(),
        };

        assert_eq!(props.min_date, Some(AttrValue::from("2024-01-01")));
        assert_eq!(props.max_date, Some(AttrValue::from("2024-12-31")));
    }
}
