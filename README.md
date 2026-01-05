# shadcn-rs

A comprehensive UI component library for Rust/WebAssembly inspired by [shadcn/ui](https://ui.shadcn.com).

> **Status**: 🚧 Project setup complete. Component implementation in progress.

## Overview

shadcn-rs provides 59+ accessible, customizable UI components for building modern web applications with Rust and [Yew](https://yew.rs). All components are built with WCAG 2.1 AA compliance, full keyboard navigation support, and mobile touch gestures.

## Features

- ✅ **59+ Components** - Complete implementation of shadcn/ui components
- ✅ **Type-Safe** - Rust enums for variants, sizes, and colors
- ✅ **Accessible** - WCAG 2.1 AA compliant with ARIA attributes
- ✅ **Themeable** - Light and dark mode with CSS variables
- ✅ **Mobile-Friendly** - Touch gesture support for key components
- ✅ **No Runtime Overhead** - Pure CSS animations and styling
- ✅ **Icon Library** - Complete Lucide icon set as Yew components

## Quick Start

### Installation

Add shadcn-rs to your `Cargo.toml`:

```toml
[dependencies]
shadcn-rs = "0.1.0"
shadcn-icons = "0.1.0"  # Optional: icon library
yew = "0.21"
```

### Include CSS

Add the stylesheet to your HTML:

```html
<link rel="stylesheet" href="shadcn-rs.css">
```

### Usage Example

```rust
use yew::prelude::*;
use shadcn_rs::{Button, Variant, Size};
use shadcn_icons::Check;

#[function_component(App)]
fn app() -> Html {
    let onclick = Callback::from(|_| {
        web_sys::console::log_1(&"Button clicked!".into());
    });

    html! {
        <Button
            variant={Variant::Primary}
            size={Size::Lg}
            onclick={onclick}
            icon={html! { <Check /> }}
        >
            { "Click me" }
        </Button>
    }
}
```

## Project Structure

This repository is organized as a Cargo workspace with three crates:

- **shadcn-rs** - Main component library
- **shadcn-icons** - Lucide icons for Rust/Yew
- **shadcn-showcase** - Interactive component showcase

## Components

### Tier 1 - Foundational (10 components)
Badge, Button, Label, Separator, Skeleton, Spinner, Kbd, Typography, Avatar, Alert

### Tier 2 - Form Components (8 components)
Input, Textarea, Checkbox, Switch, Radio Group, Native Select, Slider, Progress

### Tier 3 - Layout & Structure (8 components)
Card, Aspect Ratio, Scroll Area, Resizable, Tabs, Table, Empty, Item

### Tier 4 - Interactive (7 components)
Button Group, Input Group, Field, Collapsible, Accordion, Toggle, Toggle Group

### Tier 5 - Overlays & Popups (7 components)
Dialog, Alert Dialog, Popover, Tooltip, Hover Card, Sheet, Drawer

### Tier 6 - Navigation (7 components)
Breadcrumb, Navigation Menu, Menubar, Dropdown Menu, Context Menu, Pagination, Sidebar

### Tier 7 - Advanced Forms (7 components)
Form, Select, Combobox, Command, Input OTP, Date Picker, Calendar

### Tier 8 - Complex (5 components)
Carousel, Data Table, Chart, Toast, Sonner

**Total**: 59 components

## Development

### Prerequisites

- Rust 2024 edition
- wasm-pack
- trunk (for development server)

### Build Library

```bash
# Build library crate
cd shadcn-rs
cargo build

# Build for WASM
wasm-pack build --target web
```

### Run Showcase

```bash
# Run development server
cd shadcn-showcase
trunk serve

# Open http://127.0.0.1:8080
```

### Run Tests

```bash
# Run all tests
cargo test

# Run WASM tests
wasm-pack test --headless --chrome
```

## Documentation

- [Component Documentation](https://docs.rs/shadcn-rs)
- [Icon Library](https://docs.rs/shadcn-icons)
- [Examples](./examples)
- [Architecture](./ai/03-architecture.md)
- [Implementation Guide](./ai/05-component-patterns.md)

## Browser Support

- Chrome/Edge (last 2 versions)
- Firefox (last 2 versions)
- Safari (last 2 versions)
- Mobile Safari (iOS 14+)
- Chrome Mobile (last 2 versions)

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

This project is dual-licensed under MIT or Apache-2.0.

## Acknowledgments

- Inspired by [shadcn/ui](https://ui.shadcn.com) by [@shadcn](https://twitter.com/shadcn)
- Icons from [Lucide](https://lucide.dev)
- Built with [Yew](https://yew.rs)

---

**Version**: 0.1.0 (Pre-release)
**Status**: Active Development
**Estimated v1.0 Release**: TBD
