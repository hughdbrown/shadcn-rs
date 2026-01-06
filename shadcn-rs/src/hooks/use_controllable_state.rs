//! useControllableState hook for controlled/uncontrolled pattern

use yew::prelude::*;

/// Hook for managing state that can be controlled or uncontrolled
///
/// This hook allows components to work in both controlled mode (value provided by parent)
/// and uncontrolled mode (value managed internally).
///
/// # Examples
///
/// ## Controlled usage
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::use_controllable_state;
///
/// #[function_component(Parent)]
/// fn parent() -> Html {
///     let value = use_state(|| "controlled".to_string());
///     let on_change = {
///         let value = value.clone();
///         Callback::from(move |new_value: String| value.set(new_value))
///     };
///
///     html! {
///         <Input
///             value={Some((*value).clone())}
///             on_change={on_change}
///         />
///     }
/// }
/// ```
///
/// ## Uncontrolled usage
///
/// ```rust,no_run
/// use yew::prelude::*;
///
/// #[function_component(Parent)]
/// fn parent() -> Html {
///     html! {
///         <Input
///             default_value={"uncontrolled".to_string()}
///         />
///     }
/// }
/// ```
#[hook]
pub fn use_controllable_state<T>(
    controlled_value: Option<T>,
    default_value: T,
    on_change: Option<Callback<T>>,
) -> (T, Callback<T>)
where
    T: Clone + PartialEq + 'static,
{
    let internal_state = use_state(|| default_value);

    // Use controlled value if provided, otherwise use internal state
    let value = controlled_value
        .as_ref()
        .unwrap_or(&*internal_state)
        .clone();

    let set_value = {
        let internal_state = internal_state.clone();
        let is_controlled = controlled_value.is_some();

        Callback::from(move |new_value: T| {
            // Update internal state if uncontrolled
            if !is_controlled {
                internal_state.set(new_value.clone());
            }

            // Always notify parent via callback
            if let Some(callback) = &on_change {
                callback.emit(new_value);
            }
        })
    };

    (value, set_value)
}

/// Hook for managing optional controllable state
///
/// Similar to `use_controllable_state` but the value can be `None`.
///
/// # Examples
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::use_controllable_state_optional;
///
/// #[function_component(Select)]
/// fn select() -> Html {
///     let (value, set_value) = use_controllable_state_optional::<String>(
///         None, // No controlled value
///         None, // No default value
///         None, // No change callback
///     );
///
///     html! {
///         <div>
///             { format!("Selected: {:?}", value) }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_controllable_state_optional<T>(
    controlled_value: Option<T>,
    default_value: Option<T>,
    on_change: Option<Callback<Option<T>>>,
) -> (Option<T>, Callback<Option<T>>)
where
    T: Clone + PartialEq + 'static,
{
    let internal_state = use_state(|| default_value);
    let is_controlled = controlled_value.is_some();

    let value = if is_controlled {
        controlled_value
    } else {
        (*internal_state).clone()
    };

    let set_value = {
        let internal_state = internal_state.clone();

        Callback::from(move |new_value: Option<T>| {
            if !is_controlled {
                internal_state.set(new_value.clone());
            }

            if let Some(callback) = &on_change {
                callback.emit(new_value);
            }
        })
    };

    (value, set_value)
}

/// Hook for boolean controllable state
///
/// Specialized version for boolean values (commonly used for open/closed state).
///
/// # Examples
///
/// ```rust,no_run
/// use yew::prelude::*;
/// use shadcn_rs::use_controllable_bool;
///
/// #[derive(Properties, PartialEq)]
/// pub struct DialogProps {
///     #[prop_or_default]
///     pub open: Option<bool>,
///
///     #[prop_or_default]
///     pub default_open: bool,
///
///     #[prop_or_default]
///     pub on_open_change: Option<Callback<bool>>,
/// }
///
/// #[function_component(Dialog)]
/// fn dialog(props: &DialogProps) -> Html {
///     let (is_open, set_open) = use_controllable_bool(
///         props.open,
///         props.default_open,
///         props.on_open_change.clone(),
///     );
///
///     html! {
///         if is_open {
///             <div class="dialog">
///                 { "Dialog content" }
///             </div>
///         }
///     }
/// }
/// ```
#[hook]
pub fn use_controllable_bool(
    controlled_value: Option<bool>,
    default_value: bool,
    on_change: Option<Callback<bool>>,
) -> (bool, Callback<bool>) {
    use_controllable_state(controlled_value, default_value, on_change)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_controllable_state_compiles() {
        // This test just verifies the hook API compiles correctly
        // Actual behavior testing requires wasm-bindgen-test in a browser environment
    }

    #[test]
    fn test_use_controllable_state_optional_compiles() {
        // This test just verifies the hook API compiles correctly
        // Actual behavior testing requires wasm-bindgen-test in a browser environment
    }

    #[test]
    fn test_use_controllable_bool_compiles() {
        // This test just verifies the hook API compiles correctly
        // Actual behavior testing requires wasm-bindgen-test in a browser environment
    }
}
