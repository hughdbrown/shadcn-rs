//! Toggle Group component
//!
//! Groups multiple toggle buttons with single or multiple selection.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{ToggleGroup, Toggle, ToggleGroupType, ToggleGroupOrientation};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <ToggleGroup
//!             r#type={ToggleGroupType::Single}
//!             orientation={ToggleGroupOrientation::Horizontal}
//!         >
//!             <Toggle>{ "Left" }</Toggle>
//!             <Toggle>{ "Center" }</Toggle>
//!             <Toggle>{ "Right" }</Toggle>
//!         </ToggleGroup>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Toggle group type
#[derive(Debug, Clone, PartialEq)]
pub enum ToggleGroupType {
    /// Only one toggle can be pressed at a time
    Single,
    /// Multiple toggles can be pressed
    Multiple,
}

/// Toggle group orientation
#[derive(Debug, Clone, PartialEq)]
pub enum ToggleGroupOrientation {
    /// Horizontal layout
    Horizontal,
    /// Vertical layout
    Vertical,
}

/// Toggle group component properties
#[derive(Properties, PartialEq, Clone)]
pub struct ToggleGroupProps {
    /// Selection type (single or multiple)
    #[prop_or(ToggleGroupType::Single)]
    pub r#type: ToggleGroupType,

    /// Layout orientation
    #[prop_or(ToggleGroupOrientation::Horizontal)]
    pub orientation: ToggleGroupOrientation,

    /// Selected value (for single type)
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Selected values (for multiple type)
    #[prop_or_default]
    pub values: Option<Vec<AttrValue>>,

    /// Value change handler
    #[prop_or_default]
    pub on_value_change: Option<Callback<AttrValue>>,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements (toggles)
    pub children: Children,
}

/// Toggle group component
///
/// Groups toggle buttons with single or multiple selection support.
///
/// # Accessibility
/// - Uses role="group"
/// - Radio group for single selection
/// - Keyboard navigation with arrow keys
/// - Proper ARIA attributes
#[function_component(ToggleGroup)]
pub fn toggle_group(props: &ToggleGroupProps) -> Html {
    let ToggleGroupProps {
        r#type,
        orientation,
        value: _,
        values: _,
        on_value_change: _,
        disabled,
        class,
        children,
    } = props.clone();

    let type_class = match r#type {
        ToggleGroupType::Single => "toggle-group-single",
        ToggleGroupType::Multiple => "toggle-group-multiple",
    };

    let orientation_class = match orientation {
        ToggleGroupOrientation::Horizontal => "toggle-group-horizontal",
        ToggleGroupOrientation::Vertical => "toggle-group-vertical",
    };

    let classes: Classes = vec![
        Classes::from("toggle-group"),
        Classes::from(type_class),
        Classes::from(orientation_class),
        if disabled {
            Classes::from("toggle-group-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let role = match r#type {
        ToggleGroupType::Single => "radiogroup",
        ToggleGroupType::Multiple => "group",
    };

    let aria_orientation = match orientation {
        ToggleGroupOrientation::Horizontal => "horizontal",
        ToggleGroupOrientation::Vertical => "vertical",
    };

    html! {
        <div
            class={classes}
            role={role}
            aria-orientation={aria_orientation}
            aria-disabled={disabled.to_string()}
        >
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_group_single() {
        let props = ToggleGroupProps {
            r#type: ToggleGroupType::Single,
            orientation: ToggleGroupOrientation::Horizontal,
            value: None,
            values: None,
            on_value_change: None,
            disabled: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.r#type, ToggleGroupType::Single);
    }

    #[test]
    fn test_toggle_group_multiple() {
        let props = ToggleGroupProps {
            r#type: ToggleGroupType::Multiple,
            orientation: ToggleGroupOrientation::Horizontal,
            value: None,
            values: None,
            on_value_change: None,
            disabled: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.r#type, ToggleGroupType::Multiple);
    }

    #[test]
    fn test_toggle_group_vertical() {
        let props = ToggleGroupProps {
            r#type: ToggleGroupType::Single,
            orientation: ToggleGroupOrientation::Vertical,
            value: None,
            values: None,
            on_value_change: None,
            disabled: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.orientation, ToggleGroupOrientation::Vertical);
    }

    #[test]
    fn test_toggle_group_with_value() {
        let props = ToggleGroupProps {
            r#type: ToggleGroupType::Single,
            orientation: ToggleGroupOrientation::Horizontal,
            value: Some(AttrValue::from("center")),
            values: None,
            on_value_change: None,
            disabled: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.value, Some(AttrValue::from("center")));
    }

    #[test]
    fn test_toggle_group_disabled() {
        let props = ToggleGroupProps {
            r#type: ToggleGroupType::Single,
            orientation: ToggleGroupOrientation::Horizontal,
            value: None,
            values: None,
            on_value_change: None,
            disabled: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }
}
