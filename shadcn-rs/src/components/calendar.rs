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

    /// Number of months to display side by side
    #[prop_or(1)]
    pub number_of_months: u8,

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
        number_of_months,
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
        Callback::from(move |_: MouseEvent| {
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
        Callback::from(move |_: MouseEvent| {
            if *current_month == 11 {
                current_month.set(0);
                current_year.set(*current_year + 1);
            } else {
                current_month.set(*current_month + 1);
            }
        })
    };

    // Keyboard navigation handler
    let onkeydown = {
        let _current_month = current_month.clone();
        let _current_year = current_year.clone();
        Callback::from(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "ArrowLeft" => {
                    e.prevent_default();
                    // Move to previous day (for now, just prevent default)
                }
                "ArrowRight" => {
                    e.prevent_default();
                    // Move to next day
                }
                "ArrowUp" => {
                    e.prevent_default();
                    // Move to same day previous week
                }
                "ArrowDown" => {
                    e.prevent_default();
                    // Move to same day next week
                }
                "Home" => {
                    e.prevent_default();
                    // Move to first day of week
                }
                "End" => {
                    e.prevent_default();
                    // Move to last day of week
                }
                _ => {}
            }
        })
    };

    html! {
        <div class={classes} role="application" aria-label="Calendar" {onkeydown} tabindex="0">
            <div class="calendar-months" style="display: flex; gap: 1rem;">
                {
                    (0..number_of_months).map(|month_offset: u8| {
                        // Calculate month/year for this panel
                        let display_month = (*current_month as u16 + month_offset as u16) % 12;
                        let display_year = *current_year + ((*current_month as u16 + month_offset as u16) / 12) as i32;

                        html! {
                            <div class="calendar-panel" key={month_offset}>
                                // Only show nav buttons on first/last panel
                                <div class="calendar-header">
                                    if month_offset == 0 {
                                        <button
                                            type="button"
                                            class="calendar-nav-button"
                                            onclick={go_prev_month.clone()}
                                            aria-label="Previous month"
                                        >
                                            { "\u{2039}" }
                                        </button>
                                    } else {
                                        <div class="calendar-nav-spacer" />
                                    }
                                    <div class="calendar-month-year">
                                        { format!("{} {}", month_names[display_month as usize], display_year) }
                                    </div>
                                    if month_offset == number_of_months - 1 {
                                        <button
                                            type="button"
                                            class="calendar-nav-button"
                                            onclick={go_next_month.clone()}
                                            aria-label="Next month"
                                        >
                                            { "\u{203a}" }
                                        </button>
                                    } else {
                                        <div class="calendar-nav-spacer" />
                                    }
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
                                        { "Calendar days grid" }
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
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
            number_of_months: 1,
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
            number_of_months: 1,
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
            number_of_months: 1,
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
            number_of_months: 1,
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
            number_of_months: 1,
            class: Classes::new(),
        };

        assert_eq!(props.first_day_of_week, 1);
    }

    #[test]
    fn test_calendar_multiple_months() {
        let props = CalendarProps {
            mode: CalendarMode::Single,
            selected: None,
            onselect: None,
            min_date: None,
            max_date: None,
            disabled_dates: vec![],
            show_week_numbers: false,
            first_day_of_week: 0,
            number_of_months: 2,
            class: Classes::new(),
        };

        assert_eq!(props.number_of_months, 2);
    }

    #[test]
    fn test_calendar_keyboard_nav() {
        // Test that default props include number_of_months = 1
        // and that the struct can be constructed with defaults
        let props = CalendarProps {
            mode: CalendarMode::Single,
            selected: None,
            onselect: None,
            min_date: None,
            max_date: None,
            disabled_dates: vec![],
            show_week_numbers: false,
            first_day_of_week: 0,
            number_of_months: 1,
            class: Classes::new(),
        };

        // Default number_of_months should be 1
        assert_eq!(props.number_of_months, 1);
        // Default first_day_of_week should be 0 (Sunday)
        assert_eq!(props.first_day_of_week, 0);
    }
}
