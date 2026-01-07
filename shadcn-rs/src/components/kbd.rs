//! Kbd component
//!
//! A component for displaying keyboard shortcuts.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::Kbd;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <>
//!             <Kbd>{ "Ctrl" }</Kbd>
//!             <span>{ "+" }</span>
//!             <Kbd>{ "S" }</Kbd>
//!         </>
//!     }
//! }
//! ```

use crate::types::Size;
use yew::prelude::*;

/// Kbd component properties
#[derive(Properties, PartialEq, Clone)]
pub struct KbdProps {
    /// Size of the kbd element
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Kbd component
///
/// Represents keyboard input with semantic `<kbd>` element.
///
/// # Accessibility
/// - Uses semantic `<kbd>` HTML element
/// - Screen readers announce as keyboard input
#[function_component(Kbd)]
pub fn kbd(props: &KbdProps) -> Html {
    let KbdProps {
        size,
        class,
        children,
    } = props.clone();

    let size_class = match size {
        Size::Xs => "kbd-xs",
        Size::Sm => "kbd-sm",
        Size::Md => "kbd-md",
        Size::Lg => "kbd-lg",
        Size::Xl => "kbd-xl",
        Size::Xl2 => "kbd-2xl",
    };

    let classes: Classes = vec![Classes::from("kbd"), Classes::from(size_class), class]
        .into_iter()
        .collect();

    html! {
        <kbd class={classes}>
            { children }
        </kbd>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kbd_default_size() {
        let props = KbdProps {
            size: Size::Md,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.size, Size::Md);
    }

    #[test]
    fn test_kbd_small() {
        let props = KbdProps {
            size: Size::Sm,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.size, Size::Sm);
    }

    #[test]
    fn test_kbd_large() {
        let props = KbdProps {
            size: Size::Lg,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.size, Size::Lg);
    }
}
