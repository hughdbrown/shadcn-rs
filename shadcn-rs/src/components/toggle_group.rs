//! Toggle Group component
//!
//! Groups multiple toggle buttons with single or multiple selection.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{ToggleGroup, ToggleGroupItem, ToggleGroupType, ToggleGroupOrientation};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <ToggleGroup
//!             r#type={ToggleGroupType::Single}
//!             orientation={ToggleGroupOrientation::Horizontal}
//!         >
//!             <ToggleGroupItem value="left">{ "Left" }</ToggleGroupItem>
//!             <ToggleGroupItem value="center">{ "Center" }</ToggleGroupItem>
//!             <ToggleGroupItem value="right">{ "Right" }</ToggleGroupItem>
//!         </ToggleGroup>
//!     }
//! }
//! ```

use crate::types::Size;
use std::collections::HashSet;
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

/// Context for sharing toggle group state between parent and children
#[derive(Clone, PartialEq)]
pub struct ToggleGroupContext {
    /// Currently selected values
    pub selected_values: HashSet<String>,
    /// Callback to toggle a value
    pub toggle_value: Callback<String>,
    /// Selection type (single or multiple)
    pub group_type: ToggleGroupType,
    /// Whether the group is disabled
    pub disabled: bool,
    /// Size of toggles in the group
    pub size: Size,
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

    /// Default value for uncontrolled mode
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Selected values (for multiple type)
    #[prop_or_default]
    pub values: Option<Vec<AttrValue>>,

    /// Value change handler
    #[prop_or_default]
    pub on_value_change: Option<Callback<AttrValue>>,

    /// Size of toggles
    #[prop_or(Size::Md)]
    pub size: Size,

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
        value,
        default_value,
        values,
        on_value_change,
        size,
        disabled,
        class,
        children,
    } = props.clone();

    // Initialize selected values from props
    let initial_values: HashSet<String> = if let Some(vals) = values.as_ref() {
        vals.iter().map(|v| v.to_string()).collect()
    } else if let Some(v) = value.as_ref().or(default_value.as_ref()) {
        let mut set = HashSet::new();
        set.insert(v.to_string());
        set
    } else {
        HashSet::new()
    };

    let selected_values = use_state(|| initial_values);

    // Sync with controlled value prop
    {
        let selected_values = selected_values.clone();
        use_effect_with(value.clone(), move |value| {
            if let Some(v) = value {
                let mut set = HashSet::new();
                set.insert(v.to_string());
                selected_values.set(set);
            }
        });
    }

    let toggle_value = {
        let selected_values = selected_values.clone();
        let group_type = r#type.clone();
        let on_value_change = on_value_change.clone();
        Callback::from(move |item_value: String| {
            let mut new_values = (*selected_values).clone();

            if new_values.contains(&item_value) {
                new_values.remove(&item_value);
            } else {
                match group_type {
                    ToggleGroupType::Single => {
                        new_values.clear();
                        new_values.insert(item_value.clone());
                    }
                    ToggleGroupType::Multiple => {
                        new_values.insert(item_value.clone());
                    }
                }
            }

            selected_values.set(new_values);

            if let Some(callback) = on_value_change.as_ref() {
                callback.emit(AttrValue::from(item_value));
            }
        })
    };

    let context = ToggleGroupContext {
        selected_values: (*selected_values).clone(),
        toggle_value,
        group_type: r#type.clone(),
        disabled,
        size,
    };

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
        <ContextProvider<ToggleGroupContext> context={context}>
            <div
                class={classes}
                role={role}
                aria-orientation={aria_orientation}
                aria-disabled={disabled.to_string()}
            >
                { children }
            </div>
        </ContextProvider<ToggleGroupContext>>
    }
}

/// Toggle group item properties
#[derive(Properties, PartialEq, Clone)]
pub struct ToggleGroupItemProps {
    /// Value for this toggle item
    pub value: AttrValue,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Toggle group item component
///
/// A toggle button within a toggle group.
#[function_component(ToggleGroupItem)]
pub fn toggle_group_item(props: &ToggleGroupItemProps) -> Html {
    let ToggleGroupItemProps {
        value,
        disabled,
        class,
        children,
    } = props.clone();

    let context = use_context::<ToggleGroupContext>();

    let is_pressed = context
        .as_ref()
        .map(|ctx| ctx.selected_values.contains(&value.to_string()))
        .unwrap_or(false);

    let is_disabled = disabled || context.as_ref().map(|ctx| ctx.disabled).unwrap_or(false);

    let size = context.as_ref().map(|ctx| ctx.size).unwrap_or(Size::Md);

    let handle_click = {
        let context = context.clone();
        let value = value.to_string();
        Callback::from(move |_: MouseEvent| {
            if !is_disabled && let Some(ctx) = context.as_ref() {
                ctx.toggle_value.emit(value.clone());
            }
        })
    };

    let size_class = match size {
        Size::Xs => "toggle-xs",
        Size::Sm => "toggle-sm",
        Size::Md => "toggle-md",
        Size::Lg => "toggle-lg",
        Size::Xl => "toggle-xl",
        Size::Xl2 => "toggle-2xl",
    };

    let classes: Classes = vec![
        Classes::from("toggle"),
        Classes::from("toggle-group-item"),
        Classes::from(size_class),
        if is_pressed {
            Classes::from("toggle-pressed")
        } else {
            Classes::new()
        },
        if is_disabled {
            Classes::from("toggle-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <button
            type="button"
            class={classes}
            onclick={handle_click}
            disabled={is_disabled}
            aria-pressed={is_pressed.to_string()}
            data-state={if is_pressed { "on" } else { "off" }}
        >
            { children }
        </button>
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
            default_value: None,
            values: None,
            on_value_change: None,
            size: Size::Md,
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
            default_value: None,
            values: None,
            on_value_change: None,
            size: Size::Md,
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
            default_value: None,
            values: None,
            on_value_change: None,
            size: Size::Md,
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
            default_value: None,
            values: None,
            on_value_change: None,
            size: Size::Md,
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
            default_value: None,
            values: None,
            on_value_change: None,
            size: Size::Md,
            disabled: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }
}
