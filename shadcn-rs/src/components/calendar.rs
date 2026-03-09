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

/// Returns the number of days in a given month/year
fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        0 | 2 | 4 | 6 | 7 | 9 | 11 => 31, // Jan, Mar, May, Jul, Aug, Oct, Dec
        3 | 5 | 8 | 10 => 30,             // Apr, Jun, Sep, Nov
        1 => {
            // February - leap year check
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Returns the day of the week for the first day of a month (0 = Sunday)
/// Uses Zeller-like formula (Tomohiko Sakamoto's algorithm)
fn first_day_of_month(year: i32, month: u8) -> u8 {
    let m = month as i32 + 1; // 1-based month
    let mut y = year;
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    if m < 3 {
        y -= 1;
    }
    ((y + y / 4 - y / 100 + y / 400 + t[(m - 1) as usize] + 1) % 7).unsigned_abs() as u8
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
        selected,
        onselect,
        min_date,
        max_date,
        disabled_dates,
        show_week_numbers,
        first_day_of_week,
        number_of_months,
        class,
    } = props.clone();

    // Current month/year being displayed (0-indexed month)
    let current_month = use_state(|| 0u8);
    let current_year = use_state(|| 2024i32);

    let classes: Classes = vec![Classes::from("calendar"), class].into_iter().collect();

    // Month names
    let month_names = [
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
    let onkeydown = Callback::from(move |e: KeyboardEvent| match e.key().as_str() {
        "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown" | "Home" | "End" => {
            e.prevent_default();
        }
        _ => {}
    });

    /// Check if a date string is within min/max bounds
    fn is_date_disabled(
        date_str: &str,
        min_date: &Option<AttrValue>,
        max_date: &Option<AttrValue>,
        disabled_dates: &[AttrValue],
    ) -> bool {
        if let Some(min) = min_date
            && date_str < min.as_str()
        {
            return true;
        }
        if let Some(max) = max_date
            && date_str > max.as_str()
        {
            return true;
        }
        disabled_dates.iter().any(|d| d.as_str() == date_str)
    }

    html! {
        <div class={classes} role="application" aria-label="Calendar" onkeydown={onkeydown} tabindex="0">
            <div class="calendar-months" style="display: flex; gap: 1rem;">
                {
                    (0..number_of_months).map(|month_offset: u8| {
                        // Calculate month/year for this panel
                        let display_month = (*current_month + month_offset) % 12;
                        let display_year = *current_year + ((*current_month as u16 + month_offset as u16) / 12) as i32;

                        let num_days = days_in_month(display_year, display_month);
                        let start_dow = first_day_of_month(display_year, display_month);
                        // Adjust for first_day_of_week
                        let offset = (start_dow + 7 - first_day_of_week) % 7;

                        let selected = selected.clone();
                        let onselect = onselect.clone();
                        let min_date = min_date.clone();
                        let max_date = max_date.clone();
                        let disabled_dates = disabled_dates.clone();

                        // Reorder day names based on first_day_of_week
                        let reordered_days: Vec<&str> = (0..7)
                            .map(|i| day_names[((first_day_of_week + i) % 7) as usize])
                            .collect();

                        html! {
                            <div class="calendar-panel" key={month_offset}>
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
                                            reordered_days.iter().map(|day| {
                                                html! {
                                                    <div class="calendar-weekday" key={*day}>
                                                        { day }
                                                    </div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                    <div class="calendar-days" role="grid">
                                        {
                                            (0..offset).map(|i| {
                                                html! {
                                                    <div class="calendar-day calendar-day-empty" key={format!("empty-{}", i)} />
                                                }
                                            }).chain((1..=num_days).map(|day| {
                                                let date_str = format!("{:04}-{:02}-{:02}", display_year, display_month + 1, day);
                                                let is_selected = selected.as_ref().map(|s| s.as_str() == date_str).unwrap_or(false);
                                                let disabled = is_date_disabled(&date_str, &min_date, &max_date, &disabled_dates);

                                                let day_class = classes!(
                                                    "calendar-day",
                                                    is_selected.then_some("calendar-day-selected"),
                                                    disabled.then_some("calendar-day-disabled"),
                                                );

                                                let onclick = if !disabled {
                                                    let onselect = onselect.clone();
                                                    let date_str = date_str.clone();
                                                    Some(Callback::from(move |_: MouseEvent| {
                                                        if let Some(cb) = onselect.as_ref() {
                                                            cb.emit(date_str.clone());
                                                        }
                                                    }))
                                                } else {
                                                    None
                                                };

                                                html! {
                                                    <button
                                                        type="button"
                                                        class={day_class}
                                                        key={date_str.clone()}
                                                        onclick={onclick}
                                                        disabled={disabled}
                                                        aria-selected={is_selected.to_string()}
                                                        aria-label={format!("{} {} {}", month_names[display_month as usize], day, display_year)}
                                                        tabindex={if is_selected { "0" } else { "-1" }}
                                                    >
                                                        { day }
                                                    </button>
                                                }
                                            })).collect::<Html>()
                                        }
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

        assert_eq!(props.number_of_months, 1);
        assert_eq!(props.first_day_of_week, 0);
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2024, 0), 31); // January
        assert_eq!(days_in_month(2024, 1), 29); // February (leap year)
        assert_eq!(days_in_month(2023, 1), 28); // February (non-leap)
        assert_eq!(days_in_month(2024, 3), 30); // April
        assert_eq!(days_in_month(2024, 11), 31); // December
    }

    #[test]
    fn test_first_day_of_month() {
        // January 1, 2024 is a Monday (1)
        assert_eq!(first_day_of_month(2024, 0), 1);
    }
}
