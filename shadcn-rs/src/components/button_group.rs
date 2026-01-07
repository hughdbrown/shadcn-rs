//! Button Group component
//!
//! Groups multiple buttons together with connected appearance.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{ButtonGroup, Button, ButtonGroupOrientation};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <ButtonGroup orientation={ButtonGroupOrientation::Horizontal}>
//!             <Button>{ "Left" }</Button>
//!             <Button>{ "Center" }</Button>
//!             <Button>{ "Right" }</Button>
//!         </ButtonGroup>
//!     }
//! }
//! ```

use crate::types::Size;
use yew::prelude::*;

/// Button group orientation
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonGroupOrientation {
    /// Horizontal layout (side by side)
    Horizontal,
    /// Vertical layout (stacked)
    Vertical,
}

/// Button group component properties
#[derive(Properties, PartialEq, Clone)]
pub struct ButtonGroupProps {
    /// Layout orientation
    #[prop_or(ButtonGroupOrientation::Horizontal)]
    pub orientation: ButtonGroupOrientation,

    /// Size of buttons in group
    #[prop_or_default]
    pub size: Option<Size>,

    /// Connected appearance (no spacing between buttons)
    #[prop_or(true)]
    pub connected: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements (buttons)
    pub children: Children,
}

/// Button group component
///
/// Groups multiple buttons together with consistent styling.
///
/// # Accessibility
/// - Uses role="group" for semantic grouping
/// - Maintains individual button accessibility
#[function_component(ButtonGroup)]
pub fn button_group(props: &ButtonGroupProps) -> Html {
    let ButtonGroupProps {
        orientation,
        size: _,
        connected,
        class,
        children,
    } = props.clone();

    let orientation_class = match orientation {
        ButtonGroupOrientation::Horizontal => "button-group-horizontal",
        ButtonGroupOrientation::Vertical => "button-group-vertical",
    };

    let classes: Classes = vec![
        Classes::from("button-group"),
        Classes::from(orientation_class),
        if connected {
            Classes::from("button-group-connected")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div class={classes} role="group">
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_group_horizontal() {
        let props = ButtonGroupProps {
            orientation: ButtonGroupOrientation::Horizontal,
            size: None,
            connected: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.orientation, ButtonGroupOrientation::Horizontal);
        assert!(props.connected);
    }

    #[test]
    fn test_button_group_vertical() {
        let props = ButtonGroupProps {
            orientation: ButtonGroupOrientation::Vertical,
            size: None,
            connected: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.orientation, ButtonGroupOrientation::Vertical);
    }

    #[test]
    fn test_button_group_not_connected() {
        let props = ButtonGroupProps {
            orientation: ButtonGroupOrientation::Horizontal,
            size: None,
            connected: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.connected);
    }

    #[test]
    fn test_button_group_with_size() {
        let props = ButtonGroupProps {
            orientation: ButtonGroupOrientation::Horizontal,
            size: Some(Size::Lg),
            connected: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.size, Some(Size::Lg));
    }
}
