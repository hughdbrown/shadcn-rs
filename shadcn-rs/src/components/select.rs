//! Select component
//!
//! A dropdown select input for choosing from a list of options.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Select, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let selected = use_state(|| String::from(""));
//!
//!     let onchange = {
//!         let selected = selected.clone();
//!         Callback::from(move |e: Event| {
//!             let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
//!             selected.set(select.value());
//!         })
//!     };
//!
//!     html! {
//!         <div>
//!             <Label html_for="fruit">{ "Select a fruit" }</Label>
//!             <Select id="fruit" value={(*selected).clone()} {onchange}>
//!                 <option value="">{ "Choose..." }</option>
//!                 <option value="apple">{ "Apple" }</option>
//!                 <option value="banana">{ "Banana" }</option>
//!                 <option value="orange">{ "Orange" }</option>
//!             </Select>
//!         </div>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::types::Size;
use crate::utils::class_names;

/// Select component properties
#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps {
    /// Selected value
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Default value (for uncontrolled selects)
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Placeholder option (shown when no value selected)
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    /// Select size
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Allow multiple selections
    #[prop_or(false)]
    pub multiple: bool,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Required field
    #[prop_or(false)]
    pub required: bool,

    /// Error state
    #[prop_or(false)]
    pub error: bool,

    /// Name attribute
    #[prop_or_default]
    pub name: Option<AttrValue>,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Autocomplete attribute
    #[prop_or_default]
    pub autocomplete: Option<AttrValue>,

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

    /// Children elements (option and optgroup elements)
    pub children: Children,
}

/// Select component
///
/// A dropdown select component for choosing from a list of options.
///
/// # Features
/// - Single and multiple selection modes
/// - Grouped options (via optgroup)
/// - Keyboard navigation
/// - Accessibility support
///
/// # States
/// - Normal: Default interactive state
/// - Disabled: Non-interactive
/// - Error: Indicates validation error
/// - Required: Indicates required field
///
/// # Accessibility
/// - Supports ARIA attributes
/// - Keyboard navigation (Arrow keys, Enter, Escape)
/// - Screen reader friendly
#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let SelectProps {
        value,
        default_value: _,
        placeholder,
        size,
        multiple,
        disabled,
        required,
        error,
        name,
        id,
        autocomplete,
        onchange,
        onfocus,
        onblur,
        aria_label,
        aria_describedby,
        aria_invalid,
        class,
        style,
        node_ref,
        children,
    } = props.clone();

    // Build class names
    let classes = class_names(&[
        Some("select"),
        Some(size.to_class()),
        if error { Some("select-error") } else { None },
        if disabled { Some("select-disabled") } else { None },
    ]);

    // Merge with custom classes
    let final_classes: Classes = vec![classes, class].into_iter().collect();

    // Determine aria-invalid
    let aria_invalid_value = aria_invalid.or(Some(error)).map(|v| v.to_string());

    let has_value = value.is_some();

    html! {
        <select
            ref={node_ref}
            class={final_classes}
            value={value}
            multiple={multiple}
            disabled={disabled}
            required={required}
            name={name}
            id={id}
            autocomplete={autocomplete}
            onchange={onchange}
            onfocus={onfocus}
            onblur={onblur}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            aria-invalid={aria_invalid_value}
            style={style}
        >
            if let Some(placeholder_text) = placeholder {
                <option value="" disabled=true selected={!has_value}>
                    { placeholder_text }
                </option>
            }
            { children }
        </select>
    }
}

// Advanced Select Components (Custom Implementation with Context)

use crate::hooks::{use_escape_key_conditional, use_click_outside_conditional};

/// Context for sharing select state with children
#[derive(Clone, PartialEq)]
pub struct SelectContext {
    /// Whether the dropdown is open
    pub is_open: bool,
    /// Currently selected value
    pub selected_value: Option<AttrValue>,
    /// Display label for the selected value
    pub selected_label: Option<AttrValue>,
    /// Callback to toggle open/close state
    pub toggle_open: Callback<()>,
    /// Callback to select a value (value, label)
    pub select_value: Callback<(AttrValue, AttrValue)>,
    /// Whether the select is disabled
    pub disabled: bool,
}

/// Advanced select container properties (also used as the main Select when using compound components)
#[derive(Properties, PartialEq, Clone)]
pub struct SelectAdvancedProps {
    /// Selected value (controlled)
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Default value (for uncontrolled select)
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Whether the select is open (controlled)
    #[prop_or_default]
    pub open: Option<bool>,

    /// Whether the select is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Callback when open state changes
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Callback when value changes
    #[prop_or_default]
    pub on_value_change: Option<Callback<AttrValue>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Advanced Select container component
///
/// Container for custom select with trigger and content.
/// This component manages the open/close state and selected value.
#[function_component(SelectAdvanced)]
pub fn select_advanced(props: &SelectAdvancedProps) -> Html {
    let SelectAdvancedProps {
        value,
        default_value,
        open,
        disabled,
        on_open_change,
        on_value_change,
        class,
        children,
    } = props.clone();

    // Internal state for open/close
    let internal_open = use_state(|| false);
    let is_open = open.unwrap_or(*internal_open);

    // Internal state for selected value
    let internal_value = use_state(|| default_value.clone());
    let internal_label = use_state(|| Option::<AttrValue>::None);
    let selected_value = value.clone().or_else(|| (*internal_value).clone());
    let selected_label = (*internal_label).clone();

    let toggle_open = {
        let internal_open = internal_open.clone();
        let on_open_change = on_open_change.clone();
        Callback::from(move |_: ()| {
            let new_state = !*internal_open;
            internal_open.set(new_state);
            if let Some(callback) = on_open_change.as_ref() {
                callback.emit(new_state);
            }
        })
    };

    let select_value = {
        let internal_value = internal_value.clone();
        let internal_label = internal_label.clone();
        let internal_open = internal_open.clone();
        let on_value_change = on_value_change.clone();
        let on_open_change = on_open_change.clone();
        Callback::from(move |(val, label): (AttrValue, AttrValue)| {
            internal_value.set(Some(val.clone()));
            internal_label.set(Some(label));
            internal_open.set(false);
            if let Some(callback) = on_value_change.as_ref() {
                callback.emit(val);
            }
            if let Some(callback) = on_open_change.as_ref() {
                callback.emit(false);
            }
        })
    };

    let context = SelectContext {
        is_open,
        selected_value,
        selected_label,
        toggle_open,
        select_value,
        disabled,
    };

    let classes: Classes = vec![
        Classes::from("select-advanced"),
        if is_open { Classes::from("select-open") } else { Classes::new() },
        class
    ]
    .into_iter()
    .collect();

    html! {
        <ContextProvider<SelectContext> context={context}>
            <div class={classes}>
                { children }
            </div>
        </ContextProvider<SelectContext>>
    }
}

/// Select trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct SelectTriggerProps {
    /// Whether the trigger is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Select trigger component
///
/// The button that opens the select dropdown.
#[function_component(SelectTrigger)]
pub fn select_trigger(props: &SelectTriggerProps) -> Html {
    let SelectTriggerProps {
        disabled: prop_disabled,
        class,
        children,
    } = props.clone();

    let context = use_context::<SelectContext>();
    let is_open = context.as_ref().map(|c| c.is_open).unwrap_or(false);
    let is_disabled = prop_disabled || context.as_ref().map(|c| c.disabled).unwrap_or(false);

    let onclick = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ctx) = context.as_ref() {
                if !ctx.disabled {
                    ctx.toggle_open.emit(());
                }
            }
        })
    };

    let classes: Classes = vec![
        Classes::from("select-trigger"),
        if is_disabled {
            Classes::from("select-trigger-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <button
            type="button"
            role="combobox"
            aria-expanded={is_open.to_string()}
            aria-haspopup="listbox"
            disabled={is_disabled}
            class={classes}
            onclick={onclick}
        >
            { children }
            <svg class="select-chevron" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="m6 9 6 6 6-6"/>
            </svg>
        </button>
    }
}

/// Select value properties
#[derive(Properties, PartialEq, Clone)]
pub struct SelectValueProps {
    /// Placeholder text when no value selected
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    #[prop_or_default]
    pub children: Children,
}

/// Select value component
///
/// Displays the selected value or placeholder.
#[function_component(SelectValue)]
pub fn select_value(props: &SelectValueProps) -> Html {
    let SelectValueProps {
        placeholder,
        class,
        children,
    } = props.clone();

    let context = use_context::<SelectContext>();
    let selected_label = context.as_ref().and_then(|c| c.selected_label.clone());

    let classes: Classes = vec![Classes::from("select-value"), class]
        .into_iter()
        .collect();

    let has_children = children.iter().count() > 0;

    html! {
        <span class={classes}>
            if let Some(label) = selected_label {
                { label }
            } else if has_children {
                { children }
            } else if let Some(placeholder_text) = placeholder {
                <span class="select-placeholder">{ placeholder_text }</span>
            }
        </span>
    }
}

/// Select content properties
#[derive(Properties, PartialEq, Clone)]
pub struct SelectContentProps {
    /// Whether the content is open (controlled, overrides context)
    #[prop_or_default]
    pub open: Option<bool>,

    /// Callback to close the content
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    /// Whether to close on outside click
    #[prop_or(true)]
    pub close_on_outside_click: bool,

    /// Whether to close on Escape key
    #[prop_or(true)]
    pub close_on_escape: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Select content component
///
/// The dropdown content containing select items.
#[function_component(SelectContent)]
pub fn select_content(props: &SelectContentProps) -> Html {
    let SelectContentProps {
        open: prop_open,
        on_close,
        close_on_outside_click,
        close_on_escape,
        class,
        children,
    } = props.clone();

    let context = use_context::<SelectContext>();
    let content_ref = use_node_ref();

    // Use prop if provided, otherwise use context
    let is_open = prop_open.unwrap_or_else(|| context.as_ref().map(|c| c.is_open).unwrap_or(false));

    // Handle Escape key
    let toggle_open = context.as_ref().map(|c| c.toggle_open.clone());
    let on_close_esc = on_close.clone();
    use_escape_key_conditional(
        move || {
            if let Some(callback) = on_close_esc.as_ref() {
                callback.emit(());
            } else if let Some(toggle) = toggle_open.as_ref() {
                toggle.emit(());
            }
        },
        is_open && close_on_escape,
    );

    // Handle click outside
    let toggle_open_click = context.as_ref().map(|c| c.toggle_open.clone());
    let on_close_click = on_close.clone();
    use_click_outside_conditional(
        content_ref.clone(),
        move || {
            if let Some(callback) = on_close_click.as_ref() {
                callback.emit(());
            } else if let Some(toggle) = toggle_open_click.as_ref() {
                toggle.emit(());
            }
        },
        is_open && close_on_outside_click,
    );

    if !is_open {
        return html! {};
    }

    let classes: Classes = vec![Classes::from("select-content"), class]
        .into_iter()
        .collect();

    html! {
        <div
            ref={content_ref}
            class={classes}
            role="listbox"
        >
            { children }
        </div>
    }
}

/// Select item properties
#[derive(Properties, PartialEq, Clone)]
pub struct SelectItemProps {
    /// The value of this item
    pub value: AttrValue,

    /// Whether the item is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Whether this item is selected
    #[prop_or(false)]
    pub selected: bool,

    /// Click handler
    #[prop_or_default]
    pub on_select: Option<Callback<AttrValue>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Select item component
///
/// An individual item in the select dropdown.
#[function_component(SelectItem)]
pub fn select_item(props: &SelectItemProps) -> Html {
    let SelectItemProps {
        value,
        disabled,
        selected: prop_selected,
        on_select,
        class,
        children,
    } = props.clone();

    let context = use_context::<SelectContext>();
    let label_ref = use_node_ref();

    // Check if this item is selected via context
    let is_selected = prop_selected || context.as_ref()
        .and_then(|c| c.selected_value.as_ref())
        .map(|v| *v == value)
        .unwrap_or(false);

    let onclick = {
        let value = value.clone();
        let context = context.clone();
        let on_select = on_select.clone();
        let label_ref = label_ref.clone();
        Callback::from(move |e: MouseEvent| {
            if !disabled {
                e.prevent_default();

                // Get text content from the element for the label
                let label_text = label_ref.cast::<web_sys::Element>()
                    .and_then(|el| el.text_content())
                    .unwrap_or_else(|| value.to_string());

                // Notify context
                if let Some(ctx) = context.as_ref() {
                    ctx.select_value.emit((value.clone(), AttrValue::from(label_text)));
                }

                // Also call custom handler if provided
                if let Some(cb) = on_select.as_ref() {
                    cb.emit(value.clone());
                }
            }
        })
    };

    let classes: Classes = vec![
        Classes::from("select-item"),
        if is_selected {
            Classes::from("select-item-selected")
        } else {
            Classes::new()
        },
        if disabled {
            Classes::from("select-item-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div
            ref={label_ref}
            class={classes}
            role="option"
            aria-selected={is_selected.to_string()}
            aria-disabled={disabled.to_string()}
            onclick={onclick}
        >
            if is_selected {
                <svg class="select-item-check" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="20 6 9 17 4 12"/>
                </svg>
            }
            { children }
        </div>
    }
}

/// Select group properties
#[derive(Properties, PartialEq, Clone)]
pub struct SelectGroupProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Select group component
///
/// Groups related select items together.
#[function_component(SelectGroup)]
pub fn select_group(props: &SelectGroupProps) -> Html {
    let SelectGroupProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("select-group"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="group">
            { children }
        </div>
    }
}

/// Select label properties
#[derive(Properties, PartialEq, Clone)]
pub struct SelectLabelProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Select label component
///
/// A label for a select group.
#[function_component(SelectLabel)]
pub fn select_label(props: &SelectLabelProps) -> Html {
    let SelectLabelProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("select-label"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="presentation">
            { children }
        </div>
    }
}

/// Select separator properties
#[derive(Properties, PartialEq, Clone)]
pub struct SelectSeparatorProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Select separator component
///
/// A visual separator between select items or groups.
#[function_component(SelectSeparator)]
pub fn select_separator(props: &SelectSeparatorProps) -> Html {
    let SelectSeparatorProps { class } = props.clone();

    let classes: Classes = vec![Classes::from("select-separator"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="separator" aria-orientation="horizontal" />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_props_default() {
        let props = SelectProps {
            value: None,
            default_value: None,
            placeholder: None,
            size: Size::Md,
            multiple: false,
            disabled: false,
            required: false,
            error: false,
            name: None,
            id: None,
            autocomplete: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.size, Size::Md);
        assert!(!props.multiple);
        assert!(!props.disabled);
        assert!(!props.error);
    }

    #[test]
    fn test_select_with_value() {
        let props = SelectProps {
            value: Some(AttrValue::from("option1")),
            default_value: None,
            placeholder: None,
            size: Size::Md,
            multiple: false,
            disabled: false,
            required: false,
            error: false,
            name: None,
            id: None,
            autocomplete: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.value, Some(AttrValue::from("option1")));
    }

    #[test]
    fn test_select_multiple() {
        let props = SelectProps {
            value: None,
            default_value: None,
            placeholder: None,
            size: Size::Md,
            multiple: true,
            disabled: false,
            required: false,
            error: false,
            name: None,
            id: None,
            autocomplete: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert!(props.multiple);
    }

    #[test]
    fn test_select_with_placeholder() {
        let props = SelectProps {
            value: None,
            default_value: None,
            placeholder: Some(AttrValue::from("Choose an option")),
            size: Size::Md,
            multiple: false,
            disabled: false,
            required: false,
            error: false,
            name: None,
            id: None,
            autocomplete: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert_eq!(
            props.placeholder,
            Some(AttrValue::from("Choose an option"))
        );
    }

    #[test]
    fn test_select_states() {
        let props = SelectProps {
            value: None,
            default_value: None,
            placeholder: None,
            size: Size::Md,
            multiple: false,
            disabled: true,
            required: true,
            error: true,
            name: None,
            id: None,
            autocomplete: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
        assert!(props.required);
        assert!(props.error);
    }

    #[test]
    fn test_select_advanced_default() {
        let props = SelectAdvancedProps {
            value: None,
            default_value: None,
            open: None,
            disabled: false,
            on_open_change: None,
            on_value_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.open.is_none());
        assert!(props.value.is_none());
    }

    #[test]
    fn test_select_trigger_disabled() {
        let props = SelectTriggerProps {
            disabled: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_select_item_selected() {
        let props = SelectItemProps {
            value: AttrValue::from("option1"),
            disabled: false,
            selected: true,
            on_select: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.selected);
        assert!(!props.disabled);
        assert_eq!(props.value, AttrValue::from("option1"));
    }

    #[test]
    fn test_select_item_disabled() {
        let props = SelectItemProps {
            value: AttrValue::from("option2"),
            disabled: true,
            selected: false,
            on_select: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
        assert!(!props.selected);
    }

    #[test]
    fn test_select_value_with_placeholder() {
        let props = SelectValueProps {
            placeholder: Some(AttrValue::from("Select an option")),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.placeholder, Some(AttrValue::from("Select an option")));
    }

    #[test]
    fn test_select_content_close_behaviors() {
        let props = SelectContentProps {
            open: Some(true),
            on_close: None,
            close_on_outside_click: true,
            close_on_escape: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.open, Some(true));
        assert!(props.close_on_outside_click);
        assert!(props.close_on_escape);
    }
}
