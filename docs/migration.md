# Migration Guide: React shadcn/ui to shadcn-rs

This guide helps developers familiar with the React shadcn/ui library transition to shadcn-rs.

## Key Differences

| Aspect | React shadcn/ui | shadcn-rs |
|--------|----------------|-----------|
| Language | TypeScript/JSX | Rust/Yew |
| Runtime | JavaScript | WebAssembly |
| Styling | Tailwind CSS | Static CSS with custom properties |
| State | React hooks | Yew hooks (`use_state`, `use_effect`) |
| Events | `onClick`, `onChange` | `onclick: Callback<MouseEvent>` |
| Children | `React.ReactNode` | `Children` or `Html` |
| Props | TypeScript interfaces | `#[derive(Properties)]` structs |

## Component Mapping

Most shadcn/ui components have a direct equivalent in shadcn-rs:

```tsx
// React shadcn/ui
<Button variant="destructive" size="sm" onClick={handleClick}>
  Delete
</Button>
```

```rust
// shadcn-rs
use shadcn_rs::{Button, Variant, Size};

html! {
    <Button
        variant={Variant::Destructive}
        size={Size::Sm}
        onclick={Callback::from(|_| { /* handle click */ })}
    >
        { "Delete" }
    </Button>
}
```

## Props Translation

### Variant Strings to Enums

React uses string literals; shadcn-rs uses typed enums:

| React | shadcn-rs |
|-------|-----------|
| `variant="default"` | `variant={Variant::Default}` |
| `variant="destructive"` | `variant={Variant::Destructive}` |
| `variant="outline"` | `variant={Variant::Outline}` |
| `variant="secondary"` | `variant={Variant::Secondary}` |
| `variant="ghost"` | `variant={Variant::Ghost}` |
| `variant="link"` | `variant={Variant::Link}` |
| `size="sm"` | `size={Size::Sm}` |
| `size="lg"` | `size={Size::Lg}` |

### Event Handlers

```tsx
// React
<Button onClick={(e) => console.log(e)}>Click</Button>
<Input onChange={(e) => setValue(e.target.value)} />
```

```rust
// shadcn-rs
html! {
    <Button onclick={Callback::from(|e: MouseEvent| log::info!("{:?}", e))}>
        { "Click" }
    </Button>
}

// For input changes, use oninput (not onchange)
let oninput = Callback::from(|e: InputEvent| {
    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
    // use input.value()
});
html! { <Input {oninput} /> }
```

### Boolean Props

```tsx
// React - boolean shorthand
<Input disabled />
<Button loading />
```

```rust
// shadcn-rs - explicit boolean values
html! {
    <Input disabled={true} />
    // or just:
    <Input disabled=true />
}
```

### Optional Props

```tsx
// React - optional props are undefined by default
<Dialog onOpenChange={setOpen} />
```

```rust
// shadcn-rs - use Option<T> with #[prop_or_default]
html! {
    <Dialog on_open_change={Callback::from(|open: bool| { /* ... */ })} />
}
```

## State Management

### useState to use_state

```tsx
// React
const [open, setOpen] = useState(false);
```

```rust
// Yew
let open = use_state(|| false);
let set_open = {
    let open = open.clone();
    Callback::from(move |val: bool| open.set(val))
};
```

### Controlled Components

```tsx
// React
const [value, setValue] = useState("");
<Select value={value} onValueChange={setValue}>
```

```rust
// shadcn-rs
let value = use_state(|| AttrValue::from(""));
let on_select = {
    let value = value.clone();
    Callback::from(move |(val, _label): (AttrValue, AttrValue)| {
        value.set(val);
    })
};
html! {
    <Select value={(*value).clone()} on_select={on_select} />
}
```

## Styling Differences

### Tailwind to CSS Custom Properties

React shadcn/ui uses Tailwind utility classes. shadcn-rs uses static CSS with custom properties for theming:

```css
/* shadcn-rs theming via CSS custom properties */
:root {
    --background: #ffffff;
    --foreground: #0a0a0a;
    --primary: #171717;
    --primary-foreground: #fafafa;
    --radius: 0.5rem;
}

[data-theme="dark"] {
    --background: #0a0a0a;
    --foreground: #fafafa;
    --primary: #fafafa;
    --primary-foreground: #171717;
}
```

### Custom Classes

Use the `class` prop instead of `className`:

```tsx
// React
<Button className="my-custom-class">Click</Button>
```

```rust
// shadcn-rs
html! {
    <Button class={classes!("my-custom-class")}>{ "Click" }</Button>
}
```

## Component Name Changes

Most component names are identical, but follow Rust naming conventions:

| React | shadcn-rs |
|-------|-----------|
| `AlertDialog` | `AlertDialog` |
| `DropdownMenu` | `DropdownMenu` |
| `NavigationMenu` | `NavigationMenu` |
| `InputOTP` | `InputOTP` |
| `HoverCard` | `HoverCard` |

Sub-components use the same compound naming:

| React | shadcn-rs |
|-------|-----------|
| `Card.Header` or `CardHeader` | `CardHeader` |
| `Select.Trigger` or `SelectTrigger` | `SelectTrigger` |
| `Dialog.Content` or `DialogContent` | `DialogContent` |

## What's Not Available

Some React-specific features don't apply in the WASM context:

- **Server Components** - Yew runs entirely client-side
- **Next.js integration** - Use Trunk for bundling instead
- **Radix UI primitives** - shadcn-rs implements accessibility directly
- **CSS-in-JS** - Static CSS is used instead
- **Framer Motion animations** - Use CSS animations/transitions
