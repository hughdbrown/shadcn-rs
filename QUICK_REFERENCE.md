# Quick Reference

Fast access to common commands and workflows.

## Setup

```bash
# Install prerequisites
rustup target add wasm32-unknown-unknown
cargo install wasm-pack trunk

# Clone and verify
git clone <repo>
cd shadcn-rs
cargo check --workspace
```

## Development

```bash
# Run showcase (most common during development)
cd shadcn-showcase
trunk serve
# Open http://127.0.0.1:8080

# Auto-rebuild on changes
cargo watch -x 'check --workspace'
```

## Building

```bash
# Check compilation
cargo check --workspace

# Build all
cargo build --workspace

# Build WASM
cd shadcn-rs
wasm-pack build --target web --release
```

## Testing

```bash
# All tests
cargo test --workspace

# WASM tests
wasm-pack test --headless --chrome

# Specific test
cargo test test_name -- --nocapture
```

## Code Quality

```bash
# Format
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings

# Fix lints
cargo clippy --workspace --fix

# Generate docs
cargo doc --workspace --no-deps --open
```

## Component Development

```bash
# 1. Create file
touch shadcn-rs/src/components/badge.rs

# 2. Add to mod.rs
# pub mod badge;
# pub use badge::Badge;

# 3. Implement following ai/05-component-patterns.md

# 4. Test
cargo test -p shadcn-rs

# 5. Create example
mkdir -p examples/badge
```

## Git Workflow

```bash
# Create branch
git checkout -b feat/component-name

# Commit
git add .
git commit -m "feat: add Component

- Implementation
- Tests
- Example"

# Push
git push origin feat/component-name
```

## Debugging

```bash
# Show backtraces
RUST_BACKTRACE=1 cargo test

# Expand macros
cargo expand -p shadcn-rs

# Debug logs
RUST_LOG=debug trunk serve
```

## Files to Edit

| Task | Files |
|------|-------|
| New component | `shadcn-rs/src/components/NAME.rs` |
| Export component | `shadcn-rs/src/components/mod.rs` |
| Add CSS | `shadcn-rs/styles/components.css` |
| New type | `shadcn-rs/src/types/NAME.rs` |
| New hook | `shadcn-rs/src/hooks/NAME.rs` |
| New icon | `shadcn-icons/src/lib.rs` |
| Showcase | `shadcn-showcase/src/main.rs` |

## Component Checklist

- [ ] Props with `#[derive(Properties, PartialEq)]`
- [ ] CSS classes using utilities
- [ ] ARIA attributes
- [ ] Keyboard navigation
- [ ] Tests (defaults, variants, events, a11y)
- [ ] Rustdoc with examples
- [ ] Example in `examples/`

## Current Priority

**Tier 1 Components** (implement in order):
1. Badge
2. Button
3. Label
4. Separator
5. Skeleton
6. Spinner
7. Kbd
8. Typography
9. Avatar
10. Alert

## Useful Links

- Planning: `ai/` directory
- Patterns: `ai/05-component-patterns.md`
- Tasks: `ai/02-implementation-tasks.md`
- Full guide: `DEVELOPMENT.md`

## Common Issues

**"Cannot find wasm-bindgen"**
```bash
cargo install wasm-bindgen-cli
```

**CSS not loading**
- Clear browser cache (Ctrl+Shift+R)
- Check console for errors
- Verify CSS file path

**Component not rendering**
- Check it's exported in `lib.rs`
- Verify CSS classes exist
- Inspect HTML in DevTools
