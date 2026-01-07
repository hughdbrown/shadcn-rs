//! Quick start tutorial page

use yew::prelude::*;

use crate::components::CodeBlock;

/// Quick start page component
#[function_component(QuickStartPage)]
pub fn quick_start_page() -> Html {
    html! {
        <div class="guide-page">
            <header class="guide-header">
                <h1 class="guide-title">{ "Quick Start" }</h1>
                <p class="guide-description">
                    { "Get up and running with shadcn-rs in minutes." }
                </p>
            </header>

            <section class="guide-section">
                <h2>{ "Create a New Project" }</h2>
                <p>{ "Start by creating a new Rust project:" }</p>

                <CodeBlock
                    code={r#"cargo new my-app
    cd my-app"#}
                    language="bash"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Configure Cargo.toml" }</h2>
                <p>{ "Add the required dependencies:" }</p>

                <CodeBlock
                    code={r#"[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
yew = { version = "0.21", features = ["csr"] }
wasm-bindgen = "0.2"
shadcn-rs = "0.1""#}
                    language="toml"
                    title="Cargo.toml"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Create index.html" }</h2>
                <p>{ "Create an index.html file in your project root:" }</p>

                <CodeBlock
                    code={r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My App</title>
    <link rel="stylesheet" href="shadcn-rs.css">
    <link data-trunk rel="rust" data-wasm-opt="z" />
</head>
<body>
</body>
</html>"#}
                    language="html"
                    title="index.html"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Write Your First Component" }</h2>
                <p>{ "Replace the contents of src/main.rs:" }</p>

                <CodeBlock
                    code={r#"use yew::prelude::*;
use shadcn_rs::{Button, Variant, Size, Card, CardHeader, CardTitle, CardContent};

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };

    html! {
        <div style="padding: 2rem; max-width: 400px; margin: 0 auto;">
            <Card>
                <CardHeader>
                    <CardTitle>{ "Counter Example" }</CardTitle>
                </CardHeader>
                <CardContent>
                    <p style="margin-bottom: 1rem;">
                        { "Count: " }{ *counter }
                    </p>
                    <Button variant={Variant::Primary} size={Size::Lg} {onclick}>
                        { "Increment" }
                    </Button>
                </CardContent>
            </Card>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}"#}
                    language="rust"
                    title="src/main.rs"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Run Your App" }</h2>
                <p>{ "Start the development server:" }</p>

                <CodeBlock
                    code="trunk serve --open"
                    language="bash"
                />

                <p>
                    { "Your app should now be running at " }
                    <code>{ "http://localhost:8080" }</code>
                </p>
            </section>

            <section class="guide-section">
                <h2>{ "Using More Components" }</h2>
                <p>{ "Here's a more complete example using multiple components:" }</p>

                <CodeBlock
                    code={r#"use yew::prelude::*;
use shadcn_rs::{
    Button, Variant, Size,
    Input, Label,
    Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter,
    Alert, AlertVariant,
};

#[function_component(LoginForm)]
fn login_form() -> Html {
    let email = use_state(String::new);
    let password = use_state(String::new);
    let error = use_state(|| None::<String>);

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let onsubmit = {
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            error.set(Some("This is a demo - no actual login".to_string()));
        })
    };

    html! {
        <Card>
            <CardHeader>
                <CardTitle>{ "Login" }</CardTitle>
                <CardDescription>{ "Enter your credentials" }</CardDescription>
            </CardHeader>
            <form {onsubmit}>
                <CardContent>
                    if let Some(err) = (*error).clone() {
                        <Alert variant={AlertVariant::Warning}>
                            { err }
                        </Alert>
                    }
                    <div style="margin-bottom: 1rem;">
                        <Label html_for="email">{ "Email" }</Label>
                        <Input
                            id="email"
                            input_type="email"
                            placeholder="you@example.com"
                            oninput={on_email_change}
                        />
                    </div>
                    <div style="margin-bottom: 1rem;">
                        <Label html_for="password">{ "Password" }</Label>
                        <Input
                            id="password"
                            input_type="password"
                            placeholder="Enter password"
                            oninput={on_password_change}
                        />
                    </div>
                </CardContent>
                <CardFooter>
                    <Button variant={Variant::Primary} button_type="submit">
                        { "Sign In" }
                    </Button>
                </CardFooter>
            </form>
        </Card>
    }
}"#}
                    language="rust"
                    title="Login form example"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Next Steps" }</h2>
                <ul class="guide-list">
                    <li>
                        <a href="/getting-started/theming">{ "Theming" }</a>
                        { " - Learn how to customize styles" }
                    </li>
                    <li>
                        <a href="/getting-started/accessibility">{ "Accessibility" }</a>
                        { " - Make your app accessible" }
                    </li>
                    <li>
                        <a href="/components/button">{ "Component Reference" }</a>
                        { " - Explore all components" }
                    </li>
                </ul>
            </section>
        </div>
    }
}
