//! Radio component
//!
//! A control that allows the user to select a single option from a group.
//!
//! # Examples
//!
//! Inside a [`RadioGroup`], each [`Radio`] reads its checked state, `name` and
//! `disabled` from the group, and the group reports the selected value.
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Radio, RadioGroup, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let selected = use_state(|| AttrValue::from("comfortable"));
//!
//!     let on_value_change = {
//!         let selected = selected.clone();
//!         Callback::from(move |value: AttrValue| selected.set(value))
//!     };
//!
//!     html! {
//!         <RadioGroup name="density" value={(*selected).clone()} {on_value_change}>
//!             <div class="flex items-center space-x-2">
//!                 <Radio id="r1" value="default" />
//!                 <Label html_for="r1">{ "Default" }</Label>
//!             </div>
//!             <div class="flex items-center space-x-2">
//!                 <Radio id="r2" value="comfortable" />
//!                 <Label html_for="r2">{ "Comfortable" }</Label>
//!             </div>
//!         </RadioGroup>
//!     }
//! }
//! ```
//!
//! A standalone [`Radio`] still works like a native radio input:
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::Radio;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <>
//!             <Radio name="plan" value="free" default_checked={true} />
//!             <Radio name="plan" value="pro" />
//!         </>
//!     }
//! }
//! ```

use crate::types::Size;
use crate::utils::class_names;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Radio component properties
#[derive(Properties, PartialEq, Clone)]
pub struct RadioProps {
    /// Checked state for a standalone radio. `Some` makes it controlled;
    /// `None` leaves it uncontrolled. Ignored inside a [`RadioGroup`], where
    /// the group's value decides.
    #[prop_or_default]
    pub checked: Option<bool>,

    /// Initial checked state for an uncontrolled standalone radio
    #[prop_or(false)]
    pub default_checked: bool,

    /// Radio size
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Disabled state (combined with the group's `disabled`)
    #[prop_or(false)]
    pub disabled: bool,

    /// Required field (combined with the group's `required`)
    #[prop_or(false)]
    pub required: bool,

    /// Error state
    #[prop_or(false)]
    pub error: bool,

    /// Name attribute (groups radios together). Inside a [`RadioGroup`] the
    /// group's name is used.
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
/// Place radios inside a [`RadioGroup`] so they share the group's name, value
/// and disabled state. A standalone radio groups with others by `name`, like a
/// native `<input type="radio">`.
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
        default_checked,
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

    let group = use_context::<RadioGroupContext>();

    // The rendered input, remembered across renders. An uncontrolled
    // standalone radio can be unchecked by a sibling without a change event
    // of its own, so its current state is read back from the DOM.
    let element = use_mut_ref(|| None::<HtmlInputElement>);
    {
        let element = element.clone();
        let node_ref = node_ref.clone();
        use_effect(move || {
            *element.borrow_mut() = node_ref.cast::<HtmlInputElement>();
        });
    }

    let is_checked = match (&group, checked) {
        (Some(ctx), _) => value.is_some() && ctx.value == value,
        (None, Some(controlled)) => controlled,
        (None, None) => element
            .borrow()
            .as_ref()
            .map(HtmlInputElement::checked)
            .unwrap_or(default_checked),
    };
    // Group state wins over the item's own `checked`; a lone radio only
    // follows `checked` when it is controlled.
    let controlled_checked = match &group {
        Some(_) => Some(is_checked),
        None => checked,
    };

    let name = group.as_ref().map(|ctx| ctx.name.clone()).or(name);
    let disabled = disabled || group.as_ref().is_some_and(|ctx| ctx.disabled);
    let required = required || group.as_ref().is_some_and(|ctx| ctx.required);

    let handle_change = {
        let value = value.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(callback) = onchange.as_ref() {
                callback.emit(e);
            }
            if let (Some(ctx), Some(value)) = (group.as_ref(), value.as_ref()) {
                ctx.on_select.emit(value.clone());
            }
            // A controlled radio shows the owner's value until the owner
            // re-renders with a new one, so undo the browser's own change.
            if let Some(controlled) = controlled_checked {
                input.set_checked(controlled);
            }
        })
    };

    // Build class names
    let classes = class_names(&[
        Some("radio"),
        Some(size.to_class()),
        if error { Some("radio-error") } else { None },
        if disabled {
            Some("radio-disabled")
        } else {
            None
        },
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
            checked={is_checked}
            disabled={disabled}
            required={required}
            name={name}
            value={value}
            id={id}
            onchange={handle_change}
            onfocus={onfocus}
            onblur={onblur}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            aria-invalid={aria_invalid_value}
            style={style}
        />
    }
}

/// Type alias for [`Radio`] to match upstream shadcn/ui naming
pub type RadioGroupItem = Radio;

/// Type alias for [`RadioProps`] to match upstream shadcn/ui naming
pub type RadioGroupItemProps = RadioProps;

/// State a [`RadioGroup`] shares with the [`Radio`] items inside it
#[derive(Clone, PartialEq)]
pub struct RadioGroupContext {
    /// Name given to every radio in the group
    pub name: AttrValue,
    /// Currently selected value
    pub value: Option<AttrValue>,
    /// Whether the whole group is disabled
    pub disabled: bool,
    /// Whether a selection is required
    pub required: bool,
    /// Called by an item when the user selects it
    pub on_select: Callback<AttrValue>,
    /// Bumped when a controlled group rejects a selection, so items re-render
    /// and restore the owner's value in the DOM.
    revision: u32,
}

/// Radio group component properties
#[derive(Properties, PartialEq, Clone)]
pub struct RadioGroupProps {
    /// Name for all radios in this group
    pub name: AttrValue,

    /// Currently selected value. `Some` makes the group controlled.
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Initially selected value for an uncontrolled group (`value` unset)
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

    /// Called with the newly selected value
    #[prop_or_default]
    pub on_value_change: Option<Callback<AttrValue>>,

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
/// A container for grouping related radio buttons. Radios inside it take
/// their `name`, checked state and `disabled` from the group.
///
/// # Controlled and uncontrolled
/// Pass `value` to control the selection from the parent, or leave it unset and
/// use `default_value` for the initial selection. Selecting an item emits
/// `on_value_change` (and the older `onchange`) with the item's value.
///
/// # Accessibility
/// - Uses role="radiogroup"
/// - Supports ARIA labels
/// - Arrow keys move between items (native radios sharing the group's name)
#[function_component(RadioGroup)]
pub fn radio_group(props: &RadioGroupProps) -> Html {
    let RadioGroupProps {
        name,
        value,
        default_value,
        disabled,
        required,
        onchange,
        on_value_change,
        class,
        aria_label,
        children,
    } = props.clone();

    let internal_value = use_state(|| default_value);
    let revision = use_state(|| 0u32);
    let is_controlled = value.is_some();
    let current = if is_controlled {
        value
    } else {
        (*internal_value).clone()
    };

    let on_select = {
        let internal_value = internal_value.clone();
        let revision = revision.clone();
        Callback::from(move |selected: AttrValue| {
            if is_controlled {
                revision.set(revision.wrapping_add(1));
            } else {
                internal_value.set(Some(selected.clone()));
            }
            if let Some(callback) = onchange.as_ref() {
                callback.emit(selected.to_string());
            }
            if let Some(callback) = on_value_change.as_ref() {
                callback.emit(selected);
            }
        })
    };

    let context = RadioGroupContext {
        name,
        value: current,
        disabled,
        required,
        on_select,
        revision: *revision,
    };

    let classes: Classes = vec![Classes::from("radio-group"), class]
        .into_iter()
        .collect();

    html! {
        <ContextProvider<RadioGroupContext> {context}>
            <div
                class={classes}
                role="radiogroup"
                aria-label={aria_label}
                aria-disabled={disabled.then_some("true")}
                aria-required={required.then_some("true")}
            >
                { children }
            </div>
        </ContextProvider<RadioGroupContext>>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_props_default() {
        let props = yew::props!(RadioProps {});
        assert_eq!(props.checked, None);
        assert!(!props.default_checked);
        assert_eq!(props.size, Size::Md);
        assert!(!props.disabled);
    }

    #[test]
    fn test_radio_checked() {
        let props = yew::props!(RadioProps { checked: true });
        assert_eq!(props.checked, Some(true));
    }

    #[test]
    fn test_radio_group_props() {
        let props = yew::props!(RadioGroupProps {
            name: "test-group",
            default_value: "a",
            children: Children::new(vec![]),
        });
        assert_eq!(props.name, AttrValue::from("test-group"));
        assert_eq!(props.value, None);
        assert_eq!(props.default_value, Some(AttrValue::from("a")));
        assert!(props.on_value_change.is_none());
    }

    #[test]
    fn test_radio_with_value() {
        let props = yew::props!(RadioProps {
            name: "option",
            value: "value1",
        });
        assert_eq!(props.value, Some(AttrValue::from("value1")));
        assert_eq!(props.name, Some(AttrValue::from("option")));
    }
}
