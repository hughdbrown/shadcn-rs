//! useEscapeKey hook for handling Escape key press

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use yew::prelude::*;

/// Hook for handling Escape key press
///
/// # Examples
///
/// ```rust,ignore
/// use yew::prelude::*;
/// use shadcn_rs::use_escape_key;
///
/// #[function_component(Dialog)]
/// fn dialog() -> Html {
///     let is_open = use_state(|| true);
///
///     let close = {
///         let is_open = is_open.clone();
///         Callback::from(move |_| is_open.set(false))
///     };
///
///     use_escape_key(close);
///
///     html! {
///         <div class="dialog">
///             { "Press Escape to close" }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_escape_key<F>(callback: F)
where
    F: Fn() + 'static,
{
    use_effect(move || {
        let callback = std::rc::Rc::new(callback);
        let listener = EventListener::new(&gloo::utils::document(), "keydown", move |event| {
            if let Some(event) = event.dyn_ref::<KeyboardEvent>()
                && event.key() == "Escape"
            {
                callback();
            }
        });

        move || drop(listener)
    });
}

/// Hook for handling Escape key with enabled flag
///
/// Only listens for Escape when `enabled` is true.
///
/// # Examples
///
/// ```rust,ignore
/// use yew::prelude::*;
/// use shadcn_rs::use_escape_key_conditional;
///
/// #[function_component(Modal)]
/// fn modal() -> Html {
///     let is_open = use_state(|| false);
///
///     let close = {
///         let is_open = is_open.clone();
///         Callback::from(move |_| is_open.set(false))
///     };
///
///     // Only handle Escape when modal is open
///     use_escape_key_conditional(close, *is_open);
///
///     html! {
///         if *is_open {
///             <div class="modal">
///                 { "Modal content" }
///             </div>
///         }
///     }
/// }
/// ```
#[hook]
pub fn use_escape_key_conditional<F>(callback: F, enabled: bool)
where
    F: Fn() + 'static,
{
    use_effect_with(enabled, move |enabled| {
        let listener = if *enabled {
            let callback = std::rc::Rc::new(callback);
            Some(EventListener::new(
                &gloo::utils::document(),
                "keydown",
                move |event| {
                    if let Some(event) = event.dyn_ref::<KeyboardEvent>()
                        && event.key() == "Escape"
                    {
                        callback();
                    }
                },
            ))
        } else {
            None
        };

        move || drop(listener)
    });
}

/// Hook for handling any key press
///
/// # Examples
///
/// ```rust,ignore
/// use yew::prelude::*;
/// use shadcn_rs::use_key_press;
///
/// #[function_component(Component)]
/// fn component() -> Html {
///     use_key_press("Enter", || {
///         web_sys::console::log_1(&"Enter pressed!".into());
///     });
///
///     html! {
///         <div>{ "Press Enter" }</div>
///     }
/// }
/// ```
#[hook]
pub fn use_key_press<F>(key: &'static str, callback: F)
where
    F: Fn() + 'static,
{
    use_effect(move || {
        let callback = std::rc::Rc::new(callback);
        let listener = EventListener::new(&gloo::utils::document(), "keydown", move |event| {
            if let Some(event) = event.dyn_ref::<KeyboardEvent>()
                && event.key() == key
            {
                callback();
            }
        });

        move || drop(listener)
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_use_escape_key_compiles() {
        // This test just verifies the hook API compiles correctly
        // Actual behavior testing requires wasm-bindgen-test in a browser environment
    }

    #[test]
    fn test_use_escape_key_conditional_compiles() {
        // This test just verifies the hook API compiles correctly
        // Actual behavior testing requires wasm-bindgen-test in a browser environment
    }

    #[test]
    fn test_use_key_press_compiles() {
        // This test just verifies the hook API compiles correctly
        // Actual behavior testing requires wasm-bindgen-test in a browser environment
    }
}
