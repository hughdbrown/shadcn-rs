# Development Guide

This guide covers setting up your development environment and workflows for contributing to shadcn-rs.

## Prerequisites

### Required Tools

1. **Rust** (2024 edition)
   ```bash
   # Install Rust via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Ensure you have the latest stable
   rustup update stable

   # Add WASM target
   rustup target add wasm32-unknown-unknown
   ```

2. **wasm-pack** (for building WASM packages)
   ```bash
   cargo install wasm-pack
   ```

3. **trunk** (for development server)
   ```bash
   cargo install --locked trunk
   ```

4. **wasm-bindgen-cli** (for testing)
   ```bash
   cargo install wasm-bindgen-cli
   ```

### Optional but Recommended

- **rust-analyzer** - IDE support
- **cargo-watch** - Auto-rebuild on file changes
  ```bash
  cargo install cargo-watch
  ```
- **cargo-expand** - Expand macros for debugging
  ```bash
  cargo install cargo-expand
  ```

## Project Structure

```
shadcn-rs/
├── shadcn-rs/          # Main component library
├── shadcn-icons/       # Icon library
├── shadcn-showcase/    # Demo application
└── ai/                 # Planning and documentation
```

## Development Workflow

### 1. Initial Setup

Clone and verify the workspace compiles:

```bash
cd shadcn-rs
cargo check --workspace
```

### 2. Running the Showcase Application

The showcase app is the best way to develop and test components interactively.

```bash
cd shadcn-showcase
trunk serve
```

Then open http://127.0.0.1:8080 in your browser.

Trunk will:
- Compile Rust to WASM
- Auto-reload on changes
- Serve the application
- Show compilation errors in the console

**Useful trunk options:**
```bash
# Serve with release optimizations (slower build, faster runtime)
trunk serve --release

# Serve on different port
trunk serve --port 8000

# Open browser automatically
trunk serve --open
```

### 3. Building the Library

#### Development Build

```bash
# Check all crates compile
cargo check --workspace

# Build all crates
cargo build --workspace

# Build specific crate
cargo build -p shadcn-rs
cargo build -p shadcn-icons
```

#### WASM Build

```bash
cd shadcn-rs

# Build for web
wasm-pack build --target web

# Build with optimizations
wasm-pack build --target web --release

# Output goes to pkg/ directory
```

### 4. Testing

#### Unit Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p shadcn-rs

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_button_renders
```

#### WASM Tests

```bash
# Test in headless Chrome
wasm-pack test --headless --chrome

# Test in headless Firefox
wasm-pack test --headless --firefox

# Test in browser (interactive)
wasm-pack test --chrome
```

#### Integration Tests

```bash
cd shadcn-rs
wasm-pack test --headless --chrome -- --test integration_tests
```

### 5. Code Quality

#### Formatting

```bash
# Check formatting
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all
```

#### Linting

```bash
# Run clippy (linter)
cargo clippy --workspace -- -D warnings

# Run clippy with all features
cargo clippy --workspace --all-features -- -D warnings

# Auto-fix some issues
cargo clippy --workspace --fix
```

#### Documentation

```bash
# Generate documentation
cargo doc --workspace --no-deps --open

# Check for broken doc links
cargo doc --workspace --no-deps
```

## Component Development Workflow

### Step 1: Design the API

Before writing code, design the component API:

```rust
// Example: Badge component API design
#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    #[prop_or_default]
    pub variant: Variant,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}
```

### Step 2: Write Documentation First

Write rustdoc comments before implementation:

```rust
/// A badge component for displaying status or labels.
///
/// # Examples
///
/// ```rust
/// html! {
///     <Badge variant={Variant::Primary}>
///         { "New" }
///     </Badge>
/// }
/// ```
#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    // Implementation
}
```

### Step 3: Implement the Component

Follow the pattern in `ai/05-component-patterns.md`:
- Create component file in `shadcn-rs/src/components/`
- Implement with proper props
- Add to `components/mod.rs`
- Export from `lib.rs`

### Step 4: Write Tests

Create tests in the same file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn renders_with_defaults() {
        // Test implementation
    }
}
```

### Step 5: Create Example

Create example in `examples/badge/`:

```bash
mkdir -p examples/badge
```

Add `Cargo.toml` and example code.

### Step 6: Add to Showcase

Update showcase app to demonstrate the component.

## Debugging

### Browser DevTools

When running the showcase:
1. Open browser DevTools (F12)
2. Check Console for Rust panics
3. Use Network tab to see WASM loading
4. Use Elements tab to inspect generated HTML

### Rust Debugging

```bash
# Enable debug logging
RUST_LOG=debug trunk serve

# Expand macros to see what they generate
cargo expand -p shadcn-rs

# Show full error backtraces
RUST_BACKTRACE=1 cargo test
```

### WASM Debugging

Add to your component:

```rust
use web_sys::console;

console::log_1(&format!("Debug: {:?}", value).into());
```

## Performance Optimization

### Build Size

```bash
# Check WASM size
ls -lh pkg/*.wasm

# Optimize with wasm-opt
wasm-opt -Oz pkg/shadcn_rs_bg.wasm -o pkg/optimized.wasm
```

### Build Time

```bash
# Use cargo-watch for incremental builds
cargo watch -x 'check --workspace'

# Parallel compilation (add to ~/.cargo/config.toml)
# [build]
# jobs = 8
```

## Common Tasks

### Add a New Component

```bash
# 1. Create component file
touch shadcn-rs/src/components/badge.rs

# 2. Edit components/mod.rs to add module
# pub mod badge;
# pub use badge::Badge;

# 3. Create tests
# (in same file with #[cfg(test)])

# 4. Create example
mkdir -p examples/badge
touch examples/badge/Cargo.toml
touch examples/badge/src/main.rs

# 5. Test it compiles
cargo check -p shadcn-rs
```

### Update Dependencies

```bash
# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Update to latest compatible versions
cargo upgrade  # requires cargo-edit
```

### Generate CSS

When modifying CSS:

```bash
# CSS is in shadcn-rs/styles/
# No build step needed - CSS is static

# To minify for production (optional):
# Use a CSS minifier or PostCSS
```

### Port Lucide Icons

```bash
cd shadcn-icons

# Icons are hand-coded for now
# Future: create script to auto-generate from Lucide SVGs

# Example structure:
#[function_component(IconName)]
pub fn icon_name(props: &IconProps) -> Html {
    html! {
        <svg ...>
            <path d="..." />
        </svg>
    }
}
```

## IDE Setup

### VS Code

Recommended extensions:
- rust-analyzer
- Even Better TOML
- crates

Settings (`.vscode/settings.json`):

```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "rust-lang.rust-analyzer"
}
```

### IntelliJ IDEA / CLion

1. Install Rust plugin
2. Enable "Run clippy on save"
3. Enable "Run rustfmt on save"

## CI/CD

The project uses GitHub Actions for:
- Testing on every PR
- Linting (clippy)
- Formatting checks
- WASM build verification
- Documentation generation

## Troubleshooting

### "Cannot find wasm-bindgen"

```bash
cargo install wasm-bindgen-cli
```

### "trunk not found"

```bash
cargo install --locked trunk
```

### WASM tests fail

```bash
# Make sure Chrome/Firefox is installed
# Try updating wasm-pack
cargo install wasm-pack --force
```

### CSS not loading

Check that:
1. CSS file path is correct in HTML
2. Trunk is serving from correct directory
3. Browser cache is cleared (Ctrl+Shift+R)

### Component not showing

1. Check browser console for errors
2. Verify component is exported from lib.rs
3. Check that CSS classes are defined
4. Inspect HTML in DevTools

## Best Practices

### Code Style

- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Use meaningful variable names
- Keep functions small and focused
- Add comments for complex logic

### Component Design

- Make components composable
- Support both controlled and uncontrolled patterns
- Include comprehensive prop validation
- Add ARIA attributes for accessibility
- Test keyboard navigation

### Testing

- Test all variants and sizes
- Test event handlers
- Test accessibility features
- Test edge cases (empty props, invalid data)

### Documentation

- Write rustdoc for all public items
- Include usage examples
- Document prop types and defaults
- Explain accessibility features

## Resources

- [Yew Documentation](https://yew.rs/docs/getting-started/introduction)
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [shadcn/ui Documentation](https://ui.shadcn.com/docs)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WASM Book](https://rustwasm.github.io/book/)

## Getting Help

- Check existing issues on GitHub
- Read planning documents in `ai/` directory
- Review component patterns in `ai/05-component-patterns.md`
- Ask questions in discussions

---

Happy coding! 🦀
