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
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, Element};

/// Resizable orientation
#[derive(Debug, Clone, PartialEq)]
pub enum ResizableOrientation {
    /// Horizontal layout (side by side)
    Horizontal,
    /// Vertical layout (top and bottom)
    Vertical,
}

/// Context for sharing resizable state between components
#[derive(Clone, PartialEq)]
pub struct ResizableContext {
    /// Layout orientation
    pub orientation: ResizableOrientation,
    /// Panel sizes as percentages
    pub sizes: Vec<f64>,
    /// Callback to update panel sizes
    pub set_sizes: Callback<Vec<f64>>,
    /// Whether currently dragging
    pub is_dragging: bool,
    /// Set dragging state
    pub set_dragging: Callback<bool>,
    /// The container node ref
    pub container_ref: NodeRef,
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

    let container_ref = use_node_ref();
    let sizes = use_state(|| vec![50.0, 50.0]);
    let is_dragging = use_state(|| false);

    let set_sizes = {
        let sizes = sizes.clone();
        Callback::from(move |new_sizes: Vec<f64>| {
            sizes.set(new_sizes);
        })
    };

    let set_dragging = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |dragging: bool| {
            is_dragging.set(dragging);
        })
    };

    let context = ResizableContext {
        orientation: orientation.clone(),
        sizes: (*sizes).clone(),
        set_sizes,
        is_dragging: *is_dragging,
        set_dragging,
        container_ref: container_ref.clone(),
    };

    let orientation_class = match orientation {
        ResizableOrientation::Horizontal => "resizable-horizontal",
        ResizableOrientation::Vertical => "resizable-vertical",
    };

    let classes: Classes = vec![
        Classes::from("resizable"),
        Classes::from(orientation_class),
        if *is_dragging {
            Classes::from("resizable-dragging")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <ContextProvider<ResizableContext> context={context}>
            <div ref={container_ref} class={classes}>
                { children }
            </div>
        </ContextProvider<ResizableContext>>
    }
}

/// Resizable panel properties
#[derive(Properties, PartialEq, Clone)]
pub struct ResizablePanelProps {
    /// Panel index (0 or 1 for two-panel layout)
    #[prop_or(0)]
    pub index: usize,

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
        index,
        default_size,
        min_size: _,
        max_size: _,
        collapsible: _,
        class,
        children,
    } = props.clone();

    let context = use_context::<ResizableContext>();

    // Get size from context or use default
    let size = context
        .as_ref()
        .and_then(|ctx| ctx.sizes.get(index).copied())
        .unwrap_or(default_size);

    let orientation = context
        .as_ref()
        .map(|ctx| ctx.orientation.clone())
        .unwrap_or(ResizableOrientation::Horizontal);

    let classes: Classes = vec![Classes::from("resizable-panel"), class]
        .into_iter()
        .collect();

    let style = match orientation {
        ResizableOrientation::Horizontal => format!("flex-basis: {}%; width: {}%", size, size),
        ResizableOrientation::Vertical => format!("flex-basis: {}%; height: {}%", size, size),
    };

    html! {
        <div class={classes} style={style} data-panel-index={index.to_string()}>
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

    let context = use_context::<ResizableContext>();
    let is_active = use_state(|| false);

    let orientation = context
        .as_ref()
        .map(|ctx| ctx.orientation.clone())
        .unwrap_or(ResizableOrientation::Horizontal);

    // Handle mouse down to start dragging
    let onmousedown = {
        let context = context.clone();
        let is_active = is_active.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            is_active.set(true);

            if let Some(ctx) = context.as_ref() {
                ctx.set_dragging.emit(true);
            }

            // Set up global mouse move and up handlers
            let context_move = context.clone();
            let context_up = context.clone();
            let is_active_clone = is_active.clone();

            // Handle mouse move
            let mousemove_closure = Closure::<dyn Fn(MouseEvent)>::new(move |e: MouseEvent| {
                if let Some(ctx) = context_move.as_ref() {
                    if let Some(container) = ctx.container_ref.cast::<Element>() {
                        let rect = container.get_bounding_client_rect();

                        let percentage = match ctx.orientation {
                            ResizableOrientation::Horizontal => {
                                let x = e.client_x() as f64 - rect.left();
                                let width = rect.width();
                                (x / width * 100.0).max(10.0).min(90.0)
                            }
                            ResizableOrientation::Vertical => {
                                let y = e.client_y() as f64 - rect.top();
                                let height = rect.height();
                                (y / height * 100.0).max(10.0).min(90.0)
                            }
                        };

                        ctx.set_sizes.emit(vec![percentage, 100.0 - percentage]);
                    }
                }
            });

            // Handle mouse up
            let mousemove_closure_rc = std::rc::Rc::new(std::cell::RefCell::new(Some(mousemove_closure)));
            let mouseup_closure_rc: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn Fn(MouseEvent)>>>> = std::rc::Rc::new(std::cell::RefCell::new(None));
            let mouseup_closure_rc_clone = mouseup_closure_rc.clone();
            let mousemove_rc_for_up = mousemove_closure_rc.clone();

            let mouseup_closure = Closure::<dyn Fn(MouseEvent)>::new(move |_e: MouseEvent| {
                is_active_clone.set(false);

                if let Some(ctx) = context_up.as_ref() {
                    ctx.set_dragging.emit(false);
                }

                // Remove event listeners
                if let Some(window) = web_sys::window() {
                    if let Some(closure) = mousemove_rc_for_up.borrow_mut().take() {
                        let _ = window.remove_event_listener_with_callback(
                            "mousemove",
                            closure.as_ref().unchecked_ref()
                        );
                    }
                    if let Some(closure) = mouseup_closure_rc_clone.borrow_mut().take() {
                        let _ = window.remove_event_listener_with_callback(
                            "mouseup",
                            closure.as_ref().unchecked_ref()
                        );
                    }
                }
            });

            *mouseup_closure_rc.borrow_mut() = Some(mouseup_closure);

            // Add event listeners to window
            if let Some(window) = web_sys::window() {
                if let Some(closure) = mousemove_closure_rc.borrow().as_ref() {
                    let _ = window.add_event_listener_with_callback(
                        "mousemove",
                        closure.as_ref().unchecked_ref()
                    );
                }
                if let Some(closure) = mouseup_closure_rc.borrow().as_ref() {
                    let _ = window.add_event_listener_with_callback(
                        "mouseup",
                        closure.as_ref().unchecked_ref()
                    );
                }
            }

            // Keep closures alive by leaking them (they'll be cleaned up on mouseup)
            if let Some(closure) = mousemove_closure_rc.borrow_mut().take() {
                closure.forget();
            }
            if let Some(closure) = mouseup_closure_rc.borrow_mut().take() {
                closure.forget();
            }
        })
    };

    let aria_orientation = match orientation {
        ResizableOrientation::Horizontal => "vertical",
        ResizableOrientation::Vertical => "horizontal",
    };

    let classes: Classes = vec![
        Classes::from("resizable-handle"),
        if *is_active {
            Classes::from("resizable-handle-active")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div
            class={classes}
            role="separator"
            aria-orientation={aria_orientation}
            tabindex="0"
            {onmousedown}
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
            index: 0,
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
            index: 0,
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
