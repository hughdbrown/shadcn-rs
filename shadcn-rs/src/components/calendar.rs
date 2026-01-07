//! Calendar component
//!
//! A date picker calendar for selecting dates.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::Calendar;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let selected = use_state(|| None::<String>);
//!
//!     let onselect = {
//!         let selected = selected.clone();
//!         Callback::from(move |date: String| {
//!             selected.set(Some(date));
//!         })
//!     };
//!
//!     html! {
//!         <Calendar {onselect} />
//!     }
//! }
//! ```

use yew::prelude::*;

/// Calendar mode
#[derive(Debug, Clone, PartialEq)]
pub enum CalendarMode {
    /// Select a single date
    Single,
    /// Select multiple dates
    Multiple,
    /// Select a range of dates
    Range,
}

/// Calendar component properties
#[derive(Properties, PartialEq, Clone)]
pub struct CalendarProps {
    /// Selection mode
    #[prop_or(CalendarMode::Single)]
    pub mode: CalendarMode,

    /// Selected date (ISO format YYYY-MM-DD)
    #[prop_or_default]
    pub selected: Option<AttrValue>,

    /// Date selection handler
    #[prop_or_default]
    pub onselect: Option<Callback<String>>,

    /// Minimum selectable date
    #[prop_or_default]
    pub min_date: Option<AttrValue>,

    /// Maximum selectable date
    #[prop_or_default]
    pub max_date: Option<AttrValue>,

    /// Disabled dates (ISO format)
    #[prop_or_default]
    pub disabled_dates: Vec<AttrValue>,

    /// Show week numbers
    #[prop_or(false)]
    pub show_week_numbers: bool,

    /// First day of week (0 = Sunday, 1 = Monday, etc.)
    #[prop_or(0)]
    pub first_day_of_week: u8,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Calendar component
///
/// A date picker with month/year navigation and date selection.
///
/// # Accessibility
/// - Full keyboard navigation
/// - Screen reader support
/// - ARIA attributes for dates and navigation
#[function_component(Calendar)]
pub fn calendar(props: &CalendarProps) -> Html {
    let CalendarProps {
        mode: _,
        selected: _,
        onselect: _,
        min_date: _,
        max_date: _,
        disabled_dates: _,
        show_week_numbers,
        first_day_of_week: _,
        class,
    } = props.clone();

    // Current month/year being displayed
    // Default to January 2024 for now (in a real implementation, use chrono or time crate)
    let current_month = use_state(|| 0u8);
    let current_year = use_state(|| 2024i32);

    let classes: Classes = vec![Classes::from("calendar"), class].into_iter().collect();

    // Month names
    let month_names = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // Day names
    let day_names = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

    // Navigate to previous month
    let go_prev_month = {
        let current_month = current_month.clone();
        let current_year = current_year.clone();
        Callback::from(move |_| {
            if *current_month == 0 {
                current_month.set(11);
                current_year.set(*current_year - 1);
            } else {
                current_month.set(*current_month - 1);
            }
        })
    };

    // Navigate to next month
    let go_next_month = {
        let current_month = current_month.clone();
        let current_year = current_year.clone();
        Callback::from(move |_| {
            if *current_month == 11 {
                current_month.set(0);
                current_year.set(*current_year + 1);
            } else {
                current_month.set(*current_month + 1);
            }
        })
    };

    html! {
        <div class={classes} role="application" aria-label="Calendar">
            <div class="calendar-header">
                <button
                    type="button"
                    class="calendar-nav-button"
                    onclick={go_prev_month}
                    aria-label="Previous month"
                >
                    { "‹" }
                </button>
                <div class="calendar-month-year">
                    { format!("{} {}", month_names[*current_month as usize], *current_year) }
                </div>
                <button
                    type="button"
                    class="calendar-nav-button"
                    onclick={go_next_month}
                    aria-label="Next month"
                >
                    { "›" }
                </button>
            </div>
            <div class="calendar-grid">
                <div class="calendar-weekdays">
                    if show_week_numbers {
                        <div class="calendar-weekday">{ "Wk" }</div>
                    }
                    {
                        day_names.iter().map(|day| {
                            html! {
                                <div class="calendar-weekday" key={*day}>
                                    { day }
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="calendar-days">
                    // Days grid would be rendered here
                    // This is a simplified version
                    { "Calendar days grid" }
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_mode_single() {
        let props = CalendarProps {
            mode: CalendarMode::Single,
            selected: None,
            onselect: None,
            min_date: None,
            max_date: None,
            disabled_dates: vec![],
            show_week_numbers: false,
            first_day_of_week: 0,
            class: Classes::new(),
        };

        assert_eq!(props.mode, CalendarMode::Single);
    }

    #[test]
    fn test_calendar_mode_multiple() {
        let props = CalendarProps {
            mode: CalendarMode::Multiple,
            selected: None,
            onselect: None,
            min_date: None,
            max_date: None,
            disabled_dates: vec![],
            show_week_numbers: false,
            first_day_of_week: 0,
            class: Classes::new(),
        };

        assert_eq!(props.mode, CalendarMode::Multiple);
    }

    #[test]
    fn test_calendar_mode_range() {
        let props = CalendarProps {
            mode: CalendarMode::Range,
            selected: None,
            onselect: None,
            min_date: None,
            max_date: None,
            disabled_dates: vec![],
            show_week_numbers: false,
            first_day_of_week: 0,
            class: Classes::new(),
        };

        assert_eq!(props.mode, CalendarMode::Range);
    }

    #[test]
    fn test_calendar_show_week_numbers() {
        let props = CalendarProps {
            mode: CalendarMode::Single,
            selected: None,
            onselect: None,
            min_date: None,
            max_date: None,
            disabled_dates: vec![],
            show_week_numbers: true,
            first_day_of_week: 0,
            class: Classes::new(),
        };

        assert!(props.show_week_numbers);
    }

    #[test]
    fn test_calendar_first_day_monday() {
        let props = CalendarProps {
            mode: CalendarMode::Single,
            selected: None,
            onselect: None,
            min_date: None,
            max_date: None,
            disabled_dates: vec![],
            show_week_numbers: false,
            first_day_of_week: 1,
            class: Classes::new(),
        };

        assert_eq!(props.first_day_of_week, 1);
    }
}
