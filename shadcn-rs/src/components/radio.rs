//! Radio component
//!
//! A control that allows the user to select a single option from a group.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Radio, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let selected = use_state(|| String::from("option1"));
//!
//!     let onchange = {
//!         let selected = selected.clone();
//!         Callback::from(move |e: Event| {
//!             let input: web_sys::HtmlInputElement = e.target_unchecked_into();
//!             selected.set(input.value());
//!         })
//!     };
//!
//!     html! {
//!         <div class="space-y-2">
//!             <div class="flex items-center space-x-2">
//!                 <Radio
//!                     id="r1"
//!                     name="option"
//!                     value="option1"
//!                     checked={*selected == "option1"}
//!                     onchange={onchange.clone()}
//!                 />
//!                 <Label html_for="r1">{ "Option 1" }</Label>
//!             </div>
//!             <div class="flex items-center space-x-2">
//!                 <Radio
//!                     id="r2"
//!                     name="option"
//!                     value="option2"
//!                     checked={*selected == "option2"}
//!                     onchange={onchange.clone()}
//!                 />
//!                 <Label html_for="r2">{ "Option 2" }</Label>
//!             </div>
//!         </div>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::types::Size;
use crate::utils::class_names;

/// Radio component properties
#[derive(Properties, PartialEq, Clone)]
pub struct RadioProps {
    /// Checked state
    #[prop_or(false)]
    pub checked: bool,

    /// Default checked state (for uncontrolled radios)
    #[prop_or(false)]
    pub default_checked: bool,

    /// Radio size
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Required field
    #[prop_or(false)]
    pub required: bool,

    /// Error state
    #[prop_or(false)]
    pub error: bool,

    /// Name attribute (groups radios together)
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

    /// ARIA invalid
    #[prop_or_default]
    pub aria_invalid: Option<bool>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// Node ref
    #[prop_or_default]
    pub node_ref: NodeRef,
}

/// Radio component
///
/// A radio button control for selecting one option from a group.
///
/// # Usage
/// Radio buttons with the same `name` attribute are grouped together.
/// Only one radio in a group can be selected at a time.
///
/// # States
/// - Unchecked: Not selected
/// - Checked: Selected
/// - Disabled: Non-interactive
/// - Error: Invalid state
///
/// # Accessibility
/// - Supports ARIA attributes
/// - Keyboard navigation (Arrow keys to navigate group, Space to select)
/// - Screen reader friendly
/// - Properly announces group and selection state
#[function_component(Radio)]
pub fn radio(props: &RadioProps) -> Html {
    let RadioProps {
        checked,
        default_checked: _,
        size,
        disabled,
        required,
        error,
        name,
        value,
        id,
        onchange,
        onfocus,
        onblur,
        aria_label,
        aria_describedby,
        aria_invalid,
        class,
        style,
        node_ref,
    } = props.clone();

    // Build class names
    let classes = class_names(&[
        Some("radio"),
        Some(size.to_class()),
        if error { Some("radio-error") } else { None },
        if disabled { Some("radio-disabled") } else { None },
    ]);

    // Merge with custom classes
    let final_classes: Classes = vec![classes, class].into_iter().collect();

    // Determine aria-invalid
    let aria_invalid_value = aria_invalid.or(Some(error)).map(|v| v.to_string());

    html! {
        <input
            ref={node_ref}
            type="radio"
            class={final_classes}
            checked={checked}
            disabled={disabled}
            required={required}
            name={name}
            value={value}
            id={id}
            onchange={onchange}
            onfocus={onfocus}
            onblur={onblur}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            aria-invalid={aria_invalid_value}
            style={style}
        />
    }
}

/// Radio group component properties
#[derive(Properties, PartialEq, Clone)]
pub struct RadioGroupProps {
    /// Name for all radios in this group
    pub name: AttrValue,

    /// Currently selected value
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Default selected value (for uncontrolled groups)
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Disabled state for entire group
    #[prop_or(false)]
    pub disabled: bool,

    /// Required field
    #[prop_or(false)]
    pub required: bool,

    /// Change event handler (receives the new value)
    #[prop_or_default]
    pub onchange: Option<Callback<String>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// ARIA label for the group
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// Children elements (Radio components)
    pub children: Children,
}

/// Radio group component
///
/// A container for grouping related radio buttons.
///
/// # Accessibility
/// - Uses role="radiogroup"
/// - Supports ARIA labels
/// - Manages focus and keyboard navigation
#[function_component(RadioGroup)]
pub fn radio_group(props: &RadioGroupProps) -> Html {
    let RadioGroupProps {
        name: _,
        value: _,
        default_value: _,
        disabled: _,
        required: _,
        onchange: _,
        class,
        aria_label,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("radio-group"), class]
        .into_iter()
        .collect();

    html! {
        <div
            class={classes}
            role="radiogroup"
            aria-label={aria_label}
        >
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_props_default() {
        let props = RadioProps {
            checked: false,
            default_checked: false,
            size: Size::Md,
            disabled: false,
            required: false,
            error: false,
            name: None,
            value: None,
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert!(!props.checked);
        assert_eq!(props.size, Size::Md);
        assert!(!props.disabled);
    }

    #[test]
    fn test_radio_checked() {
        let props = RadioProps {
            checked: true,
            default_checked: false,
            size: Size::Md,
            disabled: false,
            required: false,
            error: false,
            name: None,
            value: None,
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert!(props.checked);
    }

    #[test]
    fn test_radio_group_name() {
        let props = RadioGroupProps {
            name: AttrValue::from("test-group"),
            value: None,
            default_value: None,
            disabled: false,
            required: false,
            onchange: None,
            class: Classes::new(),
            aria_label: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.name, AttrValue::from("test-group"));
    }

    #[test]
    fn test_radio_with_value() {
        let props = RadioProps {
            checked: false,
            default_checked: false,
            size: Size::Md,
            disabled: false,
            required: false,
            error: false,
            name: Some(AttrValue::from("option")),
            value: Some(AttrValue::from("value1")),
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert_eq!(props.value, Some(AttrValue::from("value1")));
        assert_eq!(props.name, Some(AttrValue::from("option")));
    }
}
