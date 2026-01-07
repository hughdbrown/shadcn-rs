# Theming Guide

shadcn-rs uses CSS custom properties (variables) for theming, making it easy to customize colors, spacing, and other design tokens.

## Theme Modes

shadcn-rs supports light and dark modes out of the box.

### Setting Theme Mode

Add the `data-theme` attribute to your root element:

```html
<!-- Light mode (default) -->
<html data-theme="light">

<!-- Dark mode -->
<html data-theme="dark">
```

### JavaScript Theme Toggle

```javascript
function toggleTheme() {
    const html = document.documentElement;
    const current = html.getAttribute('data-theme') || 'light';
    html.setAttribute('data-theme', current === 'light' ? 'dark' : 'light');
}
```

### Rust Theme Toggle

```rust
use web_sys::window;

fn toggle_theme() {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(root) = document.document_element() {
                let current = root.get_attribute("data-theme")
                    .unwrap_or_else(|| "light".to_string());
                let new_theme = if current == "light" { "dark" } else { "light" };
                let _ = root.set_attribute("data-theme", new_theme);
            }
        }
    }
}
```

## CSS Variables

### Color Variables

```css
:root {
    /* Primary colors */
    --primary: #0070f3;
    --primary-foreground: #ffffff;

    /* Secondary colors */
    --secondary: #f5f5f5;
    --secondary-foreground: #171717;

    /* Destructive colors */
    --destructive: #ef4444;
    --destructive-foreground: #ffffff;

    /* Background and foreground */
    --background: #ffffff;
    --foreground: #0a0a0a;

    /* Muted colors */
    --muted: #f5f5f5;
    --muted-foreground: #737373;

    /* Accent colors */
    --accent: #f5f5f5;
    --accent-foreground: #171717;

    /* Border and input */
    --border: #e5e5e5;
    --input: #e5e5e5;
    --ring: #0070f3;
}

[data-theme="dark"] {
    --primary: #3b82f6;
    --primary-foreground: #ffffff;

    --secondary: #262626;
    --secondary-foreground: #fafafa;

    --destructive: #dc2626;
    --destructive-foreground: #ffffff;

    --background: #0a0a0a;
    --foreground: #fafafa;

    --muted: #262626;
    --muted-foreground: #a3a3a3;

    --accent: #262626;
    --accent-foreground: #fafafa;

    --border: #262626;
    --input: #262626;
    --ring: #3b82f6;
}
```

### Spacing Variables

```css
:root {
    --spacing-xs: 0.25rem;   /* 4px */
    --spacing-sm: 0.5rem;    /* 8px */
    --spacing-md: 1rem;      /* 16px */
    --spacing-lg: 1.5rem;    /* 24px */
    --spacing-xl: 2rem;      /* 32px */
}
```

### Border Radius Variables

```css
:root {
    --radius-sm: 0.25rem;
    --radius-md: 0.375rem;
    --radius-lg: 0.5rem;
    --radius-xl: 0.75rem;
    --radius-full: 9999px;
}
```

## Customizing Colors

### Override in Your CSS

Create a custom stylesheet that overrides the default variables:

```css
/* custom-theme.css */
:root {
    --primary: #8b5cf6;  /* Purple */
    --primary-foreground: #ffffff;
}

[data-theme="dark"] {
    --primary: #a78bfa;
    --primary-foreground: #000000;
}
```

Include after the main stylesheet:

```html
<link rel="stylesheet" href="shadcn-rs.css">
<link rel="stylesheet" href="custom-theme.css">
```

### Brand Colors Example

```css
/* brand-theme.css */
:root {
    /* Brand primary - Blue */
    --primary: #1e40af;
    --primary-foreground: #ffffff;

    /* Brand secondary - Teal */
    --secondary: #0d9488;
    --secondary-foreground: #ffffff;

    /* Success - Green */
    --success: #16a34a;
    --success-foreground: #ffffff;

    /* Warning - Amber */
    --warning: #d97706;
    --warning-foreground: #000000;
}
```

## Component-Specific Styling

### Button Customization

```css
.btn {
    --btn-padding-x: 1rem;
    --btn-padding-y: 0.5rem;
    --btn-font-size: 0.875rem;
}

.btn-lg {
    --btn-padding-x: 1.5rem;
    --btn-padding-y: 0.75rem;
    --btn-font-size: 1rem;
}
```

### Input Customization

```css
.input {
    --input-height: 2.5rem;
    --input-padding: 0.75rem;
    --input-border-width: 1px;
}
```

## Typography

### Font Variables

```css
:root {
    --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    --font-mono: ui-monospace, SFMono-Regular, "SF Mono", Menlo, monospace;

    --font-size-xs: 0.75rem;
    --font-size-sm: 0.875rem;
    --font-size-base: 1rem;
    --font-size-lg: 1.125rem;
    --font-size-xl: 1.25rem;
    --font-size-2xl: 1.5rem;
}
```

### Custom Font Example

```css
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');

:root {
    --font-sans: 'Inter', sans-serif;
}
```

## Animation Variables

```css
:root {
    --transition-fast: 150ms;
    --transition-normal: 200ms;
    --transition-slow: 300ms;

    --ease-default: cubic-bezier(0.4, 0, 0.2, 1);
    --ease-in: cubic-bezier(0.4, 0, 1, 1);
    --ease-out: cubic-bezier(0, 0, 0.2, 1);
}
```

### Disable Animations

For users who prefer reduced motion:

```css
@media (prefers-reduced-motion: reduce) {
    * {
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
    }
}
```

## Z-Index Scale

```css
:root {
    --z-dropdown: 1000;
    --z-sticky: 1020;
    --z-fixed: 1030;
    --z-modal-backdrop: 1040;
    --z-modal: 1050;
    --z-popover: 1060;
    --z-tooltip: 1070;
}
```

## Best Practices

1. **Use CSS variables** - Always use variables instead of hardcoded values
2. **Test both themes** - Ensure your customizations work in light and dark modes
3. **Maintain contrast** - Keep text readable against backgrounds (4.5:1 ratio minimum)
4. **Document changes** - Keep a record of customized variables for team reference

## Next Steps

- [Accessibility Guide](./accessibility.md) - Ensure your themed components are accessible
- [Component Guide](./components.md) - See all available components
