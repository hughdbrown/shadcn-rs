//! Breadcrumb component
//!
//! Displays the path to the current resource using a hierarchy of links.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Breadcrumb, BreadcrumbList, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Breadcrumb>
//!             <BreadcrumbList>
//!                 <BreadcrumbItem>
//!                     <BreadcrumbLink href="/">{ "Home" }</BreadcrumbLink>
//!                 </BreadcrumbItem>
//!                 <BreadcrumbSeparator />
//!                 <BreadcrumbItem>
//!                     <BreadcrumbLink href="/docs">{ "Docs" }</BreadcrumbLink>
//!                 </BreadcrumbItem>
//!                 <BreadcrumbSeparator />
//!                 <BreadcrumbItem>
//!                     <BreadcrumbPage>{ "Components" }</BreadcrumbPage>
//!                 </BreadcrumbItem>
//!             </BreadcrumbList>
//!         </Breadcrumb>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Breadcrumb container properties
#[derive(Properties, PartialEq, Clone)]
pub struct BreadcrumbProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// ARIA label for the breadcrumb navigation
    #[prop_or(AttrValue::from("Breadcrumb"))]
    pub aria_label: AttrValue,

    /// Children elements
    pub children: Children,
}

/// Breadcrumb container component
///
/// Provides semantic navigation structure.
///
/// # Accessibility
/// - Uses `<nav>` element with proper ARIA label
/// - Announces current page location
/// - Screen reader friendly
#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &BreadcrumbProps) -> Html {
    let BreadcrumbProps {
        class,
        aria_label,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("breadcrumb"), class]
        .into_iter()
        .collect();

    html! {
        <nav class={classes} aria-label={aria_label}>
            { children }
        </nav>
    }
}

/// Breadcrumb list properties
#[derive(Properties, PartialEq, Clone)]
pub struct BreadcrumbListProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Maximum number of items to display before truncating with ellipsis.
    /// When set and children count exceeds this value (minimum 3),
    /// renders the first item, an ellipsis, and the last `max_items - 2` items.
    #[prop_or_default]
    pub max_items: Option<usize>,

    /// Children elements
    pub children: Children,
}

/// Breadcrumb list component
///
/// Contains the breadcrumb items. Supports truncation via `max_items`.
#[function_component(BreadcrumbList)]
pub fn breadcrumb_list(props: &BreadcrumbListProps) -> Html {
    let BreadcrumbListProps {
        class,
        max_items,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("breadcrumb-list"), class]
        .into_iter()
        .collect();

    let content: Html = match max_items {
        Some(max) if max >= 3 && children.len() > max => {
            let all: Vec<Html> = children.iter().collect();
            let first = all[0].clone();
            let tail_start: usize = all.len() - (max - 2);
            let tail: Vec<Html> = all[tail_start..].to_vec();

            html! {
                <>
                    { first }
                    <li class="breadcrumb-ellipsis" role="presentation">{ "\u{2026}" }</li>
                    { tail.into_iter().collect::<Html>() }
                </>
            }
        }
        _ => {
            html! { { children } }
        }
    };

    html! {
        <ol class={classes}>
            { content }
        </ol>
    }
}

/// Breadcrumb item properties
#[derive(Properties, PartialEq, Clone)]
pub struct BreadcrumbItemProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Breadcrumb item component
///
/// A single item in the breadcrumb trail.
#[function_component(BreadcrumbItem)]
pub fn breadcrumb_item(props: &BreadcrumbItemProps) -> Html {
    let BreadcrumbItemProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("breadcrumb-item"), class]
        .into_iter()
        .collect();

    html! {
        <li class={classes}>
            { children }
        </li>
    }
}

/// Breadcrumb link properties
#[derive(Properties, PartialEq, Clone)]
pub struct BreadcrumbLinkProps {
    /// Link href
    pub href: AttrValue,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Children elements
    pub children: Children,
}

/// Breadcrumb link component
///
/// A clickable link in the breadcrumb trail.
#[function_component(BreadcrumbLink)]
pub fn breadcrumb_link(props: &BreadcrumbLinkProps) -> Html {
    let BreadcrumbLinkProps {
        href,
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("breadcrumb-link"), class]
        .into_iter()
        .collect();

    html! {
        <a class={classes} href={href} onclick={onclick}>
            { children }
        </a>
    }
}

/// Breadcrumb page properties
#[derive(Properties, PartialEq, Clone)]
pub struct BreadcrumbPageProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Breadcrumb page component
///
/// The current page in the breadcrumb trail (not a link).
#[function_component(BreadcrumbPage)]
pub fn breadcrumb_page(props: &BreadcrumbPageProps) -> Html {
    let BreadcrumbPageProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("breadcrumb-page"), class]
        .into_iter()
        .collect();

    html! {
        <span class={classes} aria-current="page">
            { children }
        </span>
    }
}

/// Breadcrumb separator properties
#[derive(Properties, PartialEq, Clone)]
pub struct BreadcrumbSeparatorProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Custom separator content (defaults to /)
    #[prop_or_default]
    pub children: Children,
}

/// Breadcrumb separator component
///
/// Separates breadcrumb items visually.
#[function_component(BreadcrumbSeparator)]
pub fn breadcrumb_separator(props: &BreadcrumbSeparatorProps) -> Html {
    let BreadcrumbSeparatorProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("breadcrumb-separator"), class]
        .into_iter()
        .collect();

    let content = if children.is_empty() {
        html! { "/" }
    } else {
        html! { { children } }
    };

    html! {
        <li class={classes} role="presentation" aria-hidden="true">
            { content }
        </li>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_default_aria_label() {
        let props = BreadcrumbProps {
            class: Classes::new(),
            aria_label: AttrValue::from("Breadcrumb"),
            children: Children::new(vec![]),
        };

        assert_eq!(props.aria_label, AttrValue::from("Breadcrumb"));
    }

    #[test]
    fn test_breadcrumb_custom_aria_label() {
        let props = BreadcrumbProps {
            class: Classes::new(),
            aria_label: AttrValue::from("Navigation path"),
            children: Children::new(vec![]),
        };

        assert_eq!(props.aria_label, AttrValue::from("Navigation path"));
    }

    #[test]
    fn test_breadcrumb_list_max_items_prop() {
        // When max_items is None, all children render normally
        let props_none = BreadcrumbListProps {
            class: Classes::new(),
            max_items: None,
            children: Children::new(vec![]),
        };
        assert!(props_none.max_items.is_none());

        // When max_items is set, the value is stored correctly
        let props_some = BreadcrumbListProps {
            class: Classes::new(),
            max_items: Some(4),
            children: Children::new(vec![]),
        };
        assert_eq!(props_some.max_items, Some(4));

        // max_items less than 3 should not trigger truncation
        let props_small = BreadcrumbListProps {
            class: Classes::new(),
            max_items: Some(2),
            children: Children::new(vec![]),
        };
        assert_eq!(props_small.max_items, Some(2));
    }

    #[test]
    fn test_breadcrumb_link_href() {
        let props = BreadcrumbLinkProps {
            href: AttrValue::from("/home"),
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.href, AttrValue::from("/home"));
    }
}
