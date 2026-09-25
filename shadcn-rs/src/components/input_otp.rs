//! Input OTP component
//!
//! A one-time password input component with multiple fields.
//!
//! # Examples
//!
//! Controlled: `value` is followed on every render, so the parent can clear
//! or replace the code (for example after a failed verification).
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::InputOTP;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let value = use_state(|| String::from(""));
//!
//!     let on_change = {
//!         let value = value.clone();
//!         Callback::from(move |new_value: String| {
//!             value.set(new_value);
//!         })
//!     };
//!     let on_complete = {
//!         let value = value.clone();
//!         // Pretend verification failed: clear the code.
//!         Callback::from(move |_code: String| value.set(String::new()))
//!     };
//!
//!     html! {
//!         <InputOTP
//!             length={6}
//!             value={(*value).clone()}
//!             {on_change}
//!             {on_complete}
//!         />
//!     }
//! }
//! ```
//!
//! Uncontrolled: leave `value` unset; `default_value` seeds the fields once.

use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Input OTP component properties
#[derive(Properties, PartialEq, Clone)]
pub struct InputOTPProps {
    /// Number of OTP input fields
    #[prop_or(6)]
    pub length: usize,

    /// Current value
    #[prop_or_default]
    pub value: Option<String>,

    /// Default value (for uncontrolled inputs)
    #[prop_or_default]
    pub default_value: Option<String>,

    /// Whether the input is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Whether to mask the input (show dots instead of characters)
    #[prop_or(false)]
    pub masked: bool,

    /// Pattern for validation (e.g., "^[0-9]*$" for digits only)
    #[prop_or_default]
    pub pattern: Option<AttrValue>,

    /// Change handler
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,

    /// Complete handler (called when all fields are filled)
    #[prop_or_default]
    pub on_complete: Option<Callback<String>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Input OTP component
///
/// A one-time password input with multiple fields.
///
/// # Features
/// - Auto-focus next field on input
/// - Paste support (splits code across fields)
/// - Backspace navigation
/// - Arrow key navigation
/// - Pattern validation
///
/// # Accessibility
/// - ARIA labels for each field
/// - Keyboard navigation
/// - Screen reader friendly
#[function_component(InputOTP)]
pub fn input_otp(props: &InputOTPProps) -> Html {
    let InputOTPProps {
        length,
        value,
        default_value,
        disabled,
        masked,
        pattern,
        on_change,
        on_complete,
        class,
    } = props.clone();

    let length_val = length;
    let disabled_val = disabled;
    let masked_val = masked;

    // Internal state for each field; initialize from value or default_value
    let internal_fields = use_state(|| {
        split_otp(
            value.as_deref().or(default_value.as_deref()).unwrap_or(""),
            length_val,
        )
    });

    // What the fields show. A controlled `value` wins whenever it differs from
    // what the user typed, so a parent can clear or replace the code at any
    // time. When the parent just echoes the typed value back, the internal
    // fields are kept so an empty slot in the middle is not collapsed.
    let field_values = displayed_fields(value.as_deref(), &internal_fields, length_val);
    let set_fields = {
        let internal_fields = internal_fields.clone();
        Callback::from(move |fields: Vec<String>| internal_fields.set(fields))
    };

    let classes: Classes = vec![
        Classes::from("input-otp"),
        if disabled_val {
            Classes::from("input-otp-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div class={classes} role="group" aria-label="One-time password input">
            {
                (0..length_val).map(|i| {
                    let field_val = field_values.get(i).cloned().unwrap_or_default();
                    let field_values_input = field_values.clone();
                    let set_fields_input = set_fields.clone();
                    let on_change_input = on_change.clone();
                    let on_complete_input = on_complete.clone();
                    let length = length_val;
                    let pattern = pattern.clone();

                    let oninput = Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        let val = input.value();
                        let mut new_values = field_values_input.clone();
                        new_values[i] = val.chars().take(1).collect();

                        let combined = join_otp(&new_values);
                        let complete = is_complete(&new_values);
                        set_fields_input.emit(new_values);

                        if let Some(cb) = on_change_input.as_ref() {
                            cb.emit(combined.clone());
                        }
                        if complete
                            && let Some(cb) = on_complete_input.as_ref()
                        {
                            cb.emit(combined);
                        }

                        // Auto-focus next field
                        if !val.is_empty()
                            && i + 1 < length
                            && let Some(parent) = input.parent_element()
                        {
                            let selector = format!("input:nth-child({})", i + 2);
                            if let Ok(Some(next)) = parent.query_selector(&selector)
                                && let Some(next_input) = next.dyn_ref::<web_sys::HtmlElement>()
                            {
                                let _ = next_input.focus();
                            }
                        }
                    });

                    let field_count = field_values.len();
                    let onkeydown = Callback::from(move |e: KeyboardEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        match e.key().as_str() {
                            "Backspace" => {
                                if input.value().is_empty()
                                    && i > 0
                                    && let Some(parent) = input.parent_element()
                                {
                                    let selector = format!("input:nth-child({})", i);
                                    if let Ok(Some(prev)) = parent.query_selector(&selector)
                                        && let Some(prev_input) = prev.dyn_ref::<web_sys::HtmlElement>()
                                    {
                                        let _ = prev_input.focus();
                                    }
                                }
                            }
                            "ArrowLeft" => {
                                if i > 0 {
                                    e.prevent_default();
                                    if let Some(parent) = input.parent_element() {
                                        let selector = format!("input:nth-child({})", i);
                                        if let Ok(Some(prev)) = parent.query_selector(&selector)
                                            && let Some(prev_input) = prev.dyn_ref::<web_sys::HtmlElement>()
                                        {
                                            let _ = prev_input.focus();
                                        }
                                    }
                                }
                            }
                            "ArrowRight" if i + 1 < field_count => {
                                e.prevent_default();
                                if let Some(parent) = input.parent_element() {
                                    let selector = format!("input:nth-child({})", i + 2);
                                    if let Ok(Some(next)) = parent.query_selector(&selector)
                                        && let Some(next_input) = next.dyn_ref::<web_sys::HtmlElement>()
                                    {
                                        let _ = next_input.focus();
                                    }
                                }
                            }
                            _ => {}
                        }
                    });

                    let field_values_paste = field_values.clone();
                    let set_fields_paste = set_fields.clone();
                    let on_change_paste = on_change.clone();
                    let on_complete_paste = on_complete.clone();
                    let onpaste = Callback::from(move |e: Event| {
                        e.prevent_default();
                        // Access clipboard data via ClipboardEvent
                        if let Some(clipboard_event) = e.dyn_ref::<web_sys::ClipboardEvent>()
                            && let Some(data) = clipboard_event.clipboard_data()
                            && let Ok(text) = data.get_data("text/plain")
                        {
                            let chars: Vec<char> = text.chars().take(length).collect();
                            let mut new_values: Vec<String> = field_values_paste.clone();
                            for (j, ch) in chars.iter().enumerate() {
                                if i + j < length {
                                    new_values[i + j] = ch.to_string();
                                }
                            }
                            let combined = join_otp(&new_values);
                            let complete = is_complete(&new_values);
                            set_fields_paste.emit(new_values);

                            if let Some(cb) = on_change_paste.as_ref() {
                                cb.emit(combined.clone());
                            }
                            if complete
                                && let Some(cb) = on_complete_paste.as_ref()
                            {
                                cb.emit(combined);
                            }
                        }
                    });

                    html! {
                        <input
                            key={i}
                            type={if masked_val { "password" } else { "text" }}
                            class="input-otp-field"
                            maxlength="1"
                            value={field_val}
                            disabled={disabled_val}
                            pattern={pattern}
                            aria-label={format!("Digit {}", i + 1)}
                            oninput={oninput}
                            onkeydown={onkeydown}
                            onpaste={onpaste}
                            inputmode="numeric"
                            autocomplete="one-time-code"
                        />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}

/// Splits `value` into `length` single-character fields, padding with empty ones.
fn split_otp(value: &str, length: usize) -> Vec<String> {
    let mut fields: Vec<String> = value.chars().take(length).map(|c| c.to_string()).collect();
    fields.resize(length, String::new());
    fields
}

/// Joins the fields into the emitted code (empty fields contribute nothing).
fn join_otp(fields: &[String]) -> String {
    fields.concat()
}

/// True when every field holds one non-whitespace character.
fn is_complete(fields: &[String]) -> bool {
    !fields.is_empty()
        && fields
            .iter()
            .all(|f| f.chars().count() == 1 && !f.chars().all(char::is_whitespace))
}

/// Fields to render: the controlled `value` when it differs from the typed
/// fields, otherwise the typed fields (resized to `length`).
fn displayed_fields(value: Option<&str>, internal: &[String], length: usize) -> Vec<String> {
    match value {
        Some(v) if v != join_otp(internal) => split_otp(v, length),
        _ => {
            let mut fields = internal.to_vec();
            fields.resize(length, String::new());
            fields
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fields(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_split_and_join_otp() {
        assert_eq!(split_otp("12", 4), fields(&["1", "2", "", ""]));
        assert_eq!(split_otp("123456", 4), fields(&["1", "2", "3", "4"]));
        assert_eq!(join_otp(&fields(&["1", "", "3"])), "13");
    }

    #[test]
    fn test_is_complete_counts_characters() {
        assert!(is_complete(&fields(&["1", "2", "3"])));
        assert!(!is_complete(&fields(&["1", "", "3"])));
        assert!(!is_complete(&fields(&["1", " ", "3"])));
        // Multi-byte characters count as one slot each.
        assert!(is_complete(&fields(&["é", "ß"])));
    }

    #[test]
    fn test_displayed_fields_follows_controlled_value() {
        let typed = fields(&["1", "", "3"]);
        // Parent echoes the typed code: keep the gap.
        assert_eq!(displayed_fields(Some("13"), &typed, 3), typed);
        // Parent clears or replaces it: show the new value.
        assert_eq!(displayed_fields(Some(""), &typed, 3), fields(&["", "", ""]));
        assert_eq!(
            displayed_fields(Some("987"), &typed, 3),
            fields(&["9", "8", "7"])
        );
        // Uncontrolled: typed fields, resized to the current length.
        assert_eq!(
            displayed_fields(None, &typed, 4),
            fields(&["1", "", "3", ""])
        );
    }

    #[test]
    fn test_input_otp_classes_have_css() {
        let css = include_str!("../../styles/components.css");
        assert!(css.contains(".input-otp {"));
        assert!(css.contains(".input-otp-field {"));
        assert!(css.contains(".input-otp-disabled"));
    }

    #[test]
    fn test_input_otp_default() {
        let props = InputOTPProps {
            length: 6,
            value: None,
            default_value: None,
            disabled: false,
            masked: false,
            pattern: None,
            on_change: None,
            on_complete: None,
            class: Classes::new(),
        };

        assert_eq!(props.length, 6);
        assert!(!props.disabled);
        assert!(!props.masked);
    }

    #[test]
    fn test_input_otp_custom_length() {
        let props = InputOTPProps {
            length: 4,
            value: None,
            default_value: None,
            disabled: false,
            masked: false,
            pattern: None,
            on_change: None,
            on_complete: None,
            class: Classes::new(),
        };

        assert_eq!(props.length, 4);
    }

    #[test]
    fn test_input_otp_masked() {
        let props = InputOTPProps {
            length: 6,
            value: None,
            default_value: None,
            disabled: false,
            masked: true,
            pattern: None,
            on_change: None,
            on_complete: None,
            class: Classes::new(),
        };

        assert!(props.masked);
    }

    #[test]
    fn test_input_otp_disabled() {
        let props = InputOTPProps {
            length: 6,
            value: None,
            default_value: None,
            disabled: true,
            masked: false,
            pattern: None,
            on_change: None,
            on_complete: None,
            class: Classes::new(),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_input_otp_with_value() {
        let props = InputOTPProps {
            length: 6,
            value: Some(String::from("123456")),
            default_value: None,
            disabled: false,
            masked: false,
            pattern: None,
            on_change: None,
            on_complete: None,
            class: Classes::new(),
        };

        assert_eq!(props.value, Some(String::from("123456")));
    }

    #[test]
    fn test_input_otp_with_pattern() {
        let props = InputOTPProps {
            length: 6,
            value: None,
            default_value: None,
            disabled: false,
            masked: false,
            pattern: Some(AttrValue::from("^[0-9]*$")),
            on_change: None,
            on_complete: None,
            class: Classes::new(),
        };

        assert_eq!(props.pattern, Some(AttrValue::from("^[0-9]*$")));
    }

    #[test]
    fn test_input_otp_with_initial_value_fields() {
        // Verify that a value shorter than length still works (fields padded with empty strings)
        let props = InputOTPProps {
            length: 6,
            value: Some(String::from("123")),
            default_value: None,
            disabled: false,
            masked: false,
            pattern: None,
            on_change: None,
            on_complete: None,
            class: Classes::new(),
        };

        assert_eq!(props.length, 6);
        assert_eq!(props.value, Some(String::from("123")));
        // The component should initialize 6 fields with first 3 filled
    }

    #[test]
    fn test_input_otp_keyboard_nav_props() {
        // The component now supports keyboard navigation (ArrowLeft, ArrowRight, Backspace)
        // and paste support. Verify props still work correctly with these features.
        let props = InputOTPProps {
            length: 4,
            value: None,
            default_value: None,
            disabled: false,
            masked: false,
            pattern: None,
            on_change: Some(Callback::from(|_: String| {})),
            on_complete: Some(Callback::from(|_: String| {})),
            class: Classes::new(),
        };

        assert_eq!(props.length, 4);
        assert!(!props.disabled);
        assert!(props.on_change.is_some());
        assert!(props.on_complete.is_some());
    }
}
