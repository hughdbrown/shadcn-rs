//! useClickOutside hook for detecting clicks outside an element

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent as WebMouseEvent, Node};
use yew::prelude::*;

/// Hook for detecting clicks outside a referenced element
///
/// # Examples
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::use_click_outside;
///
/// #[function_component(Dropdown)]
/// fn dropdown() -> Html {
///     let is_open = use_state(|| false);
///     let dropdown_ref = use_node_ref();
///
///     let close = {
///         let is_open = is_open.clone();
///         Callback::from(move |_| is_open.set(false))
///     };
///
///     use_click_outside(dropdown_ref.clone(), close);
///
///     html! {
///         <div ref={dropdown_ref}>
///             if *is_open {
///                 <div class="dropdown-menu">
///                     { "Menu content" }
///                 </div>
///             }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_click_outside<F>(node_ref: NodeRef, callback: F)
where
    F: Fn() + 'static,
{
    use_effect_with(node_ref.clone(), move |node_ref| {
        let callback = std::rc::Rc::new(callback);
        let listener = {
            let node_ref = node_ref.clone();
            let callback = callback.clone();

            EventListener::new(&gloo::utils::document(), "mousedown", move |event| {
                if let Some(event) = event.dyn_ref::<WebMouseEvent>() {
                    if let Some(target) = event.target() {
                        if let Some(target_node) = target.dyn_ref::<Node>() {
                            if let Some(element) = node_ref.cast::<Element>() {
                                // Check if click target is outside the element
                                if !element.contains(Some(target_node)) {
                                    callback();
                                }
                            }
                        }
                    }
                }
            })
        };

        // Return cleanup function
        move || drop(listener)
    });
}

/// Hook for detecting clicks outside with enabled flag
///
/// Only listens for clicks when `enabled` is true.
///
/// # Examples
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::use_click_outside_conditional;
///
/// #[function_component(Modal)]
/// fn modal() -> Html {
///     let is_open = use_state(|| false);
///     let modal_ref = use_node_ref();
///
///     let close = {
///         let is_open = is_open.clone();
///         Callback::from(move |_| is_open.set(false))
///     };
///
///     // Only detect clicks outside when modal is open
///     use_click_outside_conditional(modal_ref.clone(), close, *is_open);
///
///     html! {
///         <div ref={modal_ref}>
///             { "Modal content" }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_click_outside_conditional<F>(node_ref: NodeRef, callback: F, enabled: bool)
where
    F: Fn() + 'static,
{
    use_effect_with((node_ref.clone(), enabled), move |(node_ref, enabled)| {
        let listener = if *enabled {
            let callback = std::rc::Rc::new(callback);
            let node_ref = node_ref.clone();
            let callback_clone = callback.clone();

            Some(EventListener::new(&gloo::utils::document(), "mousedown", move |event| {
                if let Some(event) = event.dyn_ref::<WebMouseEvent>() {
                    if let Some(target) = event.target() {
                        if let Some(target_node) = target.dyn_ref::<Node>() {
                            if let Some(element) = node_ref.cast::<Element>() {
                                if !element.contains(Some(target_node)) {
                                    callback_clone();
                                }
                            }
                        }
                    }
                }
            }))
        } else {
            None
        };

        move || drop(listener)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_click_outside_compiles() {
        // This test just verifies the hook API compiles correctly
        // Actual behavior testing requires wasm-bindgen-test in a browser environment
    }

    #[test]
    fn test_use_click_outside_conditional_compiles() {
        // This test just verifies the hook API compiles correctly
        // Actual behavior testing requires wasm-bindgen-test in a browser environment
    }
}
