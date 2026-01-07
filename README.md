# shadcn-rs

A comprehensive UI component library for Rust/WebAssembly inspired by [shadcn/ui](https://ui.shadcn.com).

[![Crates.io](https://img.shields.io/crates/v/shadcn-rs.svg)](https://crates.io/crates/shadcn-rs)
[![Documentation](https://docs.rs/shadcn-rs/badge.svg)](https://docs.rs/shadcn-rs)
[![License](https://img.shields.io/crates/l/shadcn-rs.svg)](LICENSE-MIT)

## Overview

shadcn-rs provides 59+ accessible, customizable UI components for building modern web applications with Rust and [Yew](https://yew.rs). All components are built with WCAG 2.1 AA compliance, full keyboard navigation support, and mobile touch gestures.

## Features

- **59+ Components** - Complete implementation of shadcn/ui components
- **Type-Safe** - Rust enums for variants, sizes, and colors
- **Accessible** - WCAG 2.1 AA compliant with ARIA attributes
- **Themeable** - Light and dark mode with CSS variables
- **Mobile-Friendly** - Touch gesture support for key components
- **No Runtime Overhead** - Pure CSS animations and styling
- **Icon Library** - Lucide icons as Yew components

## Quick Start

### Installation

Add shadcn-rs to your `Cargo.toml`:

```toml
[dependencies]
shadcn-rs = "0.1"
yew = "0.21"
```

For icons (optional):

```toml
[dependencies]
shadcn-icons = "0.1"
```

### Include CSS

Add the stylesheet to your `index.html`:

```html
<link rel="stylesheet" href="shadcn-rs.css">
```

### Usage Example

```rust
use yew::prelude::*;
use shadcn_rs::{Button, Card, CardHeader, CardTitle, CardContent, Variant};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Card>
            <CardHeader>
                <CardTitle>{ "Welcome" }</CardTitle>
            </CardHeader>
            <CardContent>
                <p>{ "Get started with shadcn-rs" }</p>
                <Button variant={Variant::Primary}>
                    { "Learn More" }
                </Button>
            </CardContent>
        </Card>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

## Components

### Foundational (10)
Badge, Button, Label, Separator, Skeleton, Spinner, Kbd, Typography, Avatar, Alert

### Form Components (9)
Input, Textarea, Checkbox, Switch, Radio Group, Native Select, Slider, Progress, Form

### Layout & Structure (8)
Card, Aspect Ratio, Scroll Area, Resizable, Tabs, Table, Empty, Item

### Interactive (7)
Button Group, Input Group, Field, Collapsible, Accordion, Toggle, Toggle Group

### Overlays & Popups (7)
Dialog, Alert Dialog, Popover, Tooltip, Hover Card, Sheet, Drawer

### Navigation (7)
Breadcrumb, Navigation Menu, Menubar, Dropdown Menu, Context Menu, Pagination, Sidebar

### Advanced Forms (7)
Select, Combobox, Command, Input OTP, Date Picker, Calendar

### Complex (5)
Carousel, Data Table, Chart, Toast, Sonner

**Total: 59+ components**

## Documentation

- [Installation Guide](./docs/installation.md)
- [Theming Guide](./docs/theming.md)
- [Accessibility Guide](./docs/accessibility.md)
- [API Documentation](https://docs.rs/shadcn-rs)
- [Live Showcase](https://hughdbrown.github.io/shadcn-rs/)

## Development

### Prerequisites

- Rust 2024 edition (rustc 1.85+)
- wasm32-unknown-unknown target
- [Trunk](https://trunkrs.dev/) for development server

### Build Library

```bash
cargo build -p shadcn-rs
```

### Run Showcase

```bash
cd shadcn-showcase
trunk serve
# Open http://127.0.0.1:8080
```

### Run Tests

```bash
cargo test --workspace
```

### Build Documentation

```bash
cargo doc --workspace --no-deps --open
```

## Project Structure

```
shadcn-rs/
├── shadcn-rs/          # Main component library
├── shadcn-icons/       # Lucide icons for Yew
├── shadcn-showcase/    # Interactive demo application
└── docs/               # User guides
```

## Browser Support

- Chrome/Edge (last 2 versions)
- Firefox (last 2 versions)
- Safari (last 2 versions)
- Mobile Safari (iOS 14+)
- Chrome Mobile (last 2 versions)

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

### Development Setup

1. Clone the repository
2. Install Rust and the WASM target: `rustup target add wasm32-unknown-unknown`
3. Install Trunk: `cargo install trunk`
4. Run the showcase: `cd shadcn-showcase && trunk serve`

## License

This project is dual-licensed under [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE).

## Acknowledgments

- Inspired by [shadcn/ui](https://ui.shadcn.com) by [@shadcn](https://twitter.com/shadcn)
- Icons from [Lucide](https://lucide.dev)
- Built with [Yew](https://yew.rs)
