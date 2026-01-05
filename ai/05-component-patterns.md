# Component Implementation Patterns

This guide provides standard patterns and templates for implementing shadcn-rs components.

## Component Template

Use this template as a starting point for new components:

```rust
//! Component Name
//!
//! Brief description of what this component does.
//!
//! # Example
//!
//! ```rust
//! use shadcn_rs::ComponentName;
//! use yew::prelude::*;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <ComponentName variant={Variant::Primary}>
//!             { "Content" }
//!         </ComponentName>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::types::{Size, Variant};
use crate::utils::classes;

/// Properties for the ComponentName component.
#[derive(Properties, PartialEq)]
pub struct ComponentNameProps {
    /// The visual variant of the component.
    #[prop_or_default]
    pub variant: Variant,

    /// The size of the component.
    #[prop_or_default]
    pub size: Size,

    /// Whether the component is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Additional CSS classes to apply.
    #[prop_or_default]
    pub class: Classes,

    /// Click event handler.
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Child elements.
    #[prop_or_default]
    pub children: Children,

    /// ARIA label for accessibility.
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
}

/// A component that does something useful.
///
/// # Accessibility
///
/// - Uses semantic HTML elements
/// - Supports keyboard navigation
/// - Includes proper ARIA attributes
///
/// # Example
///
/// ```rust
/// html! {
///     <ComponentName
///         variant={Variant::Primary}
///         size={Size::Lg}
///         onclick={handle_click}
///     >
///         { "Click me" }
///     </ComponentName>
/// }
/// ```
#[function_component(ComponentName)]
pub fn component_name(props: &ComponentNameProps) -> Html {
    let class = classes!(
        "component-name",
        props.variant.to_class(),
        props.size.to_class(),
        props.disabled.then_some("component-name-disabled"),
        props.class.clone(),
    );

    let onclick = {
        let callback = props.onclick.clone();
        let disabled = props.disabled;

        Callback::from(move |e: MouseEvent| {
            if !disabled {
                if let Some(cb) = &callback {
                    cb.emit(e);
                }
            }
        })
    };

    html! {
        <div
            class={class}
            onclick={onclick}
            aria-disabled={props.disabled.to_string()}
            aria-label={props.aria_label.clone()}
        >
            { props.children.clone() }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn renders_with_defaults() {
        // Test implementation
    }

    #[wasm_bindgen_test]
    fn renders_all_variants() {
        // Test implementation
    }

    #[wasm_bindgen_test]
    fn handles_onclick() {
        // Test implementation
    }

    #[wasm_bindgen_test]
    fn handles_disabled_state() {
        // Test implementation
    }

    #[wasm_bindgen_test]
    fn has_accessibility_attributes() {
        // Test implementation
    }
}
```

## Common Prop Patterns

### Standard Props

Most components should include these common props:

```rust
#[derive(Properties, PartialEq)]
pub struct StandardProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Inline styles (use sparingly)
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// HTML id attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// ARIA label
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// Test identifier
    #[prop_or_default]
    pub data_testid: Option<AttrValue>,
}
```

### Size and Variant Props

```rust
/// Size of the component
#[prop_or_default]
pub size: Size,

/// Visual variant
#[prop_or_default]
pub variant: Variant,
```

### Children Props

```rust
/// Child elements (for container components)
#[prop_or_default]
pub children: Children,

/// Single child element
#[prop_or_default]
pub child: Html,
```

### State Props (Controlled/Uncontrolled)

```rust
/// Controlled value
#[prop_or_default]
pub value: Option<String>,

/// Default value (uncontrolled)
#[prop_or_default]
pub default_value: Option<String>,

/// Change callback
#[prop_or_default]
pub onchange: Option<Callback<String>>,

/// Controlled open state
#[prop_or_default]
pub open: Option<bool>,

/// Default open state (uncontrolled)
#[prop_or_default]
pub default_open: bool,

/// Open change callback
#[prop_or_default]
pub on_open_change: Option<Callback<bool>>,
```

### Event Handler Props

```rust
/// Click handler
#[prop_or_default]
pub onclick: Option<Callback<MouseEvent>>,

/// Keyboard handler
#[prop_or_default]
pub onkeydown: Option<Callback<KeyboardEvent>>,

/// Focus handler
#[prop_or_default]
pub onfocus: Option<Callback<FocusEvent>>,

/// Blur handler
#[prop_or_default]
pub onblur: Option<Callback<FocusEvent>>,

/// Input handler (for form controls)
#[prop_or_default]
pub oninput: Option<Callback<InputEvent>>,
```

## Hook Patterns

### useControllableState Hook

For components supporting controlled/uncontrolled patterns:

```rust
use yew::prelude::*;

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

    let value = controlled_value
        .as_ref()
        .unwrap_or(&*internal_state)
        .clone();

    let set_value = {
        let internal_state = internal_state.clone();
        let is_controlled = controlled_value.is_some();

        Callback::from(move |new_value: T| {
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
```

### useToggle Hook

For boolean state:

```rust
#[hook]
pub fn use_toggle(initial: bool) -> (bool, Callback<()>) {
    let state = use_state(|| initial);

    let toggle = {
        let state = state.clone();
        Callback::from(move |_| state.set(!*state))
    };

    (*state, toggle)
}
```

### useClickOutside Hook

For detecting clicks outside an element:

```rust
use web_sys::MouseEvent as WebMouseEvent;

#[hook]
pub fn use_click_outside<F>(node_ref: NodeRef, callback: F)
where
    F: Fn() + 'static,
{
    use_effect_with(node_ref.clone(), move |node_ref| {
        let callback = Rc::new(callback);
        let listener = {
            let node_ref = node_ref.clone();
            let callback = callback.clone();

            EventListener::new(&gloo::utils::document(), "mousedown", move |e| {
                let event = e.dyn_ref::<WebMouseEvent>().unwrap();
                let target = event.target().unwrap();

                if let Some(element) = node_ref.cast::<Element>() {
                    if !element.contains(target.dyn_ref::<Node>()) {
                        callback();
                    }
                }
            })
        };

        move || drop(listener)
    });
}
```

### useEscapeKey Hook

For handling Escape key:

```rust
#[hook]
pub fn use_escape_key<F>(callback: F)
where
    F: Fn() + 'static,
{
    use_effect(move || {
        let callback = Rc::new(callback);
        let listener = EventListener::new(&gloo::utils::document(), "keydown", move |e| {
            let event = e.dyn_ref::<KeyboardEvent>().unwrap();
            if event.key() == "Escape" {
                callback();
            }
        });

        move || drop(listener)
    });
}
```

## Compound Component Pattern

For components with sub-components (Card, Dialog, etc.):

```rust
// Main component
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="card">
            { props.children.clone() }
        </div>
    }
}

// Sub-component
#[function_component(CardHeader)]
pub fn card_header(props: &CardHeaderProps) -> Html {
    html! {
        <div class="card-header">
            { props.children.clone() }
        </div>
    }
}

// Usage
html! {
    <Card>
        <CardHeader>
            <CardTitle>{ "Title" }</CardTitle>
        </CardHeader>
        <CardContent>
            { "Content" }
        </CardContent>
    </Card>
}
```

## Context Pattern

For sharing state across components (Theme, Form, etc.):

```rust
// Context type
#[derive(Clone, PartialEq)]
pub struct FormContext {
    pub register_field: Callback<String>,
    pub unregister_field: Callback<String>,
    pub get_error: Callback<String, Option<String>>,
}

// Provider component
#[function_component(FormProvider)]
pub fn form_provider(props: &FormProviderProps) -> Html {
    let fields = use_state(|| Vec::new());

    let context = FormContext {
        register_field: {
            let fields = fields.clone();
            Callback::from(move |field| {
                let mut new_fields = (*fields).clone();
                new_fields.push(field);
                fields.set(new_fields);
            })
        },
        // ... other methods
    };

    html! {
        <ContextProvider<FormContext> context={context}>
            { props.children.clone() }
        </ContextProvider<FormContext>>
    }
}

// Consumer hook
#[hook]
pub fn use_form_context() -> FormContext {
    use_context::<FormContext>().expect("FormProvider not found")
}
```

## Accessibility Patterns

### Keyboard Navigation

```rust
let onkeydown = Callback::from(|e: KeyboardEvent| {
    match e.key().as_str() {
        "Enter" | " " => {
            e.prevent_default();
            // Handle activation
        }
        "Escape" => {
            e.prevent_default();
            // Handle close/cancel
        }
        "ArrowDown" => {
            e.prevent_default();
            // Handle next
        }
        "ArrowUp" => {
            e.prevent_default();
            // Handle previous
        }
        "Home" => {
            e.prevent_default();
            // Handle first
        }
        "End" => {
            e.prevent_default();
            // Handle last
        }
        _ => {}
    }
});
```

### Focus Management

```rust
// Auto-focus on mount
let input_ref = use_node_ref();

use_effect_with(input_ref.clone(), |input_ref| {
    if let Some(input) = input_ref.cast::<HtmlElement>() {
        input.focus().ok();
    }
    || ()
});

// Focus trap for modals
let container_ref = use_node_ref();
let focus_trap = use_focus_trap(container_ref.clone(), is_open);
```

### ARIA Attributes

```rust
html! {
    <button
        role="button"
        aria-label={props.aria_label.clone()}
        aria-disabled={props.disabled.to_string()}
        aria-pressed={props.pressed.map(|p| p.to_string())}
        aria-expanded={props.expanded.map(|e| e.to_string())}
        aria-haspopup={props.has_popup.then_some("true")}
        aria-controls={props.controls.clone()}
        aria-describedby={props.described_by.clone()}
    >
        { props.children.clone() }
    </button>
}
```

## Testing Patterns

### Basic Render Test

```rust
#[wasm_bindgen_test]
fn renders_correctly() {
    let div = gloo::utils::document().create_element("div").unwrap();

    yew::Renderer::<Component>::with_root_and_props(
        div.clone(),
        ComponentProps {
            children: html! { "Test" }.into(),
            ..Default::default()
        },
    ).render();

    assert!(div.inner_html().contains("Test"));
}
```

### Event Handler Test

```rust
#[wasm_bindgen_test]
fn handles_click() {
    let clicked = Rc::new(Cell::new(false));
    let clicked_clone = clicked.clone();

    let div = gloo::utils::document().create_element("div").unwrap();

    yew::Renderer::<Component>::with_root_and_props(
        div.clone(),
        ComponentProps {
            onclick: Some(Callback::from(move |_| {
                clicked_clone.set(true);
            })),
            ..Default::default()
        },
    ).render();

    let button = div.query_selector("button").unwrap().unwrap();
    button.dyn_into::<HtmlElement>().unwrap().click();

    assert!(clicked.get());
}
```

### Accessibility Test

```rust
#[wasm_bindgen_test]
fn has_correct_aria_attributes() {
    let div = gloo::utils::document().create_element("div").unwrap();

    yew::Renderer::<Component>::with_root_and_props(
        div.clone(),
        ComponentProps {
            disabled: true,
            ..Default::default()
        },
    ).render();

    let button = div.query_selector("button").unwrap().unwrap();
    assert_eq!(
        button.get_attribute("aria-disabled"),
        Some("true".to_string())
    );
}
```

## CSS Class Naming Convention

Follow BEM-style naming:

```
.component-name { }                          /* Block */
.component-name__element { }                 /* Element */
.component-name--modifier { }                /* Modifier */
.component-name.variant-primary { }          /* Variant */
.component-name.size-lg { }                  /* Size */
.component-name.is-disabled { }              /* State */
```

Example for Button:

```css
.btn { /* Base styles */ }
.btn__icon { /* Icon styles */ }
.btn--loading { /* Loading state */ }
.btn.variant-primary { /* Primary variant */ }
.btn.size-lg { /* Large size */ }
.btn.is-disabled { /* Disabled state */ }
```

## Documentation Comments

Every public item should have documentation:

```rust
/// A button component for triggering actions.
///
/// Buttons communicate actions that users can take. They are typically placed
/// throughout your UI, in places like:
/// - Dialogs
/// - Modal windows
/// - Forms
/// - Cards
/// - Toolbars
///
/// # Accessibility
///
/// - Uses semantic `<button>` element
/// - Supports keyboard activation (Enter, Space)
/// - Includes `aria-disabled` when disabled
/// - Focus visible by default
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use shadcn_rs::{Button, Variant};
/// use yew::prelude::*;
///
/// html! {
///     <Button variant={Variant::Primary} onclick={handle_click}>
///         { "Click me" }
///     </Button>
/// }
/// ```
///
/// With loading state:
///
/// ```rust
/// html! {
///     <Button loading={true}>
///         { "Loading..." }
///     </Button>
/// }
/// ```
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    // Implementation
}
```

## Error Handling

```rust
// Validation errors - use callbacks
if let Some(validator) = &props.validate {
    match validator.emit(value.clone()) {
        Ok(_) => set_error(None),
        Err(e) => set_error(Some(e)),
    }
}

// Developer errors - panic with helpful message
assert!(
    !props.items.is_empty(),
    "Select component requires at least one item"
);

// Log warnings for non-critical issues
if props.value.is_some() && props.default_value.is_some() {
    web_sys::console::warn_1(
        &"Both 'value' and 'default_value' provided. 'value' will be used.".into()
    );
}
```

## Performance Optimization

```rust
// Use PartialEq to prevent unnecessary re-renders
#[derive(Properties, PartialEq)]
pub struct OptimizedProps {
    // Props
}

// Memoize expensive computations
let sorted_items = use_memo(
    |items| {
        let mut sorted = items.clone();
        sorted.sort();
        sorted
    },
    props.items.clone(),
);

// Avoid cloning in loops
for item in props.items.iter() {
    // Use references instead of cloning
}
```

## Component Checklist

Before marking a component as complete, verify:

- [ ] Props use `#[derive(Properties, PartialEq)]`
- [ ] All props have `#[prop_or_default]` or are required
- [ ] Component has rustdoc comment with examples
- [ ] All variants and sizes are implemented
- [ ] Accessibility attributes are present
- [ ] Keyboard navigation works
- [ ] Tests cover all major functionality
- [ ] Example exists and works
- [ ] CSS classes follow naming convention
- [ ] Component exported from lib.rs
- [ ] No compiler warnings
- [ ] No clippy warnings
