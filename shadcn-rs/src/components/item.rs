//! Item component
//!
//! Generic list/menu item component with icon and description support.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::Item;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let onclick = Callback::from(|_| {
//!         // Handle click
//!     });
//!
//!     html! {
//!         <Item
//!             icon="📁"
//!             description="Click to open"
//!             {onclick}
//!         >
//!             { "Documents" }
//!         </Item>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Item component properties
#[derive(Properties, PartialEq, Clone)]
pub struct ItemProps {
    /// Icon to display (emoji or text)
    #[prop_or_default]
    pub icon: Option<AttrValue>,

    /// Description text
    #[prop_or_default]
    pub description: Option<AttrValue>,

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

    /// Children elements (main content)
    pub children: Children,
}

/// Item component
///
/// A generic selectable item for lists, menus, or navigation.
///
/// # Accessibility
/// - Uses button role for interactive items
/// - Proper ARIA states (selected, disabled)
/// - Keyboard accessible
#[function_component(Item)]
pub fn item(props: &ItemProps) -> Html {
    let ItemProps {
        icon,
        description,
        selected,
        disabled,
        onclick,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("item"),
        if selected {
            Classes::from("item-selected")
        } else {
            Classes::new()
        },
        if disabled {
            Classes::from("item-disabled")
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
            role="button"
            tabindex={if disabled { "-1" } else { "0" }}
            aria-selected={selected.to_string()}
            aria-disabled={disabled.to_string()}
            onclick={onclick}
        >
            if let Some(icon_content) = icon {
                <span class="item-icon" aria-hidden="true">
                    { icon_content }
                </span>
            }
            <div class="item-content">
                <div class="item-text">
                    { children }
                </div>
                if let Some(desc_text) = description {
                    <div class="item-description">
                        { desc_text }
                    </div>
                }
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_default() {
        let props = ItemProps {
            icon: None,
            description: None,
            selected: false,
            disabled: false,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.selected);
        assert!(!props.disabled);
    }

    #[test]
    fn test_item_with_icon() {
        let props = ItemProps {
            icon: Some(AttrValue::from("📁")),
            description: None,
            selected: false,
            disabled: false,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.icon, Some(AttrValue::from("📁")));
    }

    #[test]
    fn test_item_with_description() {
        let props = ItemProps {
            icon: None,
            description: Some(AttrValue::from("Item description")),
            selected: false,
            disabled: false,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.description, Some(AttrValue::from("Item description")));
    }

    #[test]
    fn test_item_selected() {
        let props = ItemProps {
            icon: None,
            description: None,
            selected: true,
            disabled: false,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.selected);
    }

    #[test]
    fn test_item_disabled() {
        let props = ItemProps {
            icon: None,
            description: None,
            selected: false,
            disabled: true,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }
}
