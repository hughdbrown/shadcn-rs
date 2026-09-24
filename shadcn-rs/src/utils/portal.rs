//! Portal utility for rendering components outside the DOM hierarchy
//!
//! Portals allow you to render children into a DOM node that exists outside
//! the DOM hierarchy of the parent component. This is useful for modals,
//! tooltips, and other overlay components.

use wasm_bindgen::JsCast;
use web_sys::{Element, Node, window};
use yew::prelude::*;

/// Portal component for rendering children to document.body
///
/// # Examples
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::Portal;
///
/// #[function_component(Modal)]
/// fn modal() -> Html {
///     html! {
///         <Portal>
///             <div class="modal-overlay">
///                 <div class="modal-content">
///                     { "This renders in document.body" }
///                 </div>
///             </div>
///         </Portal>
///     }
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct PortalProps {
    /// Children to render in the portal
    pub children: Children,

    /// Optional target element ID to render into
    /// If None, renders to document.body
    #[prop_or_default]
    pub target: Option<AttrValue>,
}

/// Get the portal target element
fn get_portal_target(target_id: Option<&str>) -> Option<Element> {
    let window = window()?;
    let document = window.document()?;

    if let Some(id) = target_id {
        document.get_element_by_id(id)
    } else {
        document.body().map(|body| body.into())
    }
}

/// Portal component implementation
#[function_component(Portal)]
pub fn portal(props: &PortalProps) -> Html {
    let target_ref = use_state(|| None::<Element>);

    // Set up the portal target
    {
        let target_ref = target_ref.clone();
        let target_id = props.target.as_ref().map(|s| s.to_string());

        use_effect_with(target_id.clone(), move |_| {
            if let Some(element) = get_portal_target(target_id.as_deref()) {
                target_ref.set(Some(element));
            }

            || ()
        });
    }

    // Render children into portal or fallback to normal rendering
    if let Some(target) = (*target_ref).as_ref() {
        create_portal(props.children.clone(), target.clone().into())
    } else {
        // Portal target not resolved yet; render nothing until it's available
        Html::default()
    }
}

/// Create a portal using Yew's VNode and web-sys
///
/// This is a low-level utility. Prefer using the Portal component.
pub fn create_portal(children: Children, target: Node) -> Html {
    // Use Yew's create_portal when rendering
    // For now, we'll use a custom implementation that leverages use_effect
    html! {
        <PortalInner {target}>
            { children }
        </PortalInner>
    }
}

#[derive(Properties, PartialEq)]
struct PortalInnerProps {
    children: Children,
    target: Node,
}

/// Renders children through Yew's native portal so delegated event handlers
/// (`onclick`, `onkeydown`, ...) keep working after the subtree leaves the app root.
#[function_component(PortalInner)]
fn portal_inner(props: &PortalInnerProps) -> Html {
    let Some(host) = props.target.dyn_ref::<Element>() else {
        return Html::default();
    };

    yew::create_portal(
        html! {
            <div class="portal-container">
                { props.children.clone() }
            </div>
        },
        host.clone(),
    )
}

/// Hook for managing portal state
///
/// # Examples
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::{use_portal, Portal};
///
/// #[function_component(Component)]
/// fn component() -> Html {
///     let portal_open = use_state(|| false);
///     let portal_ref = use_portal();
///
///     html! {
///         <>
///             <button onclick={let portal_open = portal_open.clone(); Callback::from(move |_| portal_open.set(true))}>
///                 { "Open Portal" }
///             </button>
///             if *portal_open {
///                 <Portal>
///                     { "Portal content" }
///                 </Portal>
///             }
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_portal() -> UseStateHandle<Option<Element>> {
    use_state(|| get_portal_target(None))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_portal_props_target() {
        // Test that portal props can be created with different target values
        // Note: Full portal behavior testing requires wasm-bindgen-test in a browser environment
    }

    #[test]
    fn test_portal_with_custom_target() {
        // Test that custom target IDs can be set
        // Note: Full portal behavior testing requires wasm-bindgen-test in a browser environment
    }
}
