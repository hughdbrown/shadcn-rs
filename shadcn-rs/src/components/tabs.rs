//! Tabs component
//!
//! A set of layered sections of content—known as tab panels—that are displayed one at a time.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Tabs, TabsList, TabsTrigger, TabsContent};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Tabs default_value="account">
//!             <TabsList>
//!                 <TabsTrigger value="account">{ "Account" }</TabsTrigger>
//!                 <TabsTrigger value="password">{ "Password" }</TabsTrigger>
//!             </TabsList>
//!             <TabsContent value="account">
//!                 <p>{ "Make changes to your account here." }</p>
//!             </TabsContent>
//!             <TabsContent value="password">
//!                 <p>{ "Change your password here." }</p>
//!             </TabsContent>
//!         </Tabs>
//!     }
//! }
//! ```

use crate::utils::{focus_element, generate_id};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use yew::prelude::*;

/// Tabs orientation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TabsOrientation {
    /// Horizontal tabs (default)
    Horizontal,
    /// Vertical tabs
    Vertical,
}

impl TabsOrientation {
    /// Convert to CSS class
    pub fn to_class(&self) -> &'static str {
        match self {
            TabsOrientation::Horizontal => "tabs-horizontal",
            TabsOrientation::Vertical => "tabs-vertical",
        }
    }

    /// Get ARIA orientation value
    pub fn to_aria(&self) -> &'static str {
        match self {
            TabsOrientation::Horizontal => "horizontal",
            TabsOrientation::Vertical => "vertical",
        }
    }
}

/// Context shared by `Tabs` with its triggers and panels
#[derive(Clone, PartialEq)]
pub struct TabsContext {
    /// Value of the active tab, if any
    pub value: Option<AttrValue>,
    /// Activates the tab with the given value
    pub set_value: Callback<AttrValue>,
    /// Orientation of the tab list (drives arrow-key handling)
    pub orientation: TabsOrientation,
    /// Per-instance id prefix used to link triggers and panels
    pub base_id: AttrValue,
}

impl TabsContext {
    /// Id of the trigger for `value`
    pub fn trigger_id(&self, value: &str) -> String {
        format!("{}-trigger-{}", self.base_id, value)
    }

    /// Id of the panel for `value`
    pub fn content_id(&self, value: &str) -> String {
        format!("{}-content-{}", self.base_id, value)
    }

    /// Whether `value` is the active tab
    pub fn is_active(&self, value: &str) -> bool {
        self.value.as_deref() == Some(value)
    }
}

/// Index of the item that should receive focus after `key` is pressed on the
/// item at `current` in a list of `len` enabled items.
///
/// Arrow keys follow the orientation and wrap around; Home and End jump to
/// the first and last item. Returns `None` for keys the list ignores.
pub(crate) fn roving_index(
    current: usize,
    len: usize,
    key: &str,
    orientation: TabsOrientation,
) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let (prev, next) = match orientation {
        TabsOrientation::Horizontal => ("ArrowLeft", "ArrowRight"),
        TabsOrientation::Vertical => ("ArrowUp", "ArrowDown"),
    };
    match key {
        "Home" => Some(0),
        "End" => Some(len - 1),
        k if k == next => Some((current + 1) % len),
        k if k == prev => Some((current + len - 1) % len),
        _ => None,
    }
}

/// Collects the elements matching `selector` inside `root`, in DOM order.
pub(crate) fn query_all(root: &Element, selector: &str) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all(selector) else {
        return Vec::new();
    };
    (0..nodes.length())
        .filter_map(|index| nodes.item(index))
        .filter_map(|node| node.dyn_into::<Element>().ok())
        .collect()
}

/// Tabs component properties
#[derive(Properties, PartialEq, Clone)]
pub struct TabsProps {
    /// Currently active tab value (controlled)
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Default active tab value (uncontrolled). When neither `value` nor
    /// `default_value` is set, the first enabled tab is activated.
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Callback when active tab changes
    #[prop_or_default]
    pub on_value_change: Option<Callback<String>>,

    /// Tabs orientation
    #[prop_or(TabsOrientation::Horizontal)]
    pub orientation: TabsOrientation,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Tabs component
///
/// A container for tab triggers and content panels. Works controlled
/// (`value` + `on_value_change`) or uncontrolled (`default_value`).
///
/// # Accessibility
/// - Keyboard navigation (Arrow keys following the orientation, Home, End)
/// - Roving tabindex: only the active tab is in the Tab order
/// - Triggers and panels linked with `aria-controls` / `aria-labelledby`
#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    let TabsProps {
        value,
        default_value,
        on_value_change,
        orientation,
        class,
        children,
    } = props.clone();

    let base_id = use_state(|| AttrValue::from(generate_id("tabs")));
    let internal_value = use_state(|| default_value);
    let root_ref = use_node_ref();

    let is_controlled = value.is_some();
    let current = if is_controlled {
        value
    } else {
        (*internal_value).clone()
    };

    // Uncontrolled with no default: activate the first enabled tab.
    {
        let internal_value = internal_value.clone();
        let root_ref = root_ref.clone();
        let needs_default = current.is_none();
        use_effect_with(needs_default, move |&needs_default| {
            if needs_default
                && let Some(root) = root_ref.cast::<Element>()
                && let Ok(Some(first)) = root.query_selector("[role='tab']:not([disabled])")
                && let Some(value) = first.get_attribute("data-value")
            {
                internal_value.set(Some(AttrValue::from(value)));
            }
        });
    }

    let set_value = {
        let internal_value = internal_value.clone();
        Callback::from(move |new_value: AttrValue| {
            if !is_controlled {
                internal_value.set(Some(new_value.clone()));
            }
            if let Some(callback) = on_value_change.as_ref() {
                callback.emit(new_value.to_string());
            }
        })
    };

    let context = TabsContext {
        value: current,
        set_value,
        orientation,
        base_id: (*base_id).clone(),
    };

    let classes: Classes = vec![
        Classes::from("tabs"),
        Classes::from(orientation.to_class()),
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <ContextProvider<TabsContext> context={context}>
            <div ref={root_ref} class={classes} data-orientation={orientation.to_aria()}>
                { children }
            </div>
        </ContextProvider<TabsContext>>
    }
}

/// Tabs list properties
#[derive(Properties, PartialEq, Clone)]
pub struct TabsListProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Tabs list component
///
/// Container for tab triggers.
#[function_component(TabsList)]
pub fn tabs_list(props: &TabsListProps) -> Html {
    let TabsListProps { class, children } = props.clone();

    let orientation = use_context::<TabsContext>()
        .map(|ctx| ctx.orientation)
        .unwrap_or(TabsOrientation::Horizontal);

    let classes: Classes = vec![Classes::from("tabs-list"), class]
        .into_iter()
        .collect();

    html! {
        <div
            class={classes}
            role="tablist"
            aria-orientation={orientation.to_aria()}
            data-orientation={orientation.to_aria()}
        >
            { children }
        </div>
    }
}

/// Tabs trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct TabsTriggerProps {
    /// Value of the tab this trigger activates
    pub value: AttrValue,

    /// Whether this tab is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Tabs trigger component
///
/// A button that activates a tab panel.
#[function_component(TabsTrigger)]
pub fn tabs_trigger(props: &TabsTriggerProps) -> Html {
    let TabsTriggerProps {
        value,
        disabled,
        class,
        children,
    } = props.clone();

    let context = use_context::<TabsContext>();
    let active = context
        .as_ref()
        .is_some_and(|ctx| ctx.is_active(value.as_str()));
    let (id, aria_controls) = match context.as_ref() {
        Some(ctx) => (
            Some(ctx.trigger_id(value.as_str())),
            ctx.content_id(value.as_str()),
        ),
        None => (None, format!("tabpanel-{}", value)),
    };
    let orientation = context
        .as_ref()
        .map(|ctx| ctx.orientation)
        .unwrap_or(TabsOrientation::Horizontal);

    let onclick = {
        let context = context.clone();
        let value = value.clone();
        Callback::from(move |_: MouseEvent| {
            if disabled {
                return;
            }
            if let Some(ctx) = context.as_ref() {
                ctx.set_value.emit(value.clone());
            }
        })
    };

    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        // Yew delegates events, so `current_target` is not this button;
        // resolve the tab from the original target instead.
        let Some(target) = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
            .and_then(|target| target.closest("[role='tab']").ok().flatten())
        else {
            return;
        };
        let Some(list) = target.closest("[role='tablist']").ok().flatten() else {
            return;
        };
        let tabs = query_all(&list, "[role='tab']:not([disabled])");
        let Some(current) = tabs.iter().position(|tab| tab == &target) else {
            return;
        };
        if let Some(next) = roving_index(current, tabs.len(), &event.key(), orientation) {
            event.prevent_default();
            let next_tab = &tabs[next];
            focus_element(next_tab);
            // Automatic activation: moving focus selects the tab.
            if let Some(html) = next_tab.dyn_ref::<HtmlElement>() {
                html.click();
            }
        }
    });

    let classes: Classes = vec![
        Classes::from("tabs-trigger"),
        if disabled {
            Classes::from("tabs-trigger-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let state = if active { "active" } else { "inactive" };
    let tabindex = if active { "0" } else { "-1" };

    html! {
        <button
            type="button"
            class={classes}
            role="tab"
            {id}
            data-value={value}
            data-state={state}
            disabled={disabled}
            aria-selected={active.to_string()}
            aria-controls={aria_controls}
            tabindex={tabindex}
            {onclick}
            {onkeydown}
        >
            { children }
        </button>
    }
}

/// Tabs content properties
#[derive(Properties, PartialEq, Clone)]
pub struct TabsContentProps {
    /// Value of the tab panel
    pub value: AttrValue,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Tabs content component
///
/// A panel that contains the content for a tab. Inactive panels stay in the
/// DOM with the `hidden` attribute and `data-state="inactive"`.
#[function_component(TabsContent)]
pub fn tabs_content(props: &TabsContentProps) -> Html {
    let TabsContentProps {
        value,
        class,
        children,
    } = props.clone();

    let context = use_context::<TabsContext>();
    // Without a Tabs parent there is nothing to select, so stay visible.
    let active = context
        .as_ref()
        .is_none_or(|ctx| ctx.is_active(value.as_str()));
    let (id, labelledby) = match context.as_ref() {
        Some(ctx) => (
            ctx.content_id(value.as_str()),
            Some(ctx.trigger_id(value.as_str())),
        ),
        None => (format!("tabpanel-{}", value), None),
    };

    let classes: Classes = vec![Classes::from("tabs-content"), class]
        .into_iter()
        .collect();

    html! {
        <div
            class={classes}
            role="tabpanel"
            {id}
            aria-labelledby={labelledby}
            data-value={value}
            data-state={if active { "active" } else { "inactive" }}
            hidden={!active}
            tabindex="0"
        >
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tabs_orientation_class() {
        assert_eq!(TabsOrientation::Horizontal.to_class(), "tabs-horizontal");
        assert_eq!(TabsOrientation::Vertical.to_class(), "tabs-vertical");
    }

    #[test]
    fn test_tabs_orientation_aria() {
        assert_eq!(TabsOrientation::Horizontal.to_aria(), "horizontal");
        assert_eq!(TabsOrientation::Vertical.to_aria(), "vertical");
    }

    #[test]
    fn test_tabs_props_default() {
        let props = TabsProps {
            value: None,
            default_value: None,
            on_value_change: None,
            orientation: TabsOrientation::Horizontal,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.orientation, TabsOrientation::Horizontal);
        assert!(props.value.is_none());
    }

    #[test]
    fn test_tabs_vertical() {
        let props = TabsProps {
            value: None,
            default_value: None,
            on_value_change: None,
            orientation: TabsOrientation::Vertical,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.orientation, TabsOrientation::Vertical);
    }

    #[test]
    fn test_tabs_trigger_disabled() {
        let props = TabsTriggerProps {
            value: AttrValue::from("tab1"),
            disabled: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_tabs_trigger_value() {
        let props = TabsTriggerProps {
            value: AttrValue::from("tab1"),
            disabled: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.value, AttrValue::from("tab1"));
    }

    #[test]
    fn test_roving_index_horizontal_wraps() {
        let h = TabsOrientation::Horizontal;
        assert_eq!(roving_index(0, 3, "ArrowRight", h), Some(1));
        assert_eq!(roving_index(2, 3, "ArrowRight", h), Some(0));
        assert_eq!(roving_index(0, 3, "ArrowLeft", h), Some(2));
        assert_eq!(roving_index(1, 3, "Home", h), Some(0));
        assert_eq!(roving_index(1, 3, "End", h), Some(2));
        assert_eq!(roving_index(1, 3, "ArrowDown", h), None);
        assert_eq!(roving_index(0, 0, "Home", h), None);
    }

    #[test]
    fn test_roving_index_vertical_uses_up_down() {
        let v = TabsOrientation::Vertical;
        assert_eq!(roving_index(0, 3, "ArrowDown", v), Some(1));
        assert_eq!(roving_index(0, 3, "ArrowUp", v), Some(2));
        assert_eq!(roving_index(0, 3, "ArrowRight", v), None);
    }

    #[test]
    fn test_tabs_context_ids_and_active() {
        let ctx = TabsContext {
            value: Some(AttrValue::from("a")),
            set_value: Callback::noop(),
            orientation: TabsOrientation::Horizontal,
            base_id: AttrValue::from("tabs-7"),
        };
        assert_eq!(ctx.trigger_id("a"), "tabs-7-trigger-a");
        assert_eq!(ctx.content_id("a"), "tabs-7-content-a");
        assert!(ctx.is_active("a"));
        assert!(!ctx.is_active("b"));
    }

    #[test]
    fn test_tabs_content_value() {
        let props = TabsContentProps {
            value: AttrValue::from("tab1"),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.value, AttrValue::from("tab1"));
    }
}
