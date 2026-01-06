//! Empty component
//!
//! Displays an empty state with icon, message, and optional action.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::Empty;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Empty
//!             icon="📭"
//!             title="No messages"
//!             description="You don't have any messages yet."
//!         />
//!     }
//! }
//! ```

use yew::prelude::*;

/// Empty state component properties
#[derive(Properties, PartialEq, Clone)]
pub struct EmptyProps {
    /// Icon to display (emoji or text)
    #[prop_or_default]
    pub icon: Option<AttrValue>,

    /// Title text
    #[prop_or_default]
    pub title: Option<AttrValue>,

    /// Description text
    #[prop_or_default]
    pub description: Option<AttrValue>,

    /// Action button content
    #[prop_or_default]
    pub action: Option<Children>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements (alternative to title/description)
    #[prop_or_default]
    pub children: Children,
}

/// Empty state component
///
/// Displays a friendly empty state when there's no content to show.
///
/// # Accessibility
/// - Uses semantic HTML structure
/// - Icon is decorative with aria-hidden
/// - Proper heading hierarchy
#[function_component(Empty)]
pub fn empty(props: &EmptyProps) -> Html {
    let EmptyProps {
        icon,
        title,
        description,
        action,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("empty"), class]
        .into_iter()
        .collect();

    let has_children = children.iter().count() > 0;

    html! {
        <div class={classes} role="status">
            if let Some(icon_content) = icon {
                <div class="empty-icon" aria-hidden="true">
                    { icon_content }
                </div>
            }
            if has_children {
                { children }
            } else {
                <>
                    if let Some(title_text) = title {
                        <h3 class="empty-title">
                            { title_text }
                        </h3>
                    }
                    if let Some(desc_text) = description {
                        <p class="empty-description">
                            { desc_text }
                        </p>
                    }
                </>
            }
            if let Some(action_content) = action {
                <div class="empty-action">
                    { action_content }
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_with_icon() {
        let props = EmptyProps {
            icon: Some(AttrValue::from("📭")),
            title: None,
            description: None,
            action: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.icon, Some(AttrValue::from("📭")));
    }

    #[test]
    fn test_empty_with_title() {
        let props = EmptyProps {
            icon: None,
            title: Some(AttrValue::from("No items")),
            description: None,
            action: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.title, Some(AttrValue::from("No items")));
    }

    #[test]
    fn test_empty_with_description() {
        let props = EmptyProps {
            icon: None,
            title: None,
            description: Some(AttrValue::from("Try adding some items")),
            action: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(
            props.description,
            Some(AttrValue::from("Try adding some items"))
        );
    }

    #[test]
    fn test_empty_complete() {
        let props = EmptyProps {
            icon: Some(AttrValue::from("📭")),
            title: Some(AttrValue::from("No messages")),
            description: Some(AttrValue::from("You don't have any messages yet.")),
            action: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.icon, Some(AttrValue::from("📭")));
        assert_eq!(props.title, Some(AttrValue::from("No messages")));
        assert_eq!(
            props.description,
            Some(AttrValue::from("You don't have any messages yet."))
        );
    }
}
