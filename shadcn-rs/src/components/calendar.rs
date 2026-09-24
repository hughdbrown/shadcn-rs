//! Calendar component
//!
//! A date picker calendar for selecting single dates, multiple dates, or ranges.

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

    /// Selected date(s).
    ///
    /// Encoding:
    /// - `Single`: `YYYY-MM-DD`
    /// - `Multiple`: `YYYY-MM-DD,YYYY-MM-DD`
    /// - `Range`: `YYYY-MM-DD..YYYY-MM-DD`
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

#[derive(Debug, Clone, PartialEq)]
struct CalendarSelection {
    single: Option<String>,
    multiple: Vec<String>,
    range_start: Option<String>,
    range_end: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct VisibleMonth {
    year: i32,
    month: u8,
}

/// Returns the number of days in a given month/year.
fn days_in_month(year: i32, month: u8) -> u8 {
    debug_assert!(month < 12, "invalid month: {month}");
    match month {
        0 | 2 | 4 | 6 | 7 | 9 | 11 => 31,
        3 | 5 | 8 | 10 => 30,
        1 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Returns the day of the week for the first day of a month (0 = Sunday).
fn first_day_of_month(year: i32, month: u8) -> u8 {
    let m = month as i32 + 1;
    let mut y = year;
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    if m < 3 {
        y -= 1;
    }
    ((y + y / 4 - y / 100 + y / 400 + t[(m - 1) as usize] + 1) % 7).unsigned_abs() as u8
}

fn iso_date(year: i32, month: u8, day: u8) -> String {
    format!(
        "{year:04}-{month_plus_one:02}-{day:02}",
        month_plus_one = month + 1
    )
}

fn parse_iso_date(value: &str) -> Option<(i32, u8, u8)> {
    let mut parts = value.split('-');
    let year = parts.next()?.parse::<i32>().ok()?;
    let month = parts.next()?.parse::<u8>().ok()?.checked_sub(1)?;
    let day = parts.next()?.parse::<u8>().ok()?;
    if month < 12 && day >= 1 && day <= days_in_month(year, month) {
        Some((year, month, day))
    } else {
        None
    }
}

fn parse_selection(mode: &CalendarMode, selected: Option<&AttrValue>) -> CalendarSelection {
    let Some(selected) = selected.map(ToString::to_string) else {
        return CalendarSelection {
            single: None,
            multiple: Vec::new(),
            range_start: None,
            range_end: None,
        };
    };

    match mode {
        CalendarMode::Single => CalendarSelection {
            single: Some(selected),
            multiple: Vec::new(),
            range_start: None,
            range_end: None,
        },
        CalendarMode::Multiple => CalendarSelection {
            single: None,
            multiple: selected
                .split(',')
                .map(str::trim)
                .filter(|value| parse_iso_date(value).is_some())
                .map(str::to_string)
                .collect(),
            range_start: None,
            range_end: None,
        },
        CalendarMode::Range => {
            let mut parts = selected.split("..");
            let start = parts
                .next()
                .map(str::trim)
                .filter(|value| parse_iso_date(value).is_some())
                .map(str::to_string);
            let end = parts
                .next()
                .map(str::trim)
                .filter(|value| parse_iso_date(value).is_some())
                .map(str::to_string);
            CalendarSelection {
                single: None,
                multiple: Vec::new(),
                range_start: start,
                range_end: end,
            }
        }
    }
}

fn initial_visible_month(selected: Option<&AttrValue>) -> VisibleMonth {
    if let Some(selected) = selected
        && let Some(first_value) = selected
            .as_str()
            .split([',', '.'])
            .find(|value| value.contains('-'))
        && let Some((year, month, _)) = parse_iso_date(first_value)
    {
        return VisibleMonth { year, month };
    }

    let (year, month, _) = today_ymd();
    VisibleMonth { year, month }
}

/// Today's date in the browser's local time zone (month is zero-based).
///
/// `std::time::SystemTime::now()` panics on `wasm32-unknown-unknown`, so the
/// browser build reads the clock through `js_sys::Date`.
#[cfg(target_arch = "wasm32")]
fn today_ymd() -> (i32, u8, u8) {
    let now = js_sys::Date::new_0();
    (
        now.get_full_year() as i32,
        now.get_month() as u8,
        now.get_date() as u8,
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn today_ymd() -> (i32, u8, u8) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or_default();
    civil_from_days(seconds.div_euclid(86_400))
}

fn civil_from_days(days_since_epoch: i64) -> (i32, u8, u8) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    (year as i32, (m - 1) as u8, d as u8)
}

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
    disabled_dates.iter().any(|item| item.as_str() == date_str)
}

fn is_selected(mode: &CalendarMode, selection: &CalendarSelection, date_str: &str) -> bool {
    match mode {
        CalendarMode::Single => selection.single.as_deref() == Some(date_str),
        CalendarMode::Multiple => selection.multiple.iter().any(|item| item == date_str),
        CalendarMode::Range => {
            if let (Some(start), Some(end)) = (
                selection.range_start.as_deref(),
                selection.range_end.as_deref(),
            ) {
                (start..=end).contains(&date_str)
            } else {
                selection.range_start.as_deref() == Some(date_str)
            }
        }
    }
}

fn is_range_boundary(selection: &CalendarSelection, date_str: &str) -> bool {
    selection.range_start.as_deref() == Some(date_str)
        || selection.range_end.as_deref() == Some(date_str)
}

fn next_selection(mode: &CalendarMode, selection: &CalendarSelection, date_str: &str) -> String {
    match mode {
        CalendarMode::Single => date_str.to_string(),
        CalendarMode::Multiple => {
            let mut dates = selection.multiple.clone();
            if let Some(position) = dates.iter().position(|item| item == date_str) {
                dates.remove(position);
            } else {
                dates.push(date_str.to_string());
                dates.sort();
            }
            dates.join(",")
        }
        CalendarMode::Range => match (
            selection.range_start.as_deref(),
            selection.range_end.as_deref(),
        ) {
            (None, _) | (Some(_), Some(_)) => date_str.to_string(),
            (Some(start), None) => {
                if date_str < start {
                    format!("{date_str}..{start}")
                } else {
                    format!("{start}..{date_str}")
                }
            }
        },
    }
}

/// Days since 1970-01-01 for a civil date (month is zero-based).
fn days_from_civil(year: i32, month: u8, day: u8) -> i64 {
    let month = i64::from(month) + 1;
    let year = i64::from(year) - i64::from(month <= 2);
    let era = year.div_euclid(400);
    let yoe = year - era * 400;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// ISO-8601 week number of the date `days_since_epoch`.
fn iso_week(days_since_epoch: i64) -> u8 {
    // 1970-01-01 was a Thursday; ISO weeks start on Monday.
    let weekday_from_monday = (days_since_epoch + 3).rem_euclid(7);
    let thursday = days_since_epoch - weekday_from_monday + 3;
    let (iso_year, _, _) = civil_from_days(thursday);
    ((thursday - days_from_civil(iso_year, 0, 1)) / 7 + 1) as u8
}

/// Week number shown for a calendar row whose first cell is `row_start_day`
/// (1-based day of month, possibly < 1 for leading blank cells).
///
/// The row's fourth cell decides the week: for Monday-first rows it is the
/// ISO Thursday, and for Sunday-first rows it keeps Sunday grouped with the
/// following Monday–Saturday.
fn row_week_number(year: i32, month: u8, row_start_day: i32) -> u8 {
    let middle = days_from_civil(year, month, 1) + i64::from(row_start_day) + 2;
    iso_week(middle)
}

/// Calendar component
#[function_component(Calendar)]
pub fn calendar(props: &CalendarProps) -> Html {
    let visible_month = use_state(|| initial_visible_month(props.selected.as_ref()));
    // Out-of-range values (e.g. 7 or 255) would otherwise overflow the weekday math.
    let first_day_of_week = props.first_day_of_week % 7;
    let selection = parse_selection(&props.mode, props.selected.as_ref());
    let classes: Classes = vec![Classes::from("calendar"), props.class.clone()]
        .into_iter()
        .collect();
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
    let day_names = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
    let (today_year, today_month, today_day) = today_ymd();

    let go_prev_month = {
        let visible_month = visible_month.clone();
        Callback::from(move |_: MouseEvent| {
            let current = (*visible_month).clone();
            visible_month.set(if current.month == 0 {
                VisibleMonth {
                    year: current.year - 1,
                    month: 11,
                }
            } else {
                VisibleMonth {
                    year: current.year,
                    month: current.month - 1,
                }
            });
        })
    };

    let go_next_month = {
        let visible_month = visible_month.clone();
        Callback::from(move |_: MouseEvent| {
            let current = (*visible_month).clone();
            visible_month.set(if current.month == 11 {
                VisibleMonth {
                    year: current.year + 1,
                    month: 0,
                }
            } else {
                VisibleMonth {
                    year: current.year,
                    month: current.month + 1,
                }
            });
        })
    };

    html! {
        <div class={classes} role="application" aria-label="Calendar">
            <div class="calendar-months">
                {
                    (0..props.number_of_months).map(|offset| {
                        let base = (*visible_month).clone();
                        let month_number = u16::from(base.month) + u16::from(offset);
                        let display_month = (month_number % 12) as u8;
                        let display_year = base.year + (month_number / 12) as i32;
                        let num_days = days_in_month(display_year, display_month);
                        let start_dow = first_day_of_month(display_year, display_month);
                        let offset_dow = (start_dow + 7 - first_day_of_week) % 7;
                        let reordered_days: Vec<&str> = (0..7)
                            .map(|index| day_names[((first_day_of_week + index) % 7) as usize])
                            .collect();

                        html! {
                            <div class="calendar-month" key={offset}>
                                <div class="calendar-caption">
                                    if offset == 0 {
                                        <button
                                            type="button"
                                            class="calendar-nav-button"
                                            onclick={go_prev_month.clone()}
                                            aria-label="Previous month"
                                        >
                                            { "<" }
                                        </button>
                                    }
                                    <div class="calendar-caption-label">
                                        { format!("{} {}", month_names[display_month as usize], display_year) }
                                    </div>
                                    if offset == props.number_of_months - 1 {
                                        <button
                                            type="button"
                                            class="calendar-nav-button"
                                            onclick={go_next_month.clone()}
                                            aria-label="Next month"
                                        >
                                            { ">" }
                                        </button>
                                    }
                                </div>
                                <table class="calendar-table" role="grid">
                                    <thead>
                                        <tr class="calendar-head-row">
                                            if props.show_week_numbers {
                                                <th class="calendar-head-cell">{ "Wk" }</th>
                                            }
                                            {
                                                reordered_days.iter().map(|day| {
                                                    html! { <th key={*day} class="calendar-head-cell">{ day }</th> }
                                                }).collect::<Html>()
                                            }
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {
                                            (0..6).map(|week| {
                                                let start_day = week * 7;
                                                let row_start_day = start_day + 1 - i32::from(offset_dow);
                                                let week_number_label = (props.show_week_numbers
                                                    && row_start_day <= i32::from(num_days))
                                                .then(|| row_week_number(display_year, display_month, row_start_day));
                                                html! {
                                                    <tr key={week} class="calendar-row">
                                                        if props.show_week_numbers {
                                                            <td class="calendar-cell calendar-week-number">
                                                                { week_number_label.unwrap_or_default() }
                                                            </td>
                                                        }
                                                        {
                                                            (0..7).map(|day_offset| {
                                                                let cell_index = start_day + day_offset;
                                                                let day_number = cell_index + 1 - i32::from(offset_dow);
                                                                if day_number < 1 || day_number > i32::from(num_days) {
                                                                    html! {
                                                                        <td key={day_offset} class="calendar-cell">
                                                                            <div class="calendar-day outside" aria-hidden="true" />
                                                                        </td>
                                                                    }
                                                                } else {
                                                                    let day = day_number as u8;
                                                                    let date_str = iso_date(display_year, display_month, day);
                                                                    let disabled = is_date_disabled(
                                                                        &date_str,
                                                                        &props.min_date,
                                                                        &props.max_date,
                                                                        &props.disabled_dates,
                                                                    );
                                                                    let selected = is_selected(&props.mode, &selection, &date_str);
                                                                    let today = display_year == today_year
                                                                        && display_month == today_month
                                                                        && day == today_day;
                                                                    let is_boundary = matches!(props.mode, CalendarMode::Range)
                                                                        && is_range_boundary(&selection, &date_str);
                                                                    html! {
                                                                        <td key={date_str.clone()} class="calendar-cell">
                                                                            <button
                                                                                type="button"
                                                                                class={classes!(
                                                                                    "calendar-day",
                                                                                    selected.then_some("selected"),
                                                                                    today.then_some("today"),
                                                                                    disabled.then_some("calendar-day-disabled"),
                                                                                    is_boundary.then_some("calendar-day-range-boundary"),
                                                                                )}
                                                                                disabled={disabled}
                                                                                aria-selected={selected.to_string()}
                                                                                aria-label={format!("{} {}, {}", month_names[display_month as usize], day, display_year)}
                                                                                onclick={{
                                                                                    let onselect = props.onselect.clone();
                                                                                    let mode = props.mode.clone();
                                                                                    let selection = selection.clone();
                                                                                    let date_str = date_str.clone();
                                                                                    Callback::from(move |_: MouseEvent| {
                                                                                        if let Some(callback) = onselect.as_ref() {
                                                                                            callback.emit(next_selection(&mode, &selection, &date_str));
                                                                                        }
                                                                                    })
                                                                                }}
                                                                            >
                                                                                { day }
                                                                            </button>
                                                                        </td>
                                                                    }
                                                                }
                                                            }).collect::<Html>()
                                                        }
                                                    </tr>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </tbody>
                                </table>
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
    fn test_parse_iso_date() {
        assert_eq!(parse_iso_date("2024-01-15"), Some((2024, 0, 15)));
    }

    #[test]
    fn test_parse_selection_multiple() {
        let selection = parse_selection(
            &CalendarMode::Multiple,
            Some(&AttrValue::from("2024-01-01,2024-01-03")),
        );
        assert_eq!(selection.multiple.len(), 2);
    }

    #[test]
    fn test_next_selection_range_start() {
        let selection = CalendarSelection {
            single: None,
            multiple: Vec::new(),
            range_start: None,
            range_end: None,
        };
        assert_eq!(
            next_selection(&CalendarMode::Range, &selection, "2024-01-12"),
            "2024-01-12"
        );
    }

    #[test]
    fn test_next_selection_range_end() {
        let selection = CalendarSelection {
            single: None,
            multiple: Vec::new(),
            range_start: Some("2024-01-12".to_string()),
            range_end: None,
        };
        assert_eq!(
            next_selection(&CalendarMode::Range, &selection, "2024-01-15"),
            "2024-01-12..2024-01-15"
        );
    }

    #[test]
    fn test_days_from_civil_round_trips() {
        assert_eq!(days_from_civil(1970, 0, 1), 0);
        for days in [-1, 0, 59, 365, 11_016, 20_560, 47_482] {
            let (year, month, day) = civil_from_days(days);
            assert_eq!(days_from_civil(year, month, day), days);
        }
    }

    #[test]
    fn test_iso_week() {
        // 2021-01-03 (Sunday) belongs to 2020-W53; 2021-01-04 starts W1.
        assert_eq!(iso_week(days_from_civil(2021, 0, 3)), 53);
        assert_eq!(iso_week(days_from_civil(2021, 0, 4)), 1);
        // 2024-12-30 (Monday) is 2025-W01.
        assert_eq!(iso_week(days_from_civil(2024, 11, 30)), 1);
        assert_eq!(iso_week(days_from_civil(2026, 3, 17)), 16);
    }

    #[test]
    fn test_row_week_numbers_are_distinct() {
        // January 2024 starts on Monday; with Sunday-first rows the first row
        // has one leading blank cell (row_start_day = 0).
        let weeks: Vec<u8> = (0..5)
            .map(|row| row_week_number(2024, 0, row * 7))
            .collect();
        assert_eq!(weeks, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2024, 1), 29);
        assert_eq!(days_in_month(2023, 1), 28);
    }

    #[test]
    fn test_first_day_of_month() {
        assert_eq!(first_day_of_month(2024, 0), 1);
    }
}
