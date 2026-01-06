//! useToggle hook for boolean state management

use yew::prelude::*;

/// Hook for managing boolean toggle state
///
/// Returns a tuple of (current_state, toggle_function, set_function).
///
/// # Examples
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::use_toggle;
///
/// #[function_component(Component)]
/// fn component() -> Html {
///     let (is_open, toggle, set_open) = use_toggle(false);
///
///     html! {
///         <>
///             <button onclick={move |_| toggle()}>
///                 { if is_open { "Close" } else { "Open" } }
///             </button>
///             if is_open {
///                 <div>{ "Content is visible" }</div>
///             }
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_toggle(initial: bool) -> (bool, Callback<()>, Callback<bool>) {
    let state = use_state(|| initial);

    let toggle = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(!*state);
        })
    };

    let set = {
        let state = state.clone();
        Callback::from(move |value: bool| {
            state.set(value);
        })
    };

    (*state, toggle, set)
}

/// Hook for managing boolean toggle state with more control
///
/// Returns (current_state, toggle, set_true, set_false).
///
/// # Examples
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::use_toggle_with_controls;
///
/// #[function_component(Component)]
/// fn component() -> Html {
///     let (is_visible, toggle, show, hide) = use_toggle_with_controls(false);
///
///     html! {
///         <>
///             <button onclick={move |_| show()}>{ "Show" }</button>
///             <button onclick={move |_| hide()}>{ "Hide" }</button>
///             <button onclick={move |_| toggle()}>{ "Toggle" }</button>
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_toggle_with_controls(
    initial: bool,
) -> (bool, Callback<()>, Callback<()>, Callback<()>) {
    let state = use_state(|| initial);

    let toggle = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(!*state);
        })
    };

    let set_true = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(true);
        })
    };

    let set_false = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(false);
        })
    };

    (*state, toggle, set_true, set_false)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These are basic structure tests
    // Full integration tests require wasm-bindgen-test

    #[test]
    fn test_toggle_hook_compiles() {
        // This test just verifies the hook API compiles correctly
        // Actual behavior testing requires wasm-bindgen-test in a browser environment
    }

    #[test]
    fn test_toggle_with_controls_hook_compiles() {
        // This test just verifies the hook API compiles correctly
        // Actual behavior testing requires wasm-bindgen-test in a browser environment
    }
}
