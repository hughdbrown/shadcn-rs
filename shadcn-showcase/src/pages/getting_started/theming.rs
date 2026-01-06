//! Theming guide page

use yew::prelude::*;

use crate::components::CodeBlock;

/// Theming page component
#[function_component(ThemingPage)]
pub fn theming_page() -> Html {
    html! {
        <div class="guide-page">
            <header class="guide-header">
                <h1 class="guide-title">{ "Theming" }</h1>
                <p class="guide-description">
                    { "Customize the look and feel of shadcn-rs components using CSS variables." }
                </p>
            </header>

            <section class="guide-section">
                <h2>{ "CSS Variables" }</h2>
                <p>
                    { "shadcn-rs uses CSS variables (custom properties) for all colors, spacing, " }
                    { "and other design tokens. This makes it easy to customize the entire look " }
                    { "of your application by changing a few values." }
                </p>
            </section>

            <section class="guide-section">
                <h2>{ "Color System" }</h2>
                <p>{ "The default color palette includes the following semantic colors:" }</p>

                <CodeBlock
                    code={r#":root {
    /* Background colors */
    --background: #ffffff;
    --foreground: #0f172a;

    /* Card colors */
    --card: #ffffff;
    --card-foreground: #0f172a;

    /* Primary colors */
    --primary: #0f172a;
    --primary-foreground: #f8fafc;

    /* Secondary colors */
    --secondary: #f1f5f9;
    --secondary-foreground: #0f172a;

    /* Muted colors */
    --muted: #f1f5f9;
    --muted-foreground: #64748b;

    /* Accent colors */
    --accent: #f1f5f9;
    --accent-foreground: #0f172a;

    /* Destructive colors */
    --destructive: #ef4444;
    --destructive-foreground: #f8fafc;

    /* Border and input colors */
    --border: #e2e8f0;
    --input: #e2e8f0;
    --ring: #0f172a;

    /* Border radius */
    --radius: 0.5rem;
}"#}
                    language="css"
                    title="Light theme variables"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Dark Mode" }</h2>
                <p>
                    { "Dark mode is supported out of the box. Set the " }
                    <code>{ "data-theme=\"dark\"" }</code>
                    { " attribute on your root element to enable it:" }
                </p>

                <CodeBlock
                    code={r#"[data-theme="dark"] {
    --background: #0f172a;
    --foreground: #f8fafc;

    --card: #1e293b;
    --card-foreground: #f8fafc;

    --primary: #f8fafc;
    --primary-foreground: #0f172a;

    --secondary: #1e293b;
    --secondary-foreground: #f8fafc;

    --muted: #1e293b;
    --muted-foreground: #94a3b8;

    --accent: #1e293b;
    --accent-foreground: #f8fafc;

    --destructive: #ef4444;
    --destructive-foreground: #f8fafc;

    --border: #1e293b;
    --input: #1e293b;
    --ring: #f8fafc;
}"#}
                    language="css"
                    title="Dark theme variables"
                />

                <p>{ "Toggle dark mode programmatically in Rust:" }</p>

                <CodeBlock
                    code={r#"use web_sys::window;

fn toggle_theme(dark: bool) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(root) = document.document_element() {
                let theme = if dark { "dark" } else { "light" };
                let _ = root.set_attribute("data-theme", theme);
            }
        }
    }
}"#}
                    language="rust"
                    title="Toggle theme"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Custom Colors" }</h2>
                <p>{ "Create your own color scheme by overriding the CSS variables:" }</p>

                <CodeBlock
                    code={r#":root {
    /* Custom blue theme */
    --primary: #2563eb;
    --primary-foreground: #ffffff;

    --secondary: #dbeafe;
    --secondary-foreground: #1e40af;

    --accent: #60a5fa;
    --accent-foreground: #1e3a8a;

    /* Custom success color */
    --success: #22c55e;
    --success-foreground: #ffffff;

    /* Custom warning color */
    --warning: #f59e0b;
    --warning-foreground: #ffffff;
}"#}
                    language="css"
                    title="Custom theme"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Typography" }</h2>
                <p>{ "Customize fonts and text styles:" }</p>

                <CodeBlock
                    code={r#":root {
    /* Font family */
    --font-sans: ui-sans-serif, system-ui, -apple-system, sans-serif;
    --font-mono: ui-monospace, SFMono-Regular, monospace;

    /* Font sizes */
    --text-xs: 0.75rem;
    --text-sm: 0.875rem;
    --text-base: 1rem;
    --text-lg: 1.125rem;
    --text-xl: 1.25rem;
    --text-2xl: 1.5rem;
    --text-3xl: 1.875rem;

    /* Line heights */
    --leading-tight: 1.25;
    --leading-normal: 1.5;
    --leading-relaxed: 1.75;
}"#}
                    language="css"
                    title="Typography variables"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Spacing" }</h2>
                <p>{ "Control spacing throughout your app:" }</p>

                <CodeBlock
                    code={r#":root {
    /* Spacing scale */
    --space-1: 0.25rem;   /* 4px */
    --space-2: 0.5rem;    /* 8px */
    --space-3: 0.75rem;   /* 12px */
    --space-4: 1rem;      /* 16px */
    --space-5: 1.25rem;   /* 20px */
    --space-6: 1.5rem;    /* 24px */
    --space-8: 2rem;      /* 32px */
    --space-10: 2.5rem;   /* 40px */
    --space-12: 3rem;     /* 48px */
}"#}
                    language="css"
                    title="Spacing variables"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Shadows" }</h2>
                <p>{ "Customize shadows for elevation:" }</p>

                <CodeBlock
                    code={r#":root {
    --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
    --shadow: 0 1px 3px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06);
    --shadow-md: 0 4px 6px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.06);
    --shadow-lg: 0 10px 15px rgba(0, 0, 0, 0.1), 0 4px 6px rgba(0, 0, 0, 0.05);
    --shadow-xl: 0 20px 25px rgba(0, 0, 0, 0.1), 0 10px 10px rgba(0, 0, 0, 0.04);
}"#}
                    language="css"
                    title="Shadow variables"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Next Steps" }</h2>
                <ul class="guide-list">
                    <li>
                        <a href="/getting-started/accessibility">{ "Accessibility" }</a>
                        { " - Ensure your app is accessible" }
                    </li>
                    <li>
                        <a href="/components/button">{ "Components" }</a>
                        { " - See themed components in action" }
                    </li>
                </ul>
            </section>
        </div>
    }
}
