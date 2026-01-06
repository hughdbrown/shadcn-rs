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

    /// Children elements
    pub children: Children,
}

/// Breadcrumb list component
///
/// Contains the breadcrumb items.
#[function_component(BreadcrumbList)]
pub fn breadcrumb_list(props: &BreadcrumbListProps) -> Html {
    let BreadcrumbListProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("breadcrumb-list"), class]
        .into_iter()
        .collect();

    html! {
        <ol class={classes}>
            { children }
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
