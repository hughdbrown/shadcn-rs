//! Focus trapping for modal overlays.

use crate::utils::{active_element, focus_element, focus_first_within, trap_tab_navigation};
use web_sys::Element;
use yew::prelude::*;

/// Traps keyboard focus inside `content_ref` while `is_open` is true.
///
/// On open, focus moves to the first focusable descendant (or the container
/// itself). On close or unmount, focus returns to whatever was focused before
/// opening. Attach the returned callback as the container's `onkeydown` to
/// keep `Tab`/`Shift+Tab` cycling within the overlay.
///
/// # Examples
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::hooks::use_focus_trap;
///
/// #[function_component(Modal)]
/// fn modal() -> Html {
///     let content_ref = use_node_ref();
///     let onkeydown = use_focus_trap(content_ref.clone(), true);
///     html! {
///         <div ref={content_ref} role="dialog" tabindex="-1" {onkeydown}>
///             <button>{ "OK" }</button>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_focus_trap(content_ref: NodeRef, is_open: bool) -> Callback<KeyboardEvent> {
    {
        let content_ref = content_ref.clone();
        use_effect_with(is_open, move |is_open| {
            let previous_focus = if *is_open {
                let previous = active_element();
                if let Some(element) = content_ref.cast::<Element>() {
                    focus_first_within(&element);
                }
                previous
            } else {
                None
            };

            move || {
                if let Some(element) = previous_focus.as_ref() {
                    focus_element(element);
                }
            }
        });
    }

    use_callback(content_ref, |event: KeyboardEvent, content_ref| {
        if let Some(element) = content_ref.cast::<Element>() {
            trap_tab_navigation(&element, &event);
        }
    })
}
