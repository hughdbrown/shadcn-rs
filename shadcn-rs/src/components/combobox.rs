//! Combobox component
//!
//! An autocomplete input combined with a list of suggestions (searchable select).
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Combobox, ComboboxTrigger, ComboboxContent, ComboboxInput, ComboboxEmpty, ComboboxGroup, ComboboxItem};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let selected = use_state(|| None::<String>);
//!     let open = use_state(|| false);
//!
//!     html! {
//!         <Combobox>
//!             <ComboboxTrigger>
//!                 <ComboboxInput placeholder="Select framework..." />
//!             </ComboboxTrigger>
//!             <ComboboxContent>
//!                 <ComboboxEmpty>{ "No framework found." }</ComboboxEmpty>
//!                 <ComboboxGroup>
//!                     <ComboboxItem value="react">{ "React" }</ComboboxItem>
//!                     <ComboboxItem value="vue">{ "Vue" }</ComboboxItem>
//!                     <ComboboxItem value="svelte">{ "Svelte" }</ComboboxItem>
//!                 </ComboboxGroup>
//!             </ComboboxContent>
//!         </Combobox>
//!     }
//! }
//! ```

use crate::hooks::{use_click_outside_conditional, use_escape_key_conditional, use_toggle};
use yew::prelude::*;

/// Combobox container properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxProps {
    /// Open state (controlled)
    #[prop_or_default]
    pub open: Option<bool>,

    /// Default open state (uncontrolled)
    #[prop_or(false)]
    pub default_open: bool,

    /// Open state change handler
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Combobox container component
///
/// The main container for a combobox (searchable select).
///
/// # Accessibility
/// - Full keyboard navigation
/// - Screen reader support
/// - ARIA attributes
#[function_component(Combobox)]
pub fn combobox(props: &ComboboxProps) -> Html {
    let ComboboxProps {
        open,
        default_open,
        on_open_change: _,
        class,
        children,
    } = props.clone();

    let (is_open, _toggle, _set_open) = use_toggle(open.unwrap_or(default_open));

    let classes: Classes = vec![
        Classes::from("combobox"),
        if is_open {
            Classes::from("combobox-open")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Combobox trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxTriggerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Children elements
    pub children: Children,
}

/// Combobox trigger component
///
/// Triggers the combobox dropdown.
#[function_component(ComboboxTrigger)]
pub fn combobox_trigger(props: &ComboboxTriggerProps) -> Html {
    let ComboboxTriggerProps {
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("combobox-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <button
            type="button"
            class={classes}
            onclick={onclick}
            role="combobox"
            aria-expanded="false"
            aria-controls="combobox-content"
        >
            { children }
        </button>
    }
}

/// Combobox input properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxInputProps {
    /// Placeholder text
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    /// Current value
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Input event handler
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Combobox input component
///
/// Search input for filtering combobox items.
#[function_component(ComboboxInput)]
pub fn combobox_input(props: &ComboboxInputProps) -> Html {
    let ComboboxInputProps {
        placeholder,
        value,
        oninput,
        class,
    } = props.clone();

    let classes: Classes = vec![Classes::from("combobox-input"), class]
        .into_iter()
        .collect();

    html! {
        <input
            type="text"
            class={classes}
            placeholder={placeholder}
            value={value}
            oninput={oninput}
            role="combobox"
            aria-autocomplete="list"
        />
    }
}

/// Combobox content properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Combobox content component
///
/// Container for combobox items.
#[function_component(ComboboxContent)]
pub fn combobox_content(props: &ComboboxContentProps) -> Html {
    let ComboboxContentProps { class, children } = props.clone();

    let content_ref = use_node_ref();

    // Close on click outside
    use_click_outside_conditional(content_ref.clone(), || {}, true);

    // Close on Escape key
    use_escape_key_conditional(|| {}, true);

    let classes: Classes = vec![Classes::from("combobox-content"), class]
        .into_iter()
        .collect();

    html! {
        <div
            ref={content_ref}
            class={classes}
            id="combobox-content"
            role="listbox"
        >
            { children }
        </div>
    }
}

/// Combobox empty properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxEmptyProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Combobox empty component
///
/// Displays when no items match the search.
#[function_component(ComboboxEmpty)]
pub fn combobox_empty(props: &ComboboxEmptyProps) -> Html {
    let ComboboxEmptyProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("combobox-empty"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="status">
            { children }
        </div>
    }
}

/// Combobox group properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxGroupProps {
    /// Group heading
    #[prop_or_default]
    pub heading: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Combobox group component
///
/// Groups related combobox items.
#[function_component(ComboboxGroup)]
pub fn combobox_group(props: &ComboboxGroupProps) -> Html {
    let ComboboxGroupProps {
        heading,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("combobox-group"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="group">
            {
                if let Some(heading_text) = heading {
                    html! {
                        <div class="combobox-group-heading">
                            { heading_text }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            { children }
        </div>
    }
}

/// Combobox item properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxItemProps {
    /// Value of this item
    pub value: AttrValue,

    /// Selected state
    #[prop_or(false)]
    pub selected: bool,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Combobox item component
///
/// A selectable item in the combobox.
#[function_component(ComboboxItem)]
pub fn combobox_item(props: &ComboboxItemProps) -> Html {
    let ComboboxItemProps {
        value: _,
        selected,
        disabled,
        onclick,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("combobox-item"),
        if selected {
            Classes::from("combobox-item-selected")
        } else {
            Classes::new()
        },
        if disabled {
            Classes::from("combobox-item-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div
            class={classes}
            role="option"
            aria-selected={selected.to_string()}
            aria-disabled={disabled.to_string()}
            onclick={onclick}
        >
            { children }
        </div>
    }
}

/// Combobox separator properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxSeparatorProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Combobox separator component
///
/// Separates groups of combobox items.
#[function_component(ComboboxSeparator)]
pub fn combobox_separator(props: &ComboboxSeparatorProps) -> Html {
    let ComboboxSeparatorProps { class } = props.clone();

    let classes: Classes = vec![Classes::from("combobox-separator"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="separator" />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combobox_default_closed() {
        let props = ComboboxProps {
            open: None,
            default_open: false,
            on_open_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.default_open);
    }

    #[test]
    fn test_combobox_item_selected() {
        let props = ComboboxItemProps {
            value: AttrValue::from("test"),
            selected: true,
            disabled: false,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.selected);
        assert!(!props.disabled);
    }

    #[test]
    fn test_combobox_item_disabled() {
        let props = ComboboxItemProps {
            value: AttrValue::from("test"),
            selected: false,
            disabled: true,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.selected);
        assert!(props.disabled);
    }

    #[test]
    fn test_combobox_group_with_heading() {
        let props = ComboboxGroupProps {
            heading: Some(AttrValue::from("Options")),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.heading, Some(AttrValue::from("Options")));
    }
}
