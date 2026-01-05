# Architecture Design - shadcn-rs

## Project Structure

```
shadcn-rs/
├── Cargo.toml                 # Workspace manifest
├── README.md
├── LICENSE
├── CHANGELOG.md
│
├── shadcn-rs/                 # Library crate
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs            # Main library entry point
│   │   ├── components/       # All UI components
│   │   │   ├── mod.rs
│   │   │   ├── badge.rs
│   │   │   ├── button.rs
│   │   │   ├── input.rs
│   │   │   └── ...
│   │   ├── types/            # Shared type definitions
│   │   │   ├── mod.rs
│   │   │   ├── size.rs
│   │   │   ├── variant.rs
│   │   │   └── ...
│   │   ├── hooks/            # Custom Yew hooks
│   │   │   ├── mod.rs
│   │   │   ├── use_toggle.rs
│   │   │   ├── use_click_outside.rs
│   │   │   └── ...
│   │   ├── utils/            # Utility functions
│   │   │   ├── mod.rs
│   │   │   ├── class_name.rs
│   │   │   ├── aria.rs
│   │   │   └── ...
│   │   └── theme/            # Theme context and utilities
│   │       ├── mod.rs
│   │       └── provider.rs
│   ├── styles/               # CSS files
│   │   ├── base.css
│   │   ├── components.css
│   │   ├── utilities.css
│   │   └── themes.css
│   ├── tests/                # Integration tests
│   │   └── ...
│   └── examples/             # Component examples
│       ├── button/
│       ├── input/
│       └── ...
│
└── shadcn-showcase/          # Showcase application crate
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs           # App entry point
    │   ├── app.rs            # Main app component
    │   ├── router.rs         # Routing setup
    │   └── pages/            # Page components
    │       ├── mod.rs
    │       ├── home.rs
    │       ├── component_page.rs
    │       └── ...
    ├── static/               # Static assets
    │   └── index.html
    └── dist/                 # Built WASM output
```

## Component Architecture

### Component Structure Pattern

Each component follows this structure:

```rust
use yew::prelude::*;

// Props definition
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: Variant,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

// Component implementation
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = classes!(
        "btn",
        props.variant.to_class(),
        props.size.to_class(),
        props.disabled.then_some("btn-disabled"),
        props.class.clone(),
    );

    html! {
        <button
            class={class}
            disabled={props.disabled}
            onclick={props.onclick.clone()}
            aria-disabled={props.disabled.to_string()}
        >
            { props.children.clone() }
        </button>
    }
}
```

### Type System

#### Size Enum
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Xl2,
}

impl Size {
    pub fn to_class(&self) -> &'static str {
        match self {
            Size::Xs => "size-xs",
            Size::Sm => "size-sm",
            Size::Md => "size-md",
            Size::Lg => "size-lg",
            Size::Xl => "size-xl",
            Size::Xl2 => "size-2xl",
        }
    }
}
```

#### Variant Enum
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Primary,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

impl Variant {
    pub fn to_class(&self) -> &'static str {
        match self {
            Variant::Default => "variant-default",
            Variant::Primary => "variant-primary",
            Variant::Secondary => "variant-secondary",
            Variant::Destructive => "variant-destructive",
            Variant::Outline => "variant-outline",
            Variant::Ghost => "variant-ghost",
            Variant::Link => "variant-link",
        }
    }
}
```

## Styling System

### CSS Variable Structure

```css
:root {
    /* Color palette */
    --color-background: 0 0% 100%;
    --color-foreground: 222.2 84% 4.9%;
    --color-primary: 222.2 47.4% 11.2%;
    --color-primary-foreground: 210 40% 98%;
    --color-secondary: 210 40% 96.1%;
    --color-secondary-foreground: 222.2 47.4% 11.2%;
    --color-destructive: 0 84.2% 60.2%;
    --color-destructive-foreground: 210 40% 98%;
    --color-muted: 210 40% 96.1%;
    --color-muted-foreground: 215.4 16.3% 46.9%;
    --color-accent: 210 40% 96.1%;
    --color-accent-foreground: 222.2 47.4% 11.2%;
    --color-border: 214.3 31.8% 91.4%;
    --color-input: 214.3 31.8% 91.4%;
    --color-ring: 222.2 84% 4.9%;

    /* Spacing */
    --spacing-xs: 0.25rem;
    --spacing-sm: 0.5rem;
    --spacing-md: 1rem;
    --spacing-lg: 1.5rem;
    --spacing-xl: 2rem;

    /* Border radius */
    --radius: 0.5rem;
    --radius-sm: 0.25rem;
    --radius-md: 0.5rem;
    --radius-lg: 0.75rem;

    /* Typography */
    --font-sans: system-ui, -apple-system, sans-serif;
    --font-mono: ui-monospace, monospace;
}

/* Dark mode */
.dark {
    --color-background: 222.2 84% 4.9%;
    --color-foreground: 210 40% 98%;
    --color-primary: 210 40% 98%;
    --color-primary-foreground: 222.2 47.4% 11.2%;
    /* ... other dark mode colors */
}
```

### Utility Classes

```css
/* Sizes */
.size-xs { height: 1.5rem; font-size: 0.75rem; }
.size-sm { height: 2rem; font-size: 0.875rem; }
.size-md { height: 2.5rem; font-size: 1rem; }
.size-lg { height: 3rem; font-size: 1.125rem; }
.size-xl { height: 3.5rem; font-size: 1.25rem; }

/* Component-specific classes */
.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius);
    font-weight: 500;
    transition: all 0.2s;
}

.btn-disabled {
    opacity: 0.5;
    pointer-events: none;
}

.variant-primary {
    background: hsl(var(--color-primary));
    color: hsl(var(--color-primary-foreground));
}
/* ... other variants */
```

## State Management

### Controlled/Uncontrolled Pattern

Components support both controlled and uncontrolled patterns:

```rust
#[derive(Properties, PartialEq)]
pub struct InputProps {
    /// Controlled value
    #[prop_or_default]
    pub value: Option<String>,

    /// Uncontrolled default value
    #[prop_or_default]
    pub default_value: Option<String>,

    #[prop_or_default]
    pub onchange: Option<Callback<String>>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let internal_state = use_state(|| props.default_value.clone().unwrap_or_default());

    let value = props.value.as_ref().unwrap_or_else(|| &*internal_state);

    let oninput = {
        let onchange = props.onchange.clone();
        let internal_state = internal_state.clone();
        let is_controlled = props.value.is_some();

        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            let value = target.value();

            if !is_controlled {
                internal_state.set(value.clone());
            }

            if let Some(callback) = &onchange {
                callback.emit(value);
            }
        })
    };

    html! {
        <input
            type="text"
            value={value.clone()}
            oninput={oninput}
        />
    }
}
```

### Theme Context

```rust
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: Theme,
    pub set_theme: Callback<Theme>,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &PropsWithChildren) -> Html {
    let theme = use_state(|| Theme::System);

    let context = ThemeContext {
        theme: (*theme).clone(),
        set_theme: {
            let theme = theme.clone();
            Callback::from(move |new_theme| theme.set(new_theme))
        },
    };

    // Apply theme to document
    use_effect_with(theme.clone(), |theme| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        match **theme {
            Theme::Light => {
                body.class_list().remove_1("dark").ok();
            }
            Theme::Dark => {
                body.class_list().add_1("dark").ok();
            }
            Theme::System => {
                // Detect system preference
                // Implementation details...
            }
        }

        || ()
    });

    html! {
        <ContextProvider<ThemeContext> context={context}>
            { props.children.clone() }
        </ContextProvider<ThemeContext>>
    }
}

// Hook to use theme
#[hook]
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("ThemeProvider not found")
}
```

## Accessibility Patterns

### Focus Management

```rust
use web_sys::HtmlElement;
use yew::prelude::*;

#[hook]
pub fn use_focus_trap(is_active: bool) -> NodeRef {
    let container_ref = use_node_ref();

    use_effect_with((container_ref.clone(), is_active), |(container_ref, is_active)| {
        if !*is_active {
            return || ();
        }

        let container: HtmlElement = container_ref.cast().unwrap();

        // Find all focusable elements
        let focusable_selector = "a[href], button:not([disabled]), textarea:not([disabled]), \
                                  input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex='-1'])";

        let focusable_elements = container
            .query_selector_all(focusable_selector)
            .unwrap();

        // Trap focus logic
        // Implementation details...

        || ()
    });

    container_ref
}
```

### Keyboard Navigation

```rust
#[hook]
pub fn use_keyboard_navigation(
    items: Vec<NodeRef>,
    orientation: Orientation,
) -> impl Fn(KeyboardEvent) {
    let current_index = use_state(|| 0);

    Callback::from(move |e: KeyboardEvent| {
        let key = e.key();

        match (orientation, key.as_str()) {
            (Orientation::Horizontal, "ArrowLeft") | (Orientation::Vertical, "ArrowUp") => {
                e.prevent_default();
                // Focus previous item
            }
            (Orientation::Horizontal, "ArrowRight") | (Orientation::Vertical, "ArrowDown") => {
                e.prevent_default();
                // Focus next item
            }
            ("Home", _) => {
                e.prevent_default();
                // Focus first item
            }
            ("End", _) => {
                e.prevent_default();
                // Focus last item
            }
            _ => {}
        }
    })
}
```

## Testing Strategy

### Component Testing Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yew::functional::use_state;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_button_renders() {
        let div = gloo::utils::document().create_element("div").unwrap();

        yew::Renderer::<Button>::with_root_and_props(
            div.clone(),
            ButtonProps {
                children: html! { "Click me" }.into(),
                ..Default::default()
            },
        ).render();

        assert_eq!(div.inner_html().contains("Click me"), true);
    }

    #[wasm_bindgen_test]
    fn test_button_variants() {
        // Test each variant renders with correct class
    }

    #[wasm_bindgen_test]
    fn test_button_onclick() {
        // Test onclick callback is invoked
    }

    #[wasm_bindgen_test]
    fn test_button_disabled() {
        // Test disabled state
    }

    #[wasm_bindgen_test]
    fn test_button_accessibility() {
        // Test ARIA attributes
    }
}
```

## Build Process

### Library Build

```bash
# Development build
cargo build

# WASM build
wasm-pack build --target web

# Run tests
wasm-pack test --headless --chrome
```

### Showcase Build

```bash
# Development server with trunk
cd shadcn-showcase
trunk serve

# Production build
trunk build --release
```

## Dependencies

### Core Dependencies

```toml
[dependencies]
yew = { version = "0.21", features = ["csr"] }
web-sys = { version = "0.3", features = ["HtmlElement", "HtmlInputElement", ...] }
wasm-bindgen = "0.2"
gloo = "0.11"
chrono = { version = "0.4", features = ["wasmbind"] }

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

### Build Dependencies

```toml
[build-dependencies]
# None for library

# For showcase
trunk = "0.18" # Development server
```

## Performance Considerations

### Bundle Size Optimization

1. Use `wasm-opt` for WASM optimization
2. Enable LTO (Link Time Optimization)
3. Use `opt-level = 'z'` for size optimization
4. Tree-shake unused components
5. Lazy load heavy components (Calendar, Charts)

### Rendering Optimization

1. Use `PartialEq` for props to prevent unnecessary re-renders
2. Memoize expensive computations
3. Use `use_memo` for derived state
4. Avoid unnecessary clones

## Security Considerations

1. Sanitize user input in components that accept HTML
2. Use CSP (Content Security Policy) headers
3. Avoid `dangerously_set_inner_html` unless necessary
4. Validate all props at component boundaries
5. Escape user-provided content

## Extensibility

### Custom Variants

Users can extend variants by providing custom classes:

```rust
html! {
    <Button class="custom-variant">
        { "Custom styled button" }
    </Button>
}
```

### Component Composition

Components are designed to be composable:

```rust
html! {
    <Card>
        <CardHeader>
            <CardTitle>{ "Title" }</CardTitle>
            <CardDescription>{ "Description" }</CardDescription>
        </CardHeader>
        <CardContent>
            { "Content here" }
        </CardContent>
        <CardFooter>
            <Button>{ "Action" }</Button>
        </CardFooter>
    </Card>
}
```

## Migration Path from shadcn/ui

### API Similarities

React shadcn/ui:
```jsx
<Button variant="destructive" size="lg" onClick={handleClick}>
  Click me
</Button>
```

Rust shadcn-rs:
```rust
html! {
    <Button variant={Variant::Destructive} size={Size::Lg} onclick={handle_click}>
        { "Click me" }
    </Button>
}
```

### Key Differences

1. Props use Rust enums instead of strings for type safety
2. Event handlers use `Callback` instead of functions
3. Children use Yew's `Children` type
4. Classes use `Classes` type from Yew
