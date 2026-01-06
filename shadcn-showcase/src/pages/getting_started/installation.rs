//! Installation guide page

use yew::prelude::*;

use crate::components::CodeBlock;

/// Installation page component
#[function_component(InstallationPage)]
pub fn installation_page() -> Html {
    html! {
        <div class="guide-page">
            <header class="guide-header">
                <h1 class="guide-title">{ "Installation" }</h1>
                <p class="guide-description">
                    { "How to install and set up shadcn-rs in your Rust/Yew project." }
                </p>
            </header>

            <section class="guide-section">
                <h2>{ "Prerequisites" }</h2>
                <p>{ "Before you begin, make sure you have the following installed:" }</p>
                <ul class="guide-list">
                    <li>{ "Rust (1.75 or later) - " }<a href="https://rustup.rs">{ "rustup.rs" }</a></li>
                    <li>{ "wasm32-unknown-unknown target" }</li>
                    <li>{ "Trunk (for development server)" }</li>
                </ul>

                <CodeBlock
                    code="# Install the WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk"
                    language="bash"
                    title="Install prerequisites"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Add to Your Project" }</h2>
                <p>{ "Add shadcn-rs to your Cargo.toml dependencies:" }</p>

                <CodeBlock
                    code={r#"[dependencies]
yew = { version = "0.21", features = ["csr"] }
shadcn-rs = "0.1"
shadcn-icons = "0.1"  # Optional: for icon components"#}
                    language="toml"
                    title="Cargo.toml"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Include the CSS" }</h2>
                <p>
                    { "Copy the " }<code>{ "shadcn-rs.css" }</code>{ " file to your project's public directory " }
                    { "and include it in your HTML:" }
                </p>

                <CodeBlock
                    code={r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My App</title>
    <link rel="stylesheet" href="shadcn-rs.css">
</head>
<body>
</body>
</html>"#}
                    language="html"
                    title="index.html"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Project Structure" }</h2>
                <p>{ "A typical shadcn-rs project structure looks like this:" }</p>

                <CodeBlock
                    code={r#"my-app/
├── Cargo.toml
├── index.html
├── styles/
│   └── shadcn-rs.css
└── src/
    └── main.rs"#}
                    language="text"
                    title="Project structure"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Running Your Project" }</h2>
                <p>{ "Use Trunk to build and serve your project:" }</p>

                <CodeBlock
                    code={r#"# Development server with hot reload
trunk serve

# Build for production
trunk build --release"#}
                    language="bash"
                    title="Development commands"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Next Steps" }</h2>
                <ul class="guide-list">
                    <li>
                        <a href="/getting-started/quick-start">{ "Quick Start" }</a>
                        { " - Build your first component" }
                    </li>
                    <li>
                        <a href="/getting-started/theming">{ "Theming" }</a>
                        { " - Customize colors and styles" }
                    </li>
                    <li>
                        <a href="/components/button">{ "Components" }</a>
                        { " - Explore available components" }
                    </li>
                </ul>
            </section>
        </div>
    }
}
