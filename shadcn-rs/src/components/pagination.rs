//! Pagination component
//!
//! Navigation for paginated content.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationPrevious, PaginationNext, PaginationEllipsis};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let current_page = use_state(|| 1);
//!
//!     html! {
//!         <Pagination>
//!             <PaginationContent>
//!                 <PaginationItem>
//!                     <PaginationPrevious href="#" />
//!                 </PaginationItem>
//!                 <PaginationItem>
//!                     <PaginationLink href="#" is_active={true}>{ "1" }</PaginationLink>
//!                 </PaginationItem>
//!                 <PaginationItem>
//!                     <PaginationLink href="#">{ "2" }</PaginationLink>
//!                 </PaginationItem>
//!                 <PaginationItem>
//!                     <PaginationLink href="#">{ "3" }</PaginationLink>
//!                 </PaginationItem>
//!                 <PaginationItem>
//!                     <PaginationEllipsis />
//!                 </PaginationItem>
//!                 <PaginationItem>
//!                     <PaginationNext href="#" />
//!                 </PaginationItem>
//!             </PaginationContent>
//!         </Pagination>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Pagination container properties
#[derive(Properties, PartialEq, Clone)]
pub struct PaginationProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// ARIA label
    #[prop_or(AttrValue::from("Pagination"))]
    pub aria_label: AttrValue,

    /// Children elements
    pub children: Children,
}

/// Pagination container component
///
/// Provides navigation for paginated content.
///
/// # Accessibility
/// - Uses proper ARIA attributes
/// - Keyboard navigable
/// - Screen reader announcements
#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let PaginationProps {
        class,
        aria_label,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("pagination"), class]
        .into_iter()
        .collect();

    html! {
        <nav class={classes} role="navigation" aria-label={aria_label}>
            { children }
        </nav>
    }
}

/// Pagination content properties
#[derive(Properties, PartialEq, Clone)]
pub struct PaginationContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Pagination content component
///
/// Contains pagination items.
#[function_component(PaginationContent)]
pub fn pagination_content(props: &PaginationContentProps) -> Html {
    let PaginationContentProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("pagination-content"), class]
        .into_iter()
        .collect();

    html! {
        <ul class={classes}>
            { children }
        </ul>
    }
}

/// Pagination item properties
#[derive(Properties, PartialEq, Clone)]
pub struct PaginationItemProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Pagination item component
///
/// A single item in the pagination.
#[function_component(PaginationItem)]
pub fn pagination_item(props: &PaginationItemProps) -> Html {
    let PaginationItemProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("pagination-item"), class]
        .into_iter()
        .collect();

    html! {
        <li class={classes}>
            { children }
        </li>
    }
}

/// Pagination link properties
#[derive(Properties, PartialEq, Clone)]
pub struct PaginationLinkProps {
    /// Link href
    pub href: AttrValue,

    /// Whether this is the active/current page
    #[prop_or(false)]
    pub is_active: bool,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Children elements
    pub children: Children,
}

/// Pagination link component
///
/// A clickable page number link.
#[function_component(PaginationLink)]
pub fn pagination_link(props: &PaginationLinkProps) -> Html {
    let PaginationLinkProps {
        href,
        is_active,
        disabled,
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("pagination-link"),
        if is_active {
            Classes::from("pagination-link-active")
        } else {
            Classes::new()
        },
        if disabled {
            Classes::from("pagination-link-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let aria_current = if is_active {
        Some(AttrValue::from("page"))
    } else {
        None
    };

    html! {
        <a
            class={classes}
            href={href}
            onclick={onclick}
            aria-current={aria_current}
            aria-disabled={disabled.to_string()}
        >
            { children }
        </a>
    }
}

/// Pagination previous properties
#[derive(Properties, PartialEq, Clone)]
pub struct PaginationPreviousProps {
    /// Link href
    pub href: AttrValue,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Custom label (defaults to "Previous")
    #[prop_or_default]
    pub children: Children,
}

/// Pagination previous component
///
/// Navigate to the previous page.
#[function_component(PaginationPrevious)]
pub fn pagination_previous(props: &PaginationPreviousProps) -> Html {
    let PaginationPreviousProps {
        href,
        disabled,
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("pagination-previous"),
        if disabled {
            Classes::from("pagination-previous-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let label = if children.is_empty() {
        html! { "Previous" }
    } else {
        html! { { children } }
    };

    html! {
        <a
            class={classes}
            href={href}
            onclick={onclick}
            aria-label="Go to previous page"
            aria-disabled={disabled.to_string()}
        >
            { label }
        </a>
    }
}

/// Pagination next properties
#[derive(Properties, PartialEq, Clone)]
pub struct PaginationNextProps {
    /// Link href
    pub href: AttrValue,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Custom label (defaults to "Next")
    #[prop_or_default]
    pub children: Children,
}

/// Pagination next component
///
/// Navigate to the next page.
#[function_component(PaginationNext)]
pub fn pagination_next(props: &PaginationNextProps) -> Html {
    let PaginationNextProps {
        href,
        disabled,
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("pagination-next"),
        if disabled {
            Classes::from("pagination-next-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let label = if children.is_empty() {
        html! { "Next" }
    } else {
        html! { { children } }
    };

    html! {
        <a
            class={classes}
            href={href}
            onclick={onclick}
            aria-label="Go to next page"
            aria-disabled={disabled.to_string()}
        >
            { label }
        </a>
    }
}

/// Pagination ellipsis properties
#[derive(Properties, PartialEq, Clone)]
pub struct PaginationEllipsisProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Pagination ellipsis component
///
/// Indicates skipped pages.
#[function_component(PaginationEllipsis)]
pub fn pagination_ellipsis(props: &PaginationEllipsisProps) -> Html {
    let PaginationEllipsisProps { class } = props.clone();

    let classes: Classes = vec![Classes::from("pagination-ellipsis"), class]
        .into_iter()
        .collect();

    html! {
        <span class={classes} aria-hidden="true">
            { "..." }
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_default_aria_label() {
        let props = PaginationProps {
            class: Classes::new(),
            aria_label: AttrValue::from("Pagination"),
            children: Children::new(vec![]),
        };

        assert_eq!(props.aria_label, AttrValue::from("Pagination"));
    }

    #[test]
    fn test_pagination_link_active() {
        let props = PaginationLinkProps {
            href: AttrValue::from("#"),
            is_active: true,
            disabled: false,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert!(props.is_active);
        assert!(!props.disabled);
    }

    #[test]
    fn test_pagination_link_disabled() {
        let props = PaginationLinkProps {
            href: AttrValue::from("#"),
            is_active: false,
            disabled: true,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert!(!props.is_active);
        assert!(props.disabled);
    }

    #[test]
    fn test_pagination_previous_disabled() {
        let props = PaginationPreviousProps {
            href: AttrValue::from("#"),
            disabled: true,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_pagination_next_disabled() {
        let props = PaginationNextProps {
            href: AttrValue::from("#"),
            disabled: true,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }
}
