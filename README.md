# shadcn-rs

A broad, actively polished UI component library for Rust/WebAssembly inspired by [shadcn/ui](https://ui.shadcn.com).

[![Crates.io](https://img.shields.io/crates/v/shadcn-rs.svg)](https://crates.io/crates/shadcn-rs)
[![Documentation](https://docs.rs/shadcn-rs/badge.svg)](https://docs.rs/shadcn-rs)
[![License](https://img.shields.io/crates/l/shadcn-rs.svg)](LICENSE-MIT)

## Overview

shadcn-rs provides a full shadcn/ui-style component surface for building modern web applications with Rust and [Yew](https://yew.rs). The repo includes 66 component modules, a matching showcase application running on Yew 0.23, generated Lucide-style icons, and ongoing polish work for the most complex interactive widgets.

## Features

- **Broad Coverage** - 66 component modules with matching showcase pages
- **Type-Safe** - Rust enums and typed props for variants, sizes, and behavior
- **Accessible** - keyboard navigation, focus trapping for overlays, and ARIA support
- **Themeable** - light and dark mode via CSS variables
- **Interactive Examples** - showcase pages for complex widgets including chart, data table, carousel, calendar, and date picker
- **Generated Icons** - Lucide-style icons exposed as Yew components from checked-in generated code

## Quick Start

### Installation

Add shadcn-rs to your `Cargo.toml`:

```toml
[dependencies]
shadcn-rs = "0.1"
yew = "0.23"
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

### Advanced Forms (6)
Select, Combobox, Command, Input OTP, Date Picker, Calendar

### Complex (5)
Carousel, Data Table, Chart, Toast, Sonner

### Conversation & Utilities (7)
Attachment, Bubble, Marker, Message, Message Scroller, Questionnaire, Direction

**Total: 66 component modules**

## Complex Component Notes

- `DataTable` now uses explicit `DataTableColumn<T>` metadata for headers, sort/filter participation, and custom cell rendering.
- `Calendar` supports three selection encodings: single `YYYY-MM-DD`, multiple `YYYY-MM-DD,YYYY-MM-DD`, and range `YYYY-MM-DD..YYYY-MM-DD`.
- `DatePicker` uses the library `Calendar` popup instead of delegating to the browser's native date input UI.
- Overlay components such as `Dialog`, `AlertDialog`, `Sheet`, and `Drawer` now trap focus and restore it to the trigger on close.
- `shadcn-icons` is generated from `shadcn-icons/icons.txt` via `scripts/generate_icons.py`.

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

### Task Runner

Install `just` with `cargo install --locked just`, then run `just` to list the
recipes in the root `justfile`:

```bash
just build           # Build the workspace for WASM
just test            # Run native unit tests and doctests
just test-browser    # Run browser interaction tests
just serve           # Serve the showcase at http://127.0.0.1:8180
just ci              # Run formatting, checks, lint, tests, docs, and showcase build
```

Other recipes include `build-release`, `check`, `fmt`, `fmt-check`, `lint`,
`docs`, `showcase-build`, `showcase-preview`, `showcase-smoke`, `audit`,
`generate-icons`, and `clean`.
Browser tests require a matching `wasm-bindgen-cli`, Chrome, and ChromeDriver; showcase recipes require
Trunk. The audit and icon generation recipes require Python 3, and icon generation
downloads upstream SVGs and rewrites the generated Rust source.

### Build Library

```bash
cargo build -p shadcn-rs
```

### Run Showcase

```bash
cd shadcn-showcase
trunk serve
# Open http://127.0.0.1:8180
```

### Run Tests

```bash
cargo test --workspace
```

### Run Browser Interaction Tests

```bash
just test-browser
```

This runs every `browser_*` suite, including forms, overlays, composites, toasts,
conversation scrolling, and questionnaire navigation. Install Chrome and a
ChromeDriver matching its version. Put `chromedriver` on PATH, or set
`CHROMEDRIVER=/absolute/path/to/chromedriver` when running the recipe.

The test runner must match the `wasm-bindgen` version in `Cargo.lock` (currently
0.2.106):

```bash
cargo install --locked wasm-bindgen-cli --version 0.2.106
```

The test-only Yew feature enables `scheduler::flush()` so rendering assertions
wait for pending updates; timer tests still wait for actual timer durations.

### Production Sample App

```bash
just showcase-build
just showcase-preview
# Open http://127.0.0.1:8181
```

The home page provides a searchable catalog of all 66 component modules. Each
page has live examples and Rust snippets. Try the Message page to send local
messages, Attachment to change file states, and Questionnaire to complete a
multi-step form. Theme switching and mobile navigation are available throughout.

The optimized, static app is written to `shadcn-showcase/dist-release/`, separate
from Trunk's development output. Serve it with a fallback to `index.html` for
client-side routes. The build keeps `wasm-opt -Oz` enabled and explicitly enables
bulk-memory and nontrapping float-to-integer instructions emitted by Rust.

With the preview running, `just showcase-smoke` loads every component route and
exercises the catalog, conversation examples, theme switching, and mobile menu
in headless Chrome. It writes screenshots and `report.json` to
`target/showcase-smoke/`. It requires Python 3 and the same ChromeDriver setup.

### Yew 0.23 Compatibility

All workspace crates use Yew 0.23; the showcase uses yew-router 0.20. Consumers
must use compatible Yew types. Dynamic text passed to a component with `Children`
props may need a fragment, for example:

```rust,ignore
html! { <Button><>{ format!("Count: {count}") }</></Button> }
```

The existing `#[function_component]` attributes remain supported. Reducers in
Select, Command, and Toast were reviewed for 0.23's same-`Rc` rendering behavior.

### Build Documentation

```bash
cargo doc --workspace --no-deps --open
```

## Project Structure

```
shadcn-rs/
├── scripts/            # Checked-in maintenance and generation scripts
├── shadcn-rs/          # Main component library
├── shadcn-icons/       # Generated Lucide-style icons for Yew
├── shadcn-showcase/    # Interactive demo application
└── docs/               # User guides
```

## Icon Generation

The icon crate is generated from the manifest in `shadcn-icons/icons.txt`.

```bash
python3 scripts/generate_icons.py
```

This rewrites `shadcn-icons/src/generated.rs`, which is then re-exported by `shadcn-icons/src/lib.rs`.

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
