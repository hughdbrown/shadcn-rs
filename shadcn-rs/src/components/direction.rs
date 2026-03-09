//! Direction component
//!
//! A provider component that sets the text direction (LTR/RTL) for the application.
//! Essential for supporting right-to-left languages like Arabic, Hebrew, and Persian.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Direction, DirectionProvider, Button};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <DirectionProvider dir={Direction::Rtl}>
//!             <Button>{ "مرحبا" }</Button>
//!         </DirectionProvider>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Text direction for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    /// Left-to-right (default for Latin, Cyrillic, etc.)
    #[default]
    Ltr,
    /// Right-to-left (for Arabic, Hebrew, Persian, etc.)
    Rtl,
}

impl Direction {
    /// Returns the CSS direction value
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Ltr => "ltr",
            Direction::Rtl => "rtl",
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Context for sharing direction state through the component tree
#[derive(Clone, Debug, PartialEq)]
pub struct DirectionContext {
    /// The current text direction
    pub dir: Direction,
}

/// DirectionProvider component properties
#[derive(Properties, PartialEq, Clone)]
pub struct DirectionProviderProps {
    /// The text direction to set
    #[prop_or_default]
    pub dir: Direction,

    /// Children elements
    pub children: Children,
}

/// DirectionProvider component
///
/// Wraps the application or a subtree to provide text direction context.
/// Direction-aware components (menus, popovers, navigation) will automatically
/// adapt their behavior based on this context.
///
/// # Accessibility
/// - Sets the `dir` attribute on the wrapper element
/// - Provides direction context to child components via `use_direction` hook
#[function_component(DirectionProvider)]
pub fn direction_provider(props: &DirectionProviderProps) -> Html {
    let DirectionProviderProps { dir, children } = props.clone();

    let context = DirectionContext { dir };

    html! {
        <ContextProvider<DirectionContext> {context}>
            <div dir={dir.as_str()}>
                { children }
            </div>
        </ContextProvider<DirectionContext>>
    }
}

/// Hook to access the current text direction from a DirectionProvider ancestor.
///
/// Returns `Direction::Ltr` if no DirectionProvider is found in the tree.
///
/// # Examples
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::components::direction::{use_direction, Direction};
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let dir = use_direction();
///
///     let class = if dir == Direction::Rtl { "text-right" } else { "text-left" };
///
///     html! {
///         <div class={class}>
///             { "Direction-aware content" }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_direction() -> Direction {
    use_context::<DirectionContext>()
        .map(|ctx| ctx.dir)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_default_is_ltr() {
        assert_eq!(Direction::default(), Direction::Ltr);
    }

    #[test]
    fn test_direction_as_str() {
        assert_eq!(Direction::Ltr.as_str(), "ltr");
        assert_eq!(Direction::Rtl.as_str(), "rtl");
    }

    #[test]
    fn test_direction_display() {
        assert_eq!(format!("{}", Direction::Ltr), "ltr");
        assert_eq!(format!("{}", Direction::Rtl), "rtl");
    }

    #[test]
    fn test_direction_context_equality() {
        let ctx1 = DirectionContext {
            dir: Direction::Ltr,
        };
        let ctx2 = DirectionContext {
            dir: Direction::Ltr,
        };
        let ctx3 = DirectionContext {
            dir: Direction::Rtl,
        };

        assert_eq!(ctx1, ctx2);
        assert_ne!(ctx1, ctx3);
    }

    #[test]
    fn test_direction_provider_props() {
        let props = DirectionProviderProps {
            dir: Direction::Rtl,
            children: Children::new(vec![]),
        };

        assert_eq!(props.dir, Direction::Rtl);
    }

    #[test]
    fn test_direction_clone_copy() {
        let dir = Direction::Rtl;
        let cloned = dir;
        assert_eq!(dir, cloned);
    }
}
