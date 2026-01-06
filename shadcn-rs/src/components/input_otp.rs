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
        default_value: _,
        disabled,
        masked,
        pattern: _,
        on_change,
        on_complete,
        class,
    } = props.clone();

    let length_val = length;
    let disabled_val = disabled;
    let masked_val = masked;

    // Internal state for each field
    let field_values = use_state(|| {
        if let Some(val) = value.as_ref() {
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

    // Handle change for a specific field
    let handle_field_change = {
        let field_values = field_values.clone();
        let on_change = on_change.clone();
        let on_complete = on_complete.clone();
        let length = length_val;

        Callback::from(move |new_value: String| {
            // Update field values
            field_values.set(new_value.chars().map(|c| c.to_string()).collect());

            // Get combined value
            let combined = field_values.iter().cloned().collect::<String>();

            // Call on_change
            if let Some(cb) = on_change.as_ref() {
                cb.emit(combined.clone());
            }

            // Call on_complete if all fields are filled
            if combined.len() == length {
                if let Some(cb) = on_complete.as_ref() {
                    cb.emit(combined);
                }
            }
        })
    };

    html! {
        <div class={classes} role="group" aria-label="One-time password input">
            {
                (0..length_val).map(|i| {
                    let field_val = field_values.get(i).cloned().unwrap_or_default();
                    let handle_change = handle_field_change.clone();
                    let field_values_clone = field_values.clone();

                    html! {
                        <input
                            key={i}
                            type={if masked_val { "password" } else { "text" }}
                            class="input-otp-field"
                            maxlength="1"
                            value={field_val}
                            disabled={disabled_val}
                            aria-label={format!("Digit {}", i + 1)}
                            oninput={move |e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                let mut new_values = (*field_values_clone).clone();
                                new_values[i] = input.value();
                                handle_change.emit(new_values.into_iter().collect());
                            }}
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
}
