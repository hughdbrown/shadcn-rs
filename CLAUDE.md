# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

shadcn-rs is a Rust/WebAssembly UI component library (59+ components) inspired by shadcn/ui, built with Yew 0.21. Uses Rust 2024 edition (1.85+).

## Build Commands

```bash
# Check/Build
cargo check --workspace
cargo build -p shadcn-rs

# Run Showcase App
cd shadcn-showcase && trunk serve

# Tests (325+ unit tests, 84 doctests)
cargo test --workspace

# Linting (warnings as errors)
cargo clippy --workspace -- -D warnings
cargo fmt --all

# Documentation
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

## Workspace Structure

```
shadcn-rs/           # Main component library (cdylib + rlib)
  src/components/    # 60 component files organized by tier
  src/hooks/         # use_toggle, use_click_outside, use_escape_key, etc.
  src/types/         # Variant, Size, Color, Position, Alignment enums
  src/utils/         # class_names, ARIA helpers, Portal, touch detection
  styles/            # Pre-generated CSS (base, variables, components, utilities)

shadcn-icons/        # Lucide icons as Yew components
shadcn-showcase/     # Interactive demo app (WASM, uses Trunk)
docs/                # User guides (installation, theming, accessibility)
ai/                  # Planning docs and architecture decisions
```

## Component Architecture

Components are organized into 8 tiers by complexity:
- Tier 1-2: Foundational (Button, Badge, Input, Checkbox, etc.)
- Tier 3-4: Layout & Interactive (Card, Tabs, Accordion, Collapsible)
- Tier 5-6: Overlays & Navigation (Dialog, Popover, Dropdown, Sidebar)
- Tier 7-8: Advanced Forms & Complex (Calendar, DataTable, Chart, Toast)

### Component Pattern

```rust
#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or(Variant::Default)] pub variant: Variant,
    #[prop_or(Size::Md)] pub size: Size,
    #[prop_or_default] pub disabled: bool,
    #[prop_or_default] pub class: Classes,
    #[prop_or_default] pub children: Children,
    // Event handlers as Callback<T>
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let classes = classes!(
        "btn",
        props.variant.button_class(),
        props.size.class(),
        props.class.clone()
    );
    html! { <button class={classes} disabled={props.disabled}>{ props.children.clone() }</button> }
}
```

### Key Types

- `Variant`: Default, Primary, Secondary, Destructive, Outline, Ghost, Link
- `Size`: Xs, Sm, Md, Lg, Xl
- `Color`, `Position`, `Alignment` - similar enum patterns with `.class()` methods

## Styling Approach

- Static CSS shipped with library (no runtime generation)
- CSS custom properties for theming
- Light/dark mode via `data-theme` attribute
- Users include: `<link rel="stylesheet" href="shadcn-rs.css">`

## Key Utilities

```rust
use shadcn_rs::utils::{class_names, merge_classes, generate_id};
use shadcn_rs::utils::{aria_labelledby, aria_describedby};  // ARIA helpers
use shadcn_rs::Portal;  // Portal rendering for overlays

// Hooks
use shadcn_rs::hooks::{use_toggle, use_click_outside, use_escape_key};
use shadcn_rs::hooks::{use_controllable_state, use_controllable_bool};
```

## Testing

Tests use `wasm-bindgen-test` and run in browser context:

```rust
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_component_renders() {
        // Test implementation
    }
}
```

## CI Requirements

All PRs must pass:
1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace -- -D warnings`
3. `cargo test --workspace`
4. `RUSTDOCFLAGS="-D warnings" cargo doc`
5. WASM build via trunk
