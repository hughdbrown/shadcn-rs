# Contributing to shadcn-rs

Thank you for your interest in contributing to shadcn-rs! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to:
- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

## How to Contribute

### Reporting Bugs

Before creating a bug report:
1. Check existing issues to avoid duplicates
2. Verify the bug exists in the latest version
3. Collect information about your environment

When reporting a bug, include:
- Clear, descriptive title
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, browser)
- Minimal reproducible example
- Screenshots/videos if applicable

### Suggesting Features

Before suggesting a feature:
1. Check if it exists in the roadmap (see `ai/02-implementation-tasks.md`)
2. Review existing feature requests
3. Consider if it fits the project scope

When suggesting a feature:
- Explain the use case
- Describe the proposed solution
- Consider alternatives
- Note if you're willing to implement it

### Pull Requests

#### Before Starting

1. **Check the roadmap** - Avoid duplicate work
2. **Open an issue** - Discuss major changes first
3. **Read planning docs** - Review `ai/` directory
4. **Follow patterns** - Use `ai/05-component-patterns.md`

#### Process

1. **Fork the repository**
   ```bash
   git clone https://github.com/YOUR_USERNAME/shadcn-rs
   cd shadcn-rs
   ```

2. **Create a branch**
   ```bash
   git checkout -b feature/component-name
   # or
   git checkout -b fix/issue-description
   ```

3. **Make your changes**
   - Follow the component development workflow in `DEVELOPMENT.md`
   - Write tests for new functionality
   - Update documentation
   - Follow coding standards

4. **Test thoroughly**
   ```bash
   # Run all checks
   cargo check --workspace
   cargo test --workspace
   cargo clippy --workspace -- -D warnings
   cargo fmt --all -- --check

   # Test WASM
   wasm-pack test --headless --chrome
   ```

5. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: add Badge component

   - Implement Badge with all variants
   - Add comprehensive tests
   - Create example
   - Update documentation"
   ```

   Use conventional commits:
   - `feat:` - New feature
   - `fix:` - Bug fix
   - `docs:` - Documentation
   - `style:` - Code style (formatting)
   - `refactor:` - Code refactoring
   - `test:` - Adding tests
   - `chore:` - Maintenance

6. **Push and create PR**
   ```bash
   git push origin feature/component-name
   ```

   Then open a PR on GitHub with:
   - Clear description of changes
   - Link to related issue
   - Screenshots/videos of UI changes
   - Test results

#### PR Requirements

Your PR must:
- ✅ Pass all CI checks
- ✅ Include tests for new functionality
- ✅ Update relevant documentation
- ✅ Follow code style guidelines
- ✅ Have no clippy warnings
- ✅ Be properly formatted (`cargo fmt`)
- ✅ Include example if adding a component

## Component Implementation Guidelines

### Implementation Order

Follow the tier system in `ai/01-project-scope.prd`:
1. Tier 1 - Foundational (simple components)
2. Tier 2 - Form Components
3. ... and so on

**Note**: Tier 1 components are implemented sequentially to establish patterns. After Tier 1, work can be parallelized.

### Component Checklist

When implementing a component, ensure:

- [ ] **API Design**
  - Props use appropriate enums (Size, Variant, Color)
  - Supports controlled/uncontrolled pattern (if applicable)
  - Accepts standard props (class, style, aria-label)
  - Children support where appropriate

- [ ] **Implementation**
  - Follows pattern in `ai/05-component-patterns.md`
  - Uses CSS classes from utility system
  - Implements all variants and sizes
  - Handles edge cases gracefully

- [ ] **Accessibility**
  - Proper ARIA attributes
  - Keyboard navigation support
  - Focus management
  - Screen reader friendly
  - Semantic HTML

- [ ] **Documentation**
  - Comprehensive rustdoc comments
  - Usage examples
  - Props documented
  - Accessibility features noted

- [ ] **Tests**
  - Default rendering
  - All variants and sizes
  - Event handlers
  - Accessibility attributes
  - Keyboard navigation
  - Edge cases

- [ ] **Example**
  - Created in `examples/component-name/`
  - Demonstrates all features
  - Shows common patterns

- [ ] **CSS**
  - Component styles added to appropriate CSS file
  - Follows naming convention (BEM-style)
  - Uses CSS variables
  - Dark mode support

- [ ] **Showcase**
  - Added to showcase app (after all components complete)

## Code Style Guidelines

### Rust Code

```rust
// Good: Clear, descriptive names
pub struct ButtonProps {
    pub variant: Variant,
    pub size: Size,
    pub disabled: bool,
}

// Bad: Unclear abbreviations
pub struct BtnProps {
    pub var: Variant,
    pub sz: Size,
    pub dis: bool,
}
```

### Documentation

```rust
/// A button component for triggering actions.
///
/// Buttons communicate actions that users can take.
///
/// # Accessibility
///
/// - Uses semantic `<button>` element
/// - Supports keyboard activation (Enter, Space)
/// - Includes `aria-disabled` when disabled
///
/// # Examples
///
/// ```rust
/// use shadcn_rs::{Button, Variant};
///
/// html! {
///     <Button variant={Variant::Primary}>
///         { "Click me" }
///     </Button>
/// }
/// ```
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    // Implementation
}
```

### CSS

```css
/* Good: Clear, BEM-style naming */
.btn {
  /* Base styles */
}

.btn--loading {
  /* Loading modifier */
}

.btn.variant-primary {
  /* Variant class */
}

/* Bad: Unclear, inconsistent naming */
.b {
  /* What is this? */
}

.button-loading-state {
  /* Inconsistent with naming pattern */
}
```

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn renders_with_defaults() {
        // Test default rendering
    }

    #[wasm_bindgen_test]
    fn handles_all_variants() {
        // Test each variant
    }

    #[wasm_bindgen_test]
    fn handles_click_events() {
        // Test event handling
    }
}
```

### Test Coverage

Aim for:
- 100% coverage of component logic
- All variants and sizes tested
- All event handlers tested
- Edge cases covered
- Accessibility features verified

## Documentation Standards

### Component Documentation

Every component needs:
1. **Overview** - What it does
2. **Usage Examples** - Basic and advanced
3. **Props** - All props documented
4. **Accessibility** - ARIA attributes, keyboard support
5. **Variants** - All variants explained

### Code Comments

```rust
// Good: Explain WHY, not WHAT
// Focus trap needed to prevent keyboard navigation outside modal
let focus_trap = use_focus_trap(is_open);

// Bad: Explain obvious code
// Set x to 5
let x = 5;
```

## Review Process

### What Reviewers Look For

1. **Code Quality**
   - Follows patterns
   - Clean, readable
   - No clippy warnings
   - Properly formatted

2. **Functionality**
   - Works as intended
   - No regressions
   - Edge cases handled

3. **Tests**
   - Comprehensive coverage
   - Tests pass
   - Good test names

4. **Documentation**
   - Complete
   - Accurate
   - Clear examples

5. **Accessibility**
   - ARIA attributes
   - Keyboard navigation
   - Screen reader support

### Addressing Review Comments

- Respond to all comments
- Make requested changes
- Ask for clarification if needed
- Mark conversations as resolved
- Re-request review when ready

## Component Implementation Priority

Current focus: **Tier 1 - Foundational Components**

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

Check `ai/02-implementation-tasks.md` for detailed task breakdown.

## Questions?

- Open a discussion on GitHub
- Review planning docs in `ai/`
- Check `DEVELOPMENT.md` for setup help
- Ask in your PR if it's related to your contribution

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).

---

Thank you for contributing to shadcn-rs! 🎉
