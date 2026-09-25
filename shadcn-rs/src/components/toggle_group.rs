//! Toggle Group component
//!
//! Groups multiple toggle buttons with single or multiple selection.
//!
//! # Examples
//!
//! Uncontrolled, with a default selection:
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
//!             default_value={vec![AttrValue::from("center")]}
//!         >
//!             <ToggleGroupItem value="left" aria_label="Align left">{ "L" }</ToggleGroupItem>
//!             <ToggleGroupItem value="center" aria_label="Align center">{ "C" }</ToggleGroupItem>
//!             <ToggleGroupItem value="right" aria_label="Align right">{ "R" }</ToggleGroupItem>
//!         </ToggleGroup>
//!     }
//! }
//! ```
//!
//! Controlled: `on_value_change` receives the whole resulting selection.
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{ToggleGroup, ToggleGroupItem, ToggleGroupType};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let marks = use_state(Vec::<AttrValue>::new);
//!     let on_value_change = {
//!         let marks = marks.clone();
//!         Callback::from(move |value: Vec<AttrValue>| marks.set(value))
//!     };
//!
//!     html! {
//!         <ToggleGroup
//!             r#type={ToggleGroupType::Multiple}
//!             value={(*marks).clone()}
//!             {on_value_change}
//!         >
//!             <ToggleGroupItem value="bold" aria_label="Bold">{ "B" }</ToggleGroupItem>
//!             <ToggleGroupItem value="italic" aria_label="Italic">{ "I" }</ToggleGroupItem>
//!         </ToggleGroup>
//!     }
//! }
//! ```

use crate::hooks::use_controllable_state;
use crate::types::Size;
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
    /// Currently pressed values, in the order they were pressed
    pub selected_values: Vec<AttrValue>,
    /// Callback to toggle a value
    pub toggle_value: Callback<AttrValue>,
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

    /// Pressed values. `Some` makes the group controlled. In single mode the
    /// vector holds at most one value.
    #[prop_or_default]
    pub value: Option<Vec<AttrValue>>,

    /// Initially pressed values for an uncontrolled group (`value` unset)
    #[prop_or_default]
    pub default_value: Vec<AttrValue>,

    /// Alias for `value`, kept for compatibility. `value` wins when both are set.
    #[prop_or_default]
    pub values: Option<Vec<AttrValue>>,

    /// Called with the resulting selection after an item is toggled
    #[prop_or_default]
    pub on_value_change: Option<Callback<Vec<AttrValue>>>,

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

/// Selection after toggling `item`: pressing a pressed item releases it;
/// otherwise single mode replaces the selection and multiple mode appends.
pub fn next_toggle_group_value(
    current: &[AttrValue],
    item: &AttrValue,
    group_type: &ToggleGroupType,
) -> Vec<AttrValue> {
    if current.contains(item) {
        return current.iter().filter(|v| *v != item).cloned().collect();
    }
    match group_type {
        ToggleGroupType::Single => vec![item.clone()],
        ToggleGroupType::Multiple => {
            let mut next = current.to_vec();
            next.push(item.clone());
            next
        }
    }
}

/// Toggle group component
///
/// Groups toggle buttons with single or multiple selection support.
///
/// # Controlled and uncontrolled
/// Pass `value` to control the pressed items from the parent, or leave it
/// unset and use `default_value`. `on_value_change` receives the full
/// resulting selection in both modes.
///
/// # Accessibility
/// - Uses role="group"
/// - Radio group for single selection
/// - `aria_label` on items names icon-only toggles
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

    let (selected_values, set_selected) =
        use_controllable_state(value.or(values), default_value, on_value_change);

    let toggle_value = {
        let current = selected_values.clone();
        let group_type = r#type.clone();
        Callback::from(move |item: AttrValue| {
            set_selected.emit(next_toggle_group_value(&current, &item, &group_type));
        })
    };

    let context = ToggleGroupContext {
        selected_values,
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

    /// Accessible name, needed when the item only shows an icon
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

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
        aria_label,
        class,
        children,
    } = props.clone();

    let context = use_context::<ToggleGroupContext>();

    let is_pressed = context
        .as_ref()
        .is_some_and(|ctx| ctx.selected_values.contains(&value));

    let is_disabled = disabled || context.as_ref().is_some_and(|ctx| ctx.disabled);

    let size = context.as_ref().map(|ctx| ctx.size).unwrap_or(Size::Md);

    let handle_click = {
        let context = context.clone();
        let value = value.clone();
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

    let is_single = context
        .as_ref()
        .is_some_and(|ctx| ctx.group_type == ToggleGroupType::Single);

    // In single mode (radiogroup), items should be role="radio" with aria-checked
    // In multiple mode (group), items use aria-pressed
    let role = if is_single { Some("radio") } else { None };

    html! {
        <button
            type="button"
            class={classes}
            onclick={handle_click}
            disabled={is_disabled}
            role={role}
            aria-label={aria_label}
            aria-checked={if is_single { Some(is_pressed.to_string()) } else { None }}
            aria-pressed={if !is_single { Some(is_pressed.to_string()) } else { None }}
            data-state={if is_pressed { "on" } else { "off" }}
            data-value={value}
        >
            { children }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vals(items: &[&'static str]) -> Vec<AttrValue> {
        items.iter().map(|s| AttrValue::from(*s)).collect()
    }

    #[test]
    fn test_single_selects_and_replaces() {
        let single = ToggleGroupType::Single;
        assert_eq!(
            next_toggle_group_value(&[], &"a".into(), &single),
            vals(&["a"])
        );
        assert_eq!(
            next_toggle_group_value(&vals(&["a"]), &"b".into(), &single),
            vals(&["b"])
        );
    }

    #[test]
    fn test_single_deselect_empties_selection() {
        assert!(
            next_toggle_group_value(&vals(&["a"]), &"a".into(), &ToggleGroupType::Single)
                .is_empty()
        );
    }

    #[test]
    fn test_multiple_adds_and_removes() {
        let multiple = ToggleGroupType::Multiple;
        let added = next_toggle_group_value(&vals(&["a"]), &"b".into(), &multiple);
        assert_eq!(added, vals(&["a", "b"]));
        let removed = next_toggle_group_value(&added, &"a".into(), &multiple);
        assert_eq!(removed, vals(&["b"]));
    }

    #[test]
    fn test_toggle_group_props() {
        let props = yew::props!(ToggleGroupProps {
            r#type: ToggleGroupType::Multiple,
            orientation: ToggleGroupOrientation::Vertical,
            value: vals(&["center"]),
            disabled: true,
            children: Children::new(vec![]),
        });
        assert_eq!(props.r#type, ToggleGroupType::Multiple);
        assert_eq!(props.orientation, ToggleGroupOrientation::Vertical);
        assert_eq!(props.value, Some(vals(&["center"])));
        assert!(props.default_value.is_empty());
        assert!(props.disabled);
    }

    #[test]
    fn test_toggle_group_item_aria_label() {
        let props = yew::props!(ToggleGroupItemProps {
            value: "bold",
            aria_label: "Toggle bold",
            children: Children::new(vec![]),
        });
        assert_eq!(props.aria_label, Some(AttrValue::from("Toggle bold")));
    }
}
