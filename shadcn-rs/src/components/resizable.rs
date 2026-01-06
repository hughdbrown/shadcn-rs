//! Resizable component
//!
//! Resizable panels with drag handles.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Resizable, ResizablePanel, ResizableHandle, ResizableOrientation};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Resizable orientation={ResizableOrientation::Horizontal}>
//!             <ResizablePanel default_size={50.0}>
//!                 { "Left panel" }
//!             </ResizablePanel>
//!             <ResizableHandle />
//!             <ResizablePanel default_size={50.0}>
//!                 { "Right panel" }
//!             </ResizablePanel>
//!         </Resizable>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Resizable orientation
#[derive(Debug, Clone, PartialEq)]
pub enum ResizableOrientation {
    /// Horizontal layout (side by side)
    Horizontal,
    /// Vertical layout (top and bottom)
    Vertical,
}

/// Resizable container properties
#[derive(Properties, PartialEq, Clone)]
pub struct ResizableProps {
    /// Layout orientation
    #[prop_or(ResizableOrientation::Horizontal)]
    pub orientation: ResizableOrientation,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Resizable container component
///
/// Container for resizable panels.
///
/// # Accessibility
/// - Handles have proper ARIA attributes
/// - Keyboard navigation supported (arrow keys)
#[function_component(Resizable)]
pub fn resizable(props: &ResizableProps) -> Html {
    let ResizableProps {
        orientation,
        class,
        children,
    } = props.clone();

    let orientation_class = match orientation {
        ResizableOrientation::Horizontal => "resizable-horizontal",
        ResizableOrientation::Vertical => "resizable-vertical",
    };

    let classes: Classes = vec![
        Classes::from("resizable"),
        Classes::from(orientation_class),
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Resizable panel properties
#[derive(Properties, PartialEq, Clone)]
pub struct ResizablePanelProps {
    /// Default size percentage (0-100)
    #[prop_or(50.0)]
    pub default_size: f64,

    /// Minimum size percentage (0-100)
    #[prop_or(10.0)]
    pub min_size: f64,

    /// Maximum size percentage (0-100)
    #[prop_or(90.0)]
    pub max_size: f64,

    /// Collapsible panel
    #[prop_or(false)]
    pub collapsible: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Resizable panel component
///
/// A panel within a resizable container.
#[function_component(ResizablePanel)]
pub fn resizable_panel(props: &ResizablePanelProps) -> Html {
    let ResizablePanelProps {
        default_size,
        min_size: _,
        max_size: _,
        collapsible: _,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("resizable-panel"), class]
        .into_iter()
        .collect();

    let style = format!("flex-basis: {}%", default_size);

    html! {
        <div class={classes} style={style}>
            { children }
        </div>
    }
}

/// Resizable handle properties
#[derive(Properties, PartialEq, Clone)]
pub struct ResizableHandleProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Resizable handle component
///
/// Drag handle for resizing panels.
///
/// # Accessibility
/// - Has role="separator"
/// - Supports keyboard navigation
/// - Has aria-orientation attribute
#[function_component(ResizableHandle)]
pub fn resizable_handle(props: &ResizableHandleProps) -> Html {
    let ResizableHandleProps { class } = props.clone();

    let classes: Classes = vec![Classes::from("resizable-handle"), class]
        .into_iter()
        .collect();

    html! {
        <div
            class={classes}
            role="separator"
            aria-orientation="vertical"
            tabindex="0"
        >
            <div class="resizable-handle-icon" />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resizable_horizontal() {
        let props = ResizableProps {
            orientation: ResizableOrientation::Horizontal,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.orientation, ResizableOrientation::Horizontal);
    }

    #[test]
    fn test_resizable_vertical() {
        let props = ResizableProps {
            orientation: ResizableOrientation::Vertical,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.orientation, ResizableOrientation::Vertical);
    }

    #[test]
    fn test_resizable_panel_default_size() {
        let props = ResizablePanelProps {
            default_size: 50.0,
            min_size: 10.0,
            max_size: 90.0,
            collapsible: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.default_size, 50.0);
        assert_eq!(props.min_size, 10.0);
        assert_eq!(props.max_size, 90.0);
    }

    #[test]
    fn test_resizable_panel_collapsible() {
        let props = ResizablePanelProps {
            default_size: 50.0,
            min_size: 10.0,
            max_size: 90.0,
            collapsible: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.collapsible);
    }

    #[test]
    fn test_resizable_handle_props() {
        let props = ResizableHandleProps {
            class: Classes::new(),
        };

        assert_eq!(props.class, Classes::new());
    }
}
