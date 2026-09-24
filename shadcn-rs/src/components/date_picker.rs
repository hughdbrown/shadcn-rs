//! Date Picker component
//!
//! A date input with a calendar popup for selecting dates.

use crate::components::{Calendar, CalendarMode};
use crate::hooks::{use_click_outside_conditional, use_escape_key_conditional, use_toggle};
use yew::prelude::*;

fn format_display_value(value: &str, format: &str) -> String {
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() != 3 {
        return value.to_string();
    }

    match format {
        "MM/DD/YYYY" => format!("{}/{}/{}", parts[1], parts[2], parts[0]),
        "DD/MM/YYYY" => format!("{}/{}/{}", parts[2], parts[1], parts[0]),
        _ => value.to_string(),
    }
}

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
#[function_component(DatePicker)]
pub fn date_picker(props: &DatePickerProps) -> Html {
    let internal_value = use_state(|| props.value.clone().or_else(|| props.default_value.clone()));
    let current_value = props.value.clone().or_else(|| (*internal_value).clone());
    let (is_open, toggle, set_open) = use_toggle(false);
    let root_ref = use_node_ref();

    use_click_outside_conditional(
        root_ref.clone(),
        {
            let set_open = set_open.clone();
            move || set_open.emit(false)
        },
        is_open,
    );
    use_escape_key_conditional(
        {
            let set_open = set_open.clone();
            move || set_open.emit(false)
        },
        is_open,
    );

    let display_value = current_value
        .as_ref()
        .map(|value| AttrValue::from(format_display_value(value.as_str(), props.format.as_str())));

    let classes: Classes = vec![
        Classes::from("date-picker"),
        if props.disabled {
            Classes::from("date-picker-disabled")
        } else {
            Classes::new()
        },
        props.class.clone(),
    ]
    .into_iter()
    .collect();

    let button_classes: Classes = vec![
        Classes::from("date-picker-trigger"),
        if props.disabled {
            Classes::from("date-picker-trigger-disabled")
        } else {
            Classes::new()
        },
    ]
    .into_iter()
    .collect();

    let toggle_calendar = {
        let toggle = toggle.clone();
        Callback::from(move |_: MouseEvent| toggle.emit(()))
    };

    let handle_select = {
        let internal_value = internal_value.clone();
        let onchange = props.onchange.clone();
        let set_open = set_open.clone();
        Callback::from(move |date: String| {
            if !date.is_empty() {
                internal_value.set(Some(AttrValue::from(date.clone())));
                if let Some(callback) = onchange.as_ref() {
                    callback.emit(date);
                }
                set_open.emit(false);
            }
        })
    };

    html! {
        <div class={classes} ref={root_ref}>
            <button
                type="button"
                class={button_classes}
                onclick={toggle_calendar}
                disabled={props.disabled}
                aria-haspopup="dialog"
                aria-expanded={is_open.to_string()}
            >
                if let Some(value) = display_value {
                    <span class="date-picker-value">{ value }</span>
                } else {
                    <span class="date-picker-placeholder">{ props.placeholder.clone() }</span>
                }
                <span class="date-picker-icon" aria-hidden="true">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M8 2v4" />
                        <path d="M16 2v4" />
                        <rect width="18" height="18" x="3" y="4" rx="2" />
                        <path d="M3 10h18" />
                    </svg>
                </span>
            </button>
            if is_open {
                <div class="date-picker-popover" role="dialog" aria-modal="false">
                    <Calendar
                        mode={CalendarMode::Single}
                        selected={current_value.clone()}
                        onselect={handle_select}
                        min_date={props.min_date.clone()}
                        max_date={props.max_date.clone()}
                    />
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_display_value() {
        assert_eq!(
            format_display_value("2024-01-15", "MM/DD/YYYY"),
            "01/15/2024"
        );
        assert_eq!(
            format_display_value("2024-01-15", "DD/MM/YYYY"),
            "15/01/2024"
        );
    }

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
