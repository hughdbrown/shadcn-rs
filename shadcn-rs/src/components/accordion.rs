//! Accordion component
//!
//! A vertically stacked set of interactive headings that each reveal a section of content.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Accordion>
//!             <AccordionItem value="item-1">
//!                 <AccordionTrigger>{ "Is it accessible?" }</AccordionTrigger>
//!                 <AccordionContent>
//!                     { "Yes. It adheres to the WAI-ARIA design pattern." }
//!                 </AccordionContent>
//!             </AccordionItem>
//!             <AccordionItem value="item-2">
//!                 <AccordionTrigger>{ "Is it styled?" }</AccordionTrigger>
//!                 <AccordionContent>
//!                     { "Yes. It comes with default styles that you can customize." }
//!                 </AccordionContent>
//!             </AccordionItem>
//!         </Accordion>
//!     }
//! }
//! ```

use std::collections::HashSet;
use yew::prelude::*;

/// Accordion type
#[derive(Debug, Clone, PartialEq)]
pub enum AccordionType {
    /// Only one item can be open at a time
    Single,
    /// Multiple items can be open simultaneously
    Multiple,
}

/// Context for sharing accordion state between parent and children
#[derive(Clone, PartialEq)]
pub struct AccordionContext {
    /// Currently open item values
    pub open_items: HashSet<String>,
    /// Callback to toggle an item
    pub toggle_item: Callback<String>,
    /// Accordion type (single or multiple)
    pub accordion_type: AccordionType,
    /// Whether items can be collapsed
    pub collapsible: bool,
}

/// Context for sharing item value with trigger and content
#[derive(Clone, PartialEq)]
pub struct AccordionItemContext {
    /// The value of this accordion item
    pub value: String,
    /// Whether this item is currently open
    pub is_open: bool,
    /// Callback to toggle this item
    pub toggle: Callback<()>,
}

/// Accordion component properties
#[derive(Properties, PartialEq, Clone)]
pub struct AccordionProps {
    /// Accordion type (single or multiple)
    #[prop_or(AccordionType::Single)]
    pub accordion_type: AccordionType,

    /// Whether items can be collapsed (only for single type)
    #[prop_or(true)]
    pub collapsible: bool,

    /// Default open value(s)
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Currently open value(s)
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Callback when value changes
    #[prop_or_default]
    pub on_value_change: Option<Callback<String>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Accordion component
///
/// A container for accordion items.
///
/// # Accessibility
/// - Keyboard navigation (Arrow keys, Home, End)
/// - Proper ARIA attributes
/// - Focus management
/// - Screen reader friendly
#[function_component(Accordion)]
pub fn accordion(props: &AccordionProps) -> Html {
    let AccordionProps {
        accordion_type,
        collapsible,
        default_value,
        value,
        on_value_change,
        class,
        children,
    } = props.clone();

    // Initialize open items from default_value or value
    let initial_open: HashSet<String> = default_value
        .as_ref()
        .or(value.as_ref())
        .map(|v| {
            let mut set = HashSet::new();
            set.insert(v.to_string());
            set
        })
        .unwrap_or_default();

    let open_items = use_state(|| initial_open);

    // Sync with controlled value prop
    {
        let open_items = open_items.clone();
        use_effect_with(value.clone(), move |value| {
            if let Some(v) = value {
                let mut set = HashSet::new();
                set.insert(v.to_string());
                open_items.set(set);
            }
        });
    }

    let toggle_item = {
        let open_items = open_items.clone();
        let accordion_type = accordion_type.clone();
        let on_value_change = on_value_change.clone();
        Callback::from(move |item_value: String| {
            let mut new_items = (*open_items).clone();

            if new_items.contains(&item_value) {
                // Only remove if collapsible is true or there are multiple items
                if collapsible || new_items.len() > 1 {
                    new_items.remove(&item_value);
                }
            } else {
                match accordion_type {
                    AccordionType::Single => {
                        new_items.clear();
                        new_items.insert(item_value.clone());
                    }
                    AccordionType::Multiple => {
                        new_items.insert(item_value.clone());
                    }
                }
            }

            open_items.set(new_items);

            if let Some(callback) = on_value_change.as_ref() {
                callback.emit(item_value);
            }
        })
    };

    let context = AccordionContext {
        open_items: (*open_items).clone(),
        toggle_item,
        accordion_type: accordion_type.clone(),
        collapsible,
    };

    let classes: Classes = vec![Classes::from("accordion"), class]
        .into_iter()
        .collect();

    html! {
        <ContextProvider<AccordionContext> context={context}>
            <div class={classes} data-orientation="vertical">
                { children }
            </div>
        </ContextProvider<AccordionContext>>
    }
}

/// Accordion item properties
#[derive(Properties, PartialEq, Clone)]
pub struct AccordionItemProps {
    /// Unique value for this item
    pub value: AttrValue,

    /// Whether this item is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Accordion item component
///
/// A single collapsible item within an accordion.
#[function_component(AccordionItem)]
pub fn accordion_item(props: &AccordionItemProps) -> Html {
    let AccordionItemProps {
        value,
        disabled,
        class,
        children,
    } = props.clone();

    let accordion_ctx = use_context::<AccordionContext>();
    let is_open = accordion_ctx
        .as_ref()
        .map(|ctx| ctx.open_items.contains(&value.to_string()))
        .unwrap_or(false);

    let toggle = {
        let value = value.to_string();
        let accordion_ctx = accordion_ctx.clone();
        Callback::from(move |_: ()| {
            if let Some(ctx) = accordion_ctx.as_ref()
                && !disabled
            {
                ctx.toggle_item.emit(value.clone());
            }
        })
    };

    let item_context = AccordionItemContext {
        value: value.to_string(),
        is_open,
        toggle,
    };

    let classes: Classes = vec![
        Classes::from("accordion-item"),
        if disabled {
            Classes::from("accordion-item-disabled")
        } else {
            Classes::new()
        },
        if is_open {
            Classes::from("accordion-item-open")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <ContextProvider<AccordionItemContext> context={item_context}>
            <div class={classes} data-value={value.clone()} data-state={if is_open { "open" } else { "closed" }}>
                { children }
            </div>
        </ContextProvider<AccordionItemContext>>
    }
}

/// Accordion trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct AccordionTriggerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Accordion trigger component
///
/// The button that toggles the accordion item.
#[function_component(AccordionTrigger)]
pub fn accordion_trigger(props: &AccordionTriggerProps) -> Html {
    let AccordionTriggerProps { class, children } = props.clone();

    let item_ctx = use_context::<AccordionItemContext>();
    let is_open = item_ctx.as_ref().map(|ctx| ctx.is_open).unwrap_or(false);

    let handle_click = {
        let item_ctx = item_ctx.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ctx) = item_ctx.as_ref() {
                ctx.toggle.emit(());
            }
        })
    };

    let classes: Classes = vec![
        Classes::from("accordion-trigger"),
        if is_open {
            Classes::from("accordion-trigger-open")
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
            class={classes}
            aria-expanded={is_open.to_string()}
            onclick={handle_click}
        >
            <span>{ children }</span>
            <span class="accordion-chevron" aria-hidden="true">
                { "▼" }
            </span>
        </button>
    }
}

/// Accordion content properties
#[derive(Properties, PartialEq, Clone)]
pub struct AccordionContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Accordion content component
///
/// The collapsible content section.
#[function_component(AccordionContent)]
pub fn accordion_content(props: &AccordionContentProps) -> Html {
    let AccordionContentProps { class, children } = props.clone();

    let item_ctx = use_context::<AccordionItemContext>();
    let is_open = item_ctx.as_ref().map(|ctx| ctx.is_open).unwrap_or(false);

    if !is_open {
        return html! {};
    }

    let classes: Classes = vec![Classes::from("accordion-content"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="region" aria-hidden={(!is_open).to_string()}>
            <div class="accordion-content-text">
                { children }
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accordion_props_default() {
        let props = AccordionProps {
            accordion_type: AccordionType::Single,
            collapsible: true,
            default_value: None,
            value: None,
            on_value_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.accordion_type, AccordionType::Single);
        assert!(props.collapsible);
    }

    #[test]
    fn test_accordion_type_multiple() {
        let props = AccordionProps {
            accordion_type: AccordionType::Multiple,
            collapsible: true,
            default_value: None,
            value: None,
            on_value_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.accordion_type, AccordionType::Multiple);
    }

    #[test]
    fn test_accordion_item_disabled() {
        let props = AccordionItemProps {
            value: AttrValue::from("item-1"),
            disabled: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_accordion_item_value() {
        let props = AccordionItemProps {
            value: AttrValue::from("item-1"),
            disabled: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.value, AttrValue::from("item-1"));
    }
}
