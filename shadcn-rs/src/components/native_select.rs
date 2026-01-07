//! Native Select component
//!
//! A styled native HTML select element with consistent design system integration.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{NativeSelect, NativeSelectOption, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <div>
//!             <Label html_for="status">{ "Select status" }</Label>
//!             <NativeSelect id="status">
//!                 <NativeSelectOption value="todo">{ "Todo" }</NativeSelectOption>
//!                 <NativeSelectOption value="in-progress">{ "In Progress" }</NativeSelectOption>
//!                 <NativeSelectOption value="done">{ "Done" }</NativeSelectOption>
//!             </NativeSelect>
//!         </div>
//!     }
//! }
//! ```

use crate::types::Size;
use crate::utils::class_names;
use yew::prelude::*;

/// Native Select component properties
#[derive(Properties, PartialEq, Clone)]
pub struct NativeSelectProps {
    /// Selected value
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Default value (for uncontrolled selects)
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Select size
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

    /// ARIA labelled by
    #[prop_or_default]
    pub aria_labelledby: Option<AttrValue>,

    /// ARIA invalid (for validation errors)
    #[prop_or_default]
    pub aria_invalid: Option<bool>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Node ref
    #[prop_or_default]
    pub node_ref: NodeRef,

    /// Children elements (NativeSelectOption and NativeSelectOptGroup)
    pub children: Children,
}

/// Native Select component
///
/// A styled native HTML select element with consistent design system integration.
///
/// # Features
/// - Native browser behavior
/// - Mobile-optimized dropdowns
/// - Consistent styling with design system
/// - Keyboard navigation
/// - Full accessibility support
///
/// # When to use
/// - Need native browser behavior
/// - Better performance required
/// - Mobile-optimized dropdowns
///
/// # When to use Select instead
/// - Custom styling needed
/// - Custom animations
/// - Complex interactions
#[function_component(NativeSelect)]
pub fn native_select(props: &NativeSelectProps) -> Html {
    let NativeSelectProps {
        value,
        default_value: _,
        size,
        disabled,
        required,
        name,
        id,
        onchange,
        onfocus,
        onblur,
        aria_label,
        aria_labelledby,
        aria_invalid,
        class,
        node_ref,
        children,
    } = props.clone();

    let is_invalid = aria_invalid.unwrap_or(false);

    // Build class names
    let classes = class_names(&[
        Some("native-select"),
        Some(size.to_class()),
        if disabled {
            Some("native-select-disabled")
        } else {
            None
        },
        if is_invalid {
            Some("native-select-invalid")
        } else {
            None
        },
    ]);

    // Merge with custom classes
    let final_classes: Classes = vec![classes, class].into_iter().collect();

    html! {
        <div class="native-select-wrapper">
            <select
                ref={node_ref}
                class={final_classes}
                value={value}
                disabled={disabled}
                required={required}
                name={name}
                id={id}
                onchange={onchange}
                onfocus={onfocus}
                onblur={onblur}
                aria-label={aria_label}
                aria-labelledby={aria_labelledby}
                aria-invalid={aria_invalid.map(|v| v.to_string())}
            >
                { children }
            </select>
            <span class="native-select-chevron" aria-hidden="true">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="m6 9 6 6 6-6"/>
                </svg>
            </span>
        </div>
    }
}

/// Native Select Option properties
#[derive(Properties, PartialEq, Clone)]
pub struct NativeSelectOptionProps {
    /// Option value
    #[prop_or_default]
    pub value: AttrValue,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Selected state (for default selection)
    #[prop_or(false)]
    pub selected: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children (option text)
    pub children: Children,
}

/// Native Select Option component
///
/// Represents an individual option within the native select.
#[function_component(NativeSelectOption)]
pub fn native_select_option(props: &NativeSelectOptionProps) -> Html {
    let NativeSelectOptionProps {
        value,
        disabled,
        selected,
        class,
        children,
    } = props.clone();

    html! {
        <option
            value={value}
            disabled={disabled}
            selected={selected}
            class={class}
        >
            { children }
        </option>
    }
}

/// Native Select OptGroup properties
#[derive(Properties, PartialEq, Clone)]
pub struct NativeSelectOptGroupProps {
    /// Group label
    pub label: AttrValue,

    /// Disabled state (disables all options in group)
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children (NativeSelectOption elements)
    pub children: Children,
}

/// Native Select OptGroup component
///
/// Groups related options together for better organization.
#[function_component(NativeSelectOptGroup)]
pub fn native_select_opt_group(props: &NativeSelectOptGroupProps) -> Html {
    let NativeSelectOptGroupProps {
        label,
        disabled,
        class,
        children,
    } = props.clone();

    html! {
        <optgroup
            label={label}
            disabled={disabled}
            class={class}
        >
            { children }
        </optgroup>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_select_props_default() {
        let props = NativeSelectProps {
            value: None,
            default_value: None,
            size: Size::Md,
            disabled: false,
            required: false,
            name: None,
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_labelledby: None,
            aria_invalid: None,
            class: Classes::new(),
            node_ref: NodeRef::default(),
            children: Children::default(),
        };
        assert!(!props.disabled);
        assert!(!props.required);
        assert!(props.value.is_none());
    }

    #[test]
    fn test_native_select_disabled() {
        let props = NativeSelectProps {
            value: None,
            default_value: None,
            size: Size::Md,
            disabled: true,
            required: false,
            name: None,
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_labelledby: None,
            aria_invalid: None,
            class: Classes::new(),
            node_ref: NodeRef::default(),
            children: Children::default(),
        };
        assert!(props.disabled);
    }

    #[test]
    fn test_native_select_invalid() {
        let props = NativeSelectProps {
            value: None,
            default_value: None,
            size: Size::Md,
            disabled: false,
            required: false,
            name: None,
            id: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_labelledby: None,
            aria_invalid: Some(true),
            class: Classes::new(),
            node_ref: NodeRef::default(),
            children: Children::default(),
        };
        assert_eq!(props.aria_invalid, Some(true));
    }

    #[test]
    fn test_native_select_option_props() {
        let props = NativeSelectOptionProps {
            value: "test".into(),
            disabled: false,
            selected: false,
            class: Classes::new(),
            children: Children::default(),
        };
        assert_eq!(props.value.as_str(), "test");
        assert!(!props.disabled);
    }

    #[test]
    fn test_native_select_optgroup_props() {
        let props = NativeSelectOptGroupProps {
            label: "Group".into(),
            disabled: false,
            class: Classes::new(),
            children: Children::default(),
        };
        assert_eq!(props.label.as_str(), "Group");
        assert!(!props.disabled);
    }
}
