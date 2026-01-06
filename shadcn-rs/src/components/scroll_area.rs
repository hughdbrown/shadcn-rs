//! Scroll Area component
//!
//! A container with custom-styled scrollbars.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{ScrollArea, ScrollDirection};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <ScrollArea direction={ScrollDirection::Vertical} max_height="400px">
//!             <div>{ "Long content here..." }</div>
//!         </ScrollArea>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Scroll direction
#[derive(Debug, Clone, PartialEq)]
pub enum ScrollDirection {
    /// Vertical scrolling only
    Vertical,
    /// Horizontal scrolling only
    Horizontal,
    /// Both vertical and horizontal scrolling
    Both,
}

/// Scroll area component properties
#[derive(Properties, PartialEq, Clone)]
pub struct ScrollAreaProps {
    /// Scroll direction
    #[prop_or(ScrollDirection::Vertical)]
    pub direction: ScrollDirection,

    /// Maximum height (CSS value)
    #[prop_or_default]
    pub max_height: Option<AttrValue>,

    /// Maximum width (CSS value)
    #[prop_or_default]
    pub max_width: Option<AttrValue>,

    /// Show scroll shadow effects
    #[prop_or(true)]
    pub show_shadow: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Scroll area component
///
/// Provides custom scrollbars with consistent cross-browser styling.
///
/// # Accessibility
/// - Maintains native scrolling behavior
/// - Keyboard navigation supported
/// - Screen reader compatible
#[function_component(ScrollArea)]
pub fn scroll_area(props: &ScrollAreaProps) -> Html {
    let ScrollAreaProps {
        direction,
        max_height,
        max_width,
        show_shadow,
        class,
        children,
    } = props.clone();

    let direction_class = match direction {
        ScrollDirection::Vertical => "scroll-area-vertical",
        ScrollDirection::Horizontal => "scroll-area-horizontal",
        ScrollDirection::Both => "scroll-area-both",
    };

    let classes: Classes = vec![
        Classes::from("scroll-area"),
        Classes::from(direction_class),
        if show_shadow {
            Classes::from("scroll-area-shadow")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let mut style_parts = Vec::new();

    if let Some(height) = max_height {
        style_parts.push(format!("max-height: {}", height));
    }

    if let Some(width) = max_width {
        style_parts.push(format!("max-width: {}", width));
    }

    let style = if !style_parts.is_empty() {
        Some(style_parts.join("; "))
    } else {
        None
    };

    html! {
        <div class={classes} style={style}>
            <div class="scroll-area-viewport">
                { children }
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_area_default() {
        let props = ScrollAreaProps {
            direction: ScrollDirection::Vertical,
            max_height: None,
            max_width: None,
            show_shadow: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.direction, ScrollDirection::Vertical);
        assert!(props.show_shadow);
    }

    #[test]
    fn test_scroll_area_horizontal() {
        let props = ScrollAreaProps {
            direction: ScrollDirection::Horizontal,
            max_height: None,
            max_width: None,
            show_shadow: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.direction, ScrollDirection::Horizontal);
    }

    #[test]
    fn test_scroll_area_both() {
        let props = ScrollAreaProps {
            direction: ScrollDirection::Both,
            max_height: None,
            max_width: None,
            show_shadow: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.direction, ScrollDirection::Both);
    }

    #[test]
    fn test_scroll_area_with_max_height() {
        let props = ScrollAreaProps {
            direction: ScrollDirection::Vertical,
            max_height: Some(AttrValue::from("400px")),
            max_width: None,
            show_shadow: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.max_height, Some(AttrValue::from("400px")));
    }

    #[test]
    fn test_scroll_area_no_shadow() {
        let props = ScrollAreaProps {
            direction: ScrollDirection::Vertical,
            max_height: None,
            max_width: None,
            show_shadow: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.show_shadow);
    }
}
