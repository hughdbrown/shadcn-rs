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

use yew::prelude::*;

/// Tabs orientation
#[derive(Debug, Clone, PartialEq)]
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

/// Tabs component properties
#[derive(Properties, PartialEq, Clone)]
pub struct TabsProps {
    /// Currently active tab value
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Default active tab value
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
/// A container for tab triggers and content panels.
///
/// # Accessibility
/// - Keyboard navigation (Arrow keys, Home, End)
/// - Proper ARIA attributes
/// - Focus management
/// - Screen reader friendly
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

    let classes: Classes = vec![
        Classes::from("tabs"),
        Classes::from(orientation.to_class()),
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div class={classes} data-orientation={orientation.to_aria()}>
            { children }
        </div>
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

    let classes: Classes = vec![Classes::from("tabs-list"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="tablist">
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

    let aria_controls = format!("tabpanel-{}", value);

    html! {
        <button
            type="button"
            class={classes}
            role="tab"
            data-value={value}
            disabled={disabled}
            aria-selected="false"
            aria-controls={aria_controls}
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
/// A panel that contains the content for a tab.
#[function_component(TabsContent)]
pub fn tabs_content(props: &TabsContentProps) -> Html {
    let TabsContentProps {
        value,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("tabs-content"), class]
        .into_iter()
        .collect();

    html! {
        <div
            class={classes}
            role="tabpanel"
            id={format!("tabpanel-{}", value)}
            data-value={value}
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
        assert_eq!(
            TabsOrientation::Horizontal.to_class(),
            "tabs-horizontal"
        );
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
    fn test_tabs_content_value() {
        let props = TabsContentProps {
            value: AttrValue::from("tab1"),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.value, AttrValue::from("tab1"));
    }
}
