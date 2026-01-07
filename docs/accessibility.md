# Accessibility Guide

shadcn-rs is built with accessibility as a core principle. All components follow WCAG 2.1 AA guidelines and implement proper ARIA attributes, keyboard navigation, and screen reader support.

## Accessibility Features

### ARIA Attributes

All interactive components include appropriate ARIA attributes:

- **Buttons**: `aria-pressed`, `aria-expanded`, `aria-disabled`
- **Dialogs**: `role="dialog"`, `aria-modal`, `aria-labelledby`, `aria-describedby`
- **Forms**: `aria-invalid`, `aria-required`, `aria-describedby`
- **Menus**: `role="menu"`, `role="menuitem"`, `aria-haspopup`
- **Tabs**: `role="tablist"`, `role="tab"`, `role="tabpanel"`, `aria-selected`
- **Progress**: `role="progressbar"`, `aria-valuenow`, `aria-valuemin`, `aria-valuemax`

### Keyboard Navigation

All components support full keyboard navigation:

| Component | Keys |
|-----------|------|
| Button | `Enter`, `Space` to activate |
| Dialog | `Escape` to close, `Tab` to cycle focus |
| Dropdown | `Arrow Up/Down` to navigate, `Enter` to select, `Escape` to close |
| Tabs | `Arrow Left/Right` to switch tabs |
| Accordion | `Arrow Up/Down` to navigate, `Enter/Space` to expand |
| Slider | `Arrow Left/Right` to adjust value |
| Switch | `Space` to toggle |
| Checkbox | `Space` to toggle |

### Focus Management

- **Focus Trap**: Dialogs, sheets, and drawers trap focus within the overlay
- **Focus Restoration**: Focus returns to the trigger element when overlays close
- **Visible Focus**: All focusable elements have visible focus indicators

## Component Examples

### Accessible Button

```rust
use shadcn_rs::{Button, Variant};

html! {
    <Button
        variant={Variant::Primary}
        aria_label={Some("Submit form".into())}
    >
        { "Submit" }
    </Button>
}
```

### Accessible Form Field

```rust
use shadcn_rs::{Field, Input};

html! {
    <Field
        label="Email address"
        required={true}
        error={Some("Please enter a valid email".into())}
    >
        <Input
            r#type="email"
            placeholder="you@example.com"
            invalid={true}
            aria_describedby={Some("email-error".into())}
        />
    </Field>
}
```

### Accessible Dialog

```rust
use shadcn_rs::{
    Dialog, DialogContent, DialogHeader, DialogTitle,
    DialogDescription, DialogFooter, Button
};

html! {
    <Dialog open={true}>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>{ "Confirm Action" }</DialogTitle>
                <DialogDescription>
                    { "Are you sure you want to proceed?" }
                </DialogDescription>
            </DialogHeader>
            <DialogFooter>
                <Button variant={Variant::Ghost}>{ "Cancel" }</Button>
                <Button variant={Variant::Primary}>{ "Confirm" }</Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
}
```

### Accessible Tabs

```rust
use shadcn_rs::{Tabs, TabsList, TabsTrigger, TabsContent};

html! {
    <Tabs default_value="tab1">
        <TabsList aria_label={Some("Settings sections".into())}>
            <TabsTrigger value="tab1">{ "Account" }</TabsTrigger>
            <TabsTrigger value="tab2">{ "Security" }</TabsTrigger>
        </TabsList>
        <TabsContent value="tab1">
            { "Account settings content" }
        </TabsContent>
        <TabsContent value="tab2">
            { "Security settings content" }
        </TabsContent>
    </Tabs>
}
```

## Best Practices

### 1. Always Provide Labels

```rust
// Good - has aria-label
<Button aria_label={Some("Close dialog".into())}>
    <XIcon />
</Button>

// Good - has visible label
<Button>{ "Close" }</Button>

// Bad - no accessible name
<Button>
    <XIcon />
</Button>
```

### 2. Use Semantic HTML

shadcn-rs components use appropriate HTML elements:

- `<button>` for buttons (not `<div>`)
- `<input>` for form inputs
- `<nav>` for navigation
- `<dialog>` semantics for modals

### 3. Provide Error Messages

```rust
<Field
    label="Password"
    error={if password_invalid {
        Some("Password must be at least 8 characters".into())
    } else {
        None
    }}
>
    <Input r#type="password" invalid={password_invalid} />
</Field>
```

### 4. Use Appropriate Heading Levels

```rust
use shadcn_rs::{Typography, TypographyVariant};

html! {
    <>
        <Typography variant={TypographyVariant::H1}>
            { "Page Title" }
        </Typography>
        <Typography variant={TypographyVariant::H2}>
            { "Section Title" }
        </Typography>
    </>
}
```

### 5. Ensure Sufficient Color Contrast

- Text should have at least 4.5:1 contrast ratio against background
- Large text (18px+ or 14px+ bold) needs at least 3:1 contrast
- Use the built-in theme colors which are tested for contrast

### 6. Don't Rely on Color Alone

```rust
// Good - uses icon and color
<Alert variant={Variant::Destructive}>
    <AlertCircleIcon />
    { "Error: Something went wrong" }
</Alert>

// Bad - only color indicates state
<span style="color: red">{ "Error" }</span>
```

### 7. Support Reduced Motion

shadcn-rs respects the `prefers-reduced-motion` media query:

```css
@media (prefers-reduced-motion: reduce) {
    .animate {
        animation: none;
        transition: none;
    }
}
```

## Testing Accessibility

### Automated Testing

Use browser developer tools:

- Chrome DevTools Accessibility panel
- Firefox Accessibility Inspector
- [axe DevTools](https://www.deque.com/axe/devtools/) browser extension

### Manual Testing

1. **Keyboard-only navigation**: Unplug your mouse and navigate your app
2. **Screen reader testing**: Test with NVDA (Windows), VoiceOver (macOS/iOS), or TalkBack (Android)
3. **Zoom testing**: Zoom to 200% and ensure content is still usable
4. **Color contrast**: Check contrast ratios with tools like [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)

### Screen Reader Commands

| Action | VoiceOver (macOS) | NVDA (Windows) |
|--------|-------------------|----------------|
| Navigate | VO + Arrow keys | Arrow keys |
| Activate | VO + Space | Enter |
| Exit | Escape | Escape |
| List headings | VO + U | H |
| List landmarks | VO + U | D |

## Common Issues and Solutions

### Issue: Focus Not Visible

**Solution**: Ensure the focus ring CSS is not overridden:

```css
:focus-visible {
    outline: 2px solid var(--ring);
    outline-offset: 2px;
}
```

### Issue: Screen Reader Announces Decorative Content

**Solution**: Use `aria-hidden` for decorative elements:

```rust
<span aria-hidden="true">{ ">" }</span>
```

### Issue: Modal Doesn't Trap Focus

**Solution**: Use the Dialog or Sheet component which handles focus trapping automatically.

### Issue: Live Regions Not Announced

**Solution**: Use appropriate ARIA live regions:

```rust
<div role="status" aria-live="polite">
    { status_message }
</div>
```

## Resources

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [WAI-ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)
- [MDN Accessibility Guide](https://developer.mozilla.org/en-US/docs/Web/Accessibility)
- [The A11Y Project](https://www.a11yproject.com/)

## Next Steps

- [Installation Guide](./installation.md) - Get started with shadcn-rs
- [Theming Guide](./theming.md) - Customize the look and feel
