//! Input OTP component
//!
//! A one-time password input component with multiple fields.
//!
//! # Examples
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
//!
//!     html! {
//!         <InputOTP
//!             length={6}
//!             value={(*value).clone()}
//!             {on_change}
//!         />
//!     }
//! }
//! ```

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
    let field_values = use_state(|| {
        let init = value.as_ref().or(default_value.as_ref());
        if let Some(val) = init {
            let chars: Vec<String> = val.chars().map(|c| c.to_string()).collect();
            let mut fields = vec![String::new(); length_val];
            for (i, ch) in chars.into_iter().take(length_val).enumerate() {
                fields[i] = ch;
            }
            fields
        } else {
            vec![String::new(); length_val]
        }
    });

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
                    let on_change_input = on_change.clone();
                    let on_complete_input = on_complete.clone();
                    let length = length_val;
                    let pattern = pattern.clone();

                    let oninput = Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        let val = input.value();
                        let mut new_values = (*field_values_input).clone();
                        new_values[i] = val.chars().take(1).collect();

                        let combined: String = new_values.iter().cloned().collect();
                        field_values_input.set(new_values);

                        if let Some(cb) = on_change_input.as_ref() {
                            cb.emit(combined.clone());
                        }
                        if combined.len() == length
                            && combined.chars().all(|c| !c.is_whitespace())
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

                    let field_values_key = field_values.clone();
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
                            "ArrowRight" => {
                                let len = (*field_values_key).len();
                                if i + 1 < len {
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
                            }
                            _ => {}
                        }
                    });

                    let field_values_paste = field_values.clone();
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
                            let mut new_values: Vec<String> = (*field_values_paste).clone();
                            for (j, ch) in chars.iter().enumerate() {
                                if i + j < length {
                                    new_values[i + j] = ch.to_string();
                                }
                            }
                            let combined: String = new_values.iter().cloned().collect();
                            field_values_paste.set(new_values);

                            if let Some(cb) = on_change_paste.as_ref() {
                                cb.emit(combined.clone());
                            }
                            if combined.len() == length
                                && combined.chars().all(|c| !c.is_whitespace())
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

#[cfg(test)]
mod tests {
    use super::*;

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
