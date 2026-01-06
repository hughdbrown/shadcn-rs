//! Switch component
//!
//! A toggle switch control.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Switch, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let enabled = use_state(|| false);
//!
//!     let onchange = {
//!         let enabled = enabled.clone();
//!         Callback::from(move |_| {
//!             enabled.set(!*enabled);
//!         })
//!     };
//!
//!     html! {
//!         <div class="flex items-center space-x-2">
//!             <Switch id="airplane-mode" checked={*enabled} {onchange} />
//!             <Label html_for="airplane-mode">{ "Airplane Mode" }</Label>
//!         </div>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::types::Size;
use crate::utils::class_names;

/// Switch component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SwitchProps {
    /// Checked state
    #[prop_or(false)]
    pub checked: bool,

    /// Default checked state (for uncontrolled switches)
    #[prop_or(false)]
    pub default_checked: bool,

    /// Switch size
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Required field
    #[prop_or(false)]
    pub required: bool,

    /// Name attribute
    #[prop_or_default]
    pub name: Option<AttrValue>,

    /// Value attribute
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Change event handler
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,

    /// Focus event handler
    #[prop_or_default]
    pub onfocus: Option<Callback<FocusEvent>>,

    /// Blur event handler
    #[prop_or_default]
    pub onblur: Option<Callback<FocusEvent>>,

    /// ARIA label
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// ARIA described by
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// Node ref (for the button element)
    #[prop_or_default]
    pub node_ref: NodeRef,
}

/// Switch component
///
/// A toggle switch component that acts like a checkbox but with a switch UI.
///
/// # States
/// - Off: Not enabled (unchecked)
/// - On: Enabled (checked)
/// - Disabled: Non-interactive
///
/// # Accessibility
/// - Uses `role="switch"`
/// - Supports ARIA attributes
/// - Keyboard navigation (Space/Enter to toggle)
/// - Screen reader friendly
/// - Announces checked state changes
///
/// # Implementation
/// Unlike a checkbox, the Switch is implemented as a button with role="switch"
/// for better semantic meaning and accessibility.
#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let SwitchProps {
        checked,
        default_checked,
        size,
        disabled,
        required,
        name,
        value,
        id,
        onchange,
        onfocus,
        onblur,
        aria_label,
        aria_describedby,
        class,
        style,
        node_ref,
    } = props.clone();

    // Internal state for uncontrolled mode
    let internal_checked = use_state(|| default_checked);

    // Use controlled value if provided, otherwise use internal state
    let is_checked = if checked { checked } else { *internal_checked };

    // Handle click events
    let onclick = {
        let internal_checked = internal_checked.clone();
        let onchange = onchange.clone();
        Callback::from(move |e: MouseEvent| {
            if !disabled {
                let new_state = !*internal_checked;
                internal_checked.set(new_state);
                if let Some(callback) = onchange.as_ref() {
                    let event: Event = e.into();
                    callback.emit(event);
                }
            }
        })
    };

    // Handle keyboard events (Space/Enter)
    let onkeydown = {
        let internal_checked = internal_checked.clone();
        let onchange = onchange.clone();
        Callback::from(move |e: KeyboardEvent| {
            if !disabled {
                let key = e.key();
                if key == " " || key == "Enter" {
                    e.prevent_default();
                    let new_state = !*internal_checked;
                    internal_checked.set(new_state);
                    if let Some(callback) = onchange.as_ref() {
                        let event: Event = e.into();
                        callback.emit(event);
                    }
                }
            }
        })
    };

    // Build class names
    let classes = class_names(&[
        Some("switch"),
        Some(size.to_class()),
        if is_checked { Some("switch-checked") } else { None },
        if disabled { Some("switch-disabled") } else { None },
    ]);

    // Merge with custom classes
    let final_classes: Classes = vec![classes, class].into_iter().collect();

    html! {
        <button
            ref={node_ref}
            type="button"
            role="switch"
            class={final_classes}
            aria-checked={is_checked.to_string()}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            disabled={disabled}
            onclick={onclick}
            onkeydown={onkeydown}
            onfocus={onfocus}
            onblur={onblur}
            style={style}
            id={id}
        >
            <span class="switch-thumb" aria-hidden="true"></span>
            // Hidden input for form submission
            if let Some(name_value) = name {
                <input
                    type="checkbox"
                    name={name_value}
                    value={value}
                    checked={is_checked}
                    required={required}
                    tabindex="-1"
                    style="position: absolute; pointer-events: none; opacity: 0; margin: 0;"
                    aria-hidden="true"
                />
            }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_props_default() {
        let props = SwitchProps {
            checked: false,
            default_checked: false,
            size: Size::Md,
            disabled: false,
            required: false,
            name: None,
            value: None,
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert!(!props.checked);
        assert_eq!(props.size, Size::Md);
        assert!(!props.disabled);
    }

    #[test]
    fn test_switch_checked() {
        let props = SwitchProps {
            checked: true,
            default_checked: false,
            size: Size::Md,
            disabled: false,
            required: false,
            name: None,
            value: None,
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert!(props.checked);
    }

    #[test]
    fn test_switch_disabled() {
        let props = SwitchProps {
            checked: false,
            default_checked: false,
            size: Size::Md,
            disabled: true,
            required: false,
            name: None,
            value: None,
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_switch_with_name() {
        let props = SwitchProps {
            checked: true,
            default_checked: false,
            size: Size::Md,
            disabled: false,
            required: false,
            name: Some(AttrValue::from("setting")),
            value: Some(AttrValue::from("on")),
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert_eq!(props.name, Some(AttrValue::from("setting")));
        assert_eq!(props.value, Some(AttrValue::from("on")));
    }
}
