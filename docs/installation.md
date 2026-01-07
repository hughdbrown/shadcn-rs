# Installation Guide

This guide covers how to install and set up shadcn-rs in your Rust/WebAssembly project.

## Prerequisites

- Rust 2024 edition (rustc 1.85+)
- A Yew-based WebAssembly project
- [Trunk](https://trunkrs.dev/) for development server (recommended)

## Adding Dependencies

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

## Including Styles

### Option 1: CDN (Recommended for Quick Start)

Add to your `index.html`:

```html
<link rel="stylesheet" href="https://unpkg.com/shadcn-rs@0.1/shadcn-rs.css">
```

### Option 2: Local CSS

1. Download `shadcn-rs.css` from the release
2. Add to your project's assets
3. Include in your `index.html`:

```html
<link rel="stylesheet" href="/assets/shadcn-rs.css">
```

### Option 3: Trunk Configuration

If using Trunk, add to your `Trunk.toml`:

```toml
[[hooks]]
stage = "build"
command = "sh"
command_arguments = ["-c", "cp node_modules/shadcn-rs/shadcn-rs.css dist/"]
```

## Basic Usage

```rust
use yew::prelude::*;
use shadcn_rs::{Button, Variant};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Button variant={Variant::Primary}>
            { "Click me" }
        </Button>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

## Project Structure

A typical shadcn-rs project structure:

```
my-app/
├── Cargo.toml
├── Trunk.toml
├── index.html
└── src/
    └── main.rs
```

### Example `index.html`

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>My App</title>
    <link data-trunk rel="css" href="shadcn-rs.css" />
</head>
<body></body>
</html>
```

### Example `Trunk.toml`

```toml
[build]
target = "index.html"
dist = "dist"

[watch]
watch = ["src", "index.html"]
```

## Development Server

Start the development server with Trunk:

```bash
trunk serve
```

Open `http://127.0.0.1:8080` in your browser.

## Building for Production

```bash
trunk build --release
```

The output will be in the `dist/` directory.

## Troubleshooting

### WASM Target Not Installed

If you get errors about the wasm32 target:

```bash
rustup target add wasm32-unknown-unknown
```

### Trunk Not Found

Install Trunk:

```bash
cargo install trunk
```

### CSS Not Loading

Ensure the CSS file path is correct and the file is being served. Check browser developer tools for 404 errors.

## Next Steps

- [Theming Guide](./theming.md) - Customize colors and styles
- [Accessibility Guide](./accessibility.md) - Build accessible applications
- [Component Guide](./components.md) - Explore all components
