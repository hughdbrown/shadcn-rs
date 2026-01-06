//! Accessibility guide page

use yew::prelude::*;

use crate::components::CodeBlock;

/// Accessibility page component
#[function_component(AccessibilityPage)]
pub fn accessibility_page() -> Html {
    html! {
        <div class="guide-page">
            <header class="guide-header">
                <h1 class="guide-title">{ "Accessibility" }</h1>
                <p class="guide-description">
                    { "shadcn-rs is built with accessibility in mind. Learn how to keep your app accessible." }
                </p>
            </header>

            <section class="guide-section">
                <h2>{ "Built-in Accessibility" }</h2>
                <p>{ "All shadcn-rs components include:" }</p>
                <ul class="guide-list">
                    <li>{ "Proper ARIA attributes and roles" }</li>
                    <li>{ "Keyboard navigation support" }</li>
                    <li>{ "Focus management and visible focus indicators" }</li>
                    <li>{ "Screen reader announcements where appropriate" }</li>
                    <li>{ "Color contrast meeting WCAG guidelines" }</li>
                </ul>
            </section>

            <section class="guide-section">
                <h2>{ "Keyboard Navigation" }</h2>
                <p>{ "All interactive components support keyboard navigation:" }</p>

                <div class="guide-table">
                    <table>
                        <thead>
                            <tr>
                                <th>{ "Component" }</th>
                                <th>{ "Keys" }</th>
                                <th>{ "Action" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>{ "Button" }</td>
                                <td><code>{ "Enter" }</code>{ ", " }<code>{ "Space" }</code></td>
                                <td>{ "Activate button" }</td>
                            </tr>
                            <tr>
                                <td>{ "Dialog" }</td>
                                <td><code>{ "Escape" }</code></td>
                                <td>{ "Close dialog" }</td>
                            </tr>
                            <tr>
                                <td>{ "Dialog" }</td>
                                <td><code>{ "Tab" }</code></td>
                                <td>{ "Cycle through focusable elements" }</td>
                            </tr>
                            <tr>
                                <td>{ "Menu" }</td>
                                <td><code>{ "Arrow keys" }</code></td>
                                <td>{ "Navigate items" }</td>
                            </tr>
                            <tr>
                                <td>{ "Menu" }</td>
                                <td><code>{ "Enter" }</code></td>
                                <td>{ "Select item" }</td>
                            </tr>
                            <tr>
                                <td>{ "Tabs" }</td>
                                <td><code>{ "Arrow Left/Right" }</code></td>
                                <td>{ "Switch tabs" }</td>
                            </tr>
                            <tr>
                                <td>{ "Switch" }</td>
                                <td><code>{ "Space" }</code></td>
                                <td>{ "Toggle state" }</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </section>

            <section class="guide-section">
                <h2>{ "ARIA Patterns" }</h2>
                <p>{ "Components implement standard ARIA patterns. Here's an example with Dialog:" }</p>

                <CodeBlock
                    code={r#"// The Dialog component automatically manages:
// - role="dialog" on the dialog container
// - aria-modal="true" for modal dialogs
// - aria-labelledby pointing to the title
// - aria-describedby pointing to the description
// - Focus trap to keep focus within the dialog
// - Focus restoration when dialog closes

html! {
    <Dialog open={*open} on_close={on_close}>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>{ "Confirm Action" }</DialogTitle>
                <DialogDescription>
                    { "Are you sure you want to proceed?" }
                </DialogDescription>
            </DialogHeader>
            <DialogFooter>
                <Button variant={Variant::Ghost} onclick={on_cancel}>
                    { "Cancel" }
                </Button>
                <Button variant={Variant::Primary} onclick={on_confirm}>
                    { "Confirm" }
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
}"#}
                    language="rust"
                    title="Accessible Dialog"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Form Accessibility" }</h2>
                <p>{ "Always associate labels with form inputs:" }</p>

                <CodeBlock
                    code={r#"use shadcn_rs::{Label, Input, Field};

// Using Field component (recommended)
html! {
    <Field
        label="Email address"
        html_for="email"
        required={true}
        error={(*error).clone()}
    >
        <Input
            id="email"
            input_type="email"
            placeholder="you@example.com"
            required={true}
            aria_invalid={error.is_some()}
        />
    </Field>
}

// Or manually with Label and Input
html! {
    <div>
        <Label html_for="email" required={true}>
            { "Email address" }
        </Label>
        <Input
            id="email"
            input_type="email"
            aria_describedby="email-error"
        />
        if let Some(err) = &*error {
            <span id="email-error" class="error-text">
                { err }
            </span>
        }
    </div>
}"#}
                    language="rust"
                    title="Accessible forms"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Focus Management" }</h2>
                <p>{ "Use focus trap for modals and popovers:" }</p>

                <CodeBlock
                    code={r#"use shadcn_rs::utils::focus_trap;

// Focus trap is automatically applied to:
// - Dialog
// - AlertDialog
// - Sheet
// - Drawer

// For custom components, use the focus trap hook:
use shadcn_rs::hooks::use_focus_trap;

#[function_component(CustomModal)]
fn custom_modal() -> Html {
    let container_ref = use_node_ref();

    // Enable focus trap when modal is open
    use_focus_trap(container_ref.clone(), *is_open);

    html! {
        <div ref={container_ref} class="modal">
            // Modal content
        </div>
    }
}"#}
                    language="rust"
                    title="Focus management"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Screen Reader Announcements" }</h2>
                <p>{ "Use live regions for dynamic content:" }</p>

                <CodeBlock
                    code={r#"// Toast and Alert components use aria-live regions
html! {
    <Alert variant={AlertVariant::Success}>
        { "Changes saved successfully" }
    </Alert>
}

// For custom announcements:
use shadcn_rs::utils::announce;

// Announce a message to screen readers
announce("Item added to cart", "polite");
announce("Error: Invalid input", "assertive");"#}
                    language="rust"
                    title="Screen reader announcements"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Color Contrast" }</h2>
                <p>
                    { "The default theme meets WCAG AA contrast requirements. " }
                    { "When customizing colors, use tools to verify contrast:" }
                </p>
                <ul class="guide-list">
                    <li>
                        <a href="https://webaim.org/resources/contrastchecker/" target="_blank">
                            { "WebAIM Contrast Checker" }
                        </a>
                    </li>
                    <li>
                        <a href="https://accessibleweb.com/color-contrast-checker/" target="_blank">
                            { "Accessible Web Color Contrast Checker" }
                        </a>
                    </li>
                </ul>
            </section>

            <section class="guide-section">
                <h2>{ "Testing Accessibility" }</h2>
                <p>{ "Recommended tools for testing accessibility:" }</p>
                <ul class="guide-list">
                    <li>{ "Browser DevTools Accessibility panel" }</li>
                    <li>{ "axe browser extension" }</li>
                    <li>{ "NVDA or VoiceOver screen readers" }</li>
                    <li>{ "Keyboard-only navigation testing" }</li>
                    <li>{ "Lighthouse accessibility audit" }</li>
                </ul>
            </section>

            <section class="guide-section">
                <h2>{ "Best Practices" }</h2>
                <ul class="guide-list">
                    <li>{ "Always provide alternative text for images" }</li>
                    <li>{ "Use semantic HTML elements" }</li>
                    <li>{ "Ensure interactive elements are focusable" }</li>
                    <li>{ "Provide visible focus indicators" }</li>
                    <li>{ "Don't rely solely on color to convey information" }</li>
                    <li>{ "Test with keyboard navigation" }</li>
                    <li>{ "Test with screen readers" }</li>
                </ul>
            </section>
        </div>
    }
}
