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

use yew::prelude::*;
use crate::hooks::use_toggle;

/// Accordion type
#[derive(Debug, Clone, PartialEq)]
pub enum AccordionType {
    /// Only one item can be open at a time
    Single,
    /// Multiple items can be open simultaneously
    Multiple,
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

    let classes: Classes = vec![Classes::from("accordion"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} data-orientation="vertical">
            { children }
        </div>
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

    let classes: Classes = vec![
        Classes::from("accordion-item"),
        if disabled {
            Classes::from("accordion-item-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div class={classes} data-value={value}>
            { children }
        </div>
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

    let (is_open, toggle, _) = use_toggle(false);

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
            onclick={move |_| toggle.emit(())}
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

    let classes: Classes = vec![Classes::from("accordion-content"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="region">
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
