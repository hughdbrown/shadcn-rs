//! Home page for the showcase application

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

/// Home page component
#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="home-page">
            <section class="hero">
                <h1 class="hero-title">{ "shadcn-rs" }</h1>
                <p class="hero-subtitle">
                    { "Beautiful, accessible UI components for Rust and WebAssembly" }
                </p>
                <p class="hero-description">
                    { "A comprehensive port of shadcn/ui to Rust using the Yew framework. " }
                    { "Build modern web applications with type-safe, accessible components." }
                </p>
                <div class="hero-actions">
                    <Link<Route> to={Route::Installation} classes="btn btn-primary btn-lg">
                        { "Get Started" }
                    </Link<Route>>
                    <Link<Route> to={Route::Button} classes="btn btn-outline btn-lg">
                        { "Browse Components" }
                    </Link<Route>>
                </div>
            </section>

            <section class="features">
                <h2 class="section-title">{ "Features" }</h2>
                <div class="feature-grid">
                    <div class="feature-card">
                        <div class="feature-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
                                <path d="M2 17l10 5 10-5"></path>
                                <path d="M2 12l10 5 10-5"></path>
                            </svg>
                        </div>
                        <h3 class="feature-title">{ "60+ Components" }</h3>
                        <p class="feature-description">
                            { "A comprehensive library of UI components, from buttons and inputs to complex data tables and charts." }
                        </p>
                    </div>

                    <div class="feature-card">
                        <div class="feature-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <circle cx="12" cy="12" r="10"></circle>
                                <path d="M12 6v6l4 2"></path>
                            </svg>
                        </div>
                        <h3 class="feature-title">{ "Type-Safe" }</h3>
                        <p class="feature-description">
                            { "Built with Rust's type system. Catch errors at compile time, not runtime." }
                        </p>
                    </div>

                    <div class="feature-card">
                        <div class="feature-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                                <circle cx="12" cy="12" r="3"></circle>
                            </svg>
                        </div>
                        <h3 class="feature-title">{ "Accessible" }</h3>
                        <p class="feature-description">
                            { "Full keyboard navigation, ARIA attributes, and screen reader support built-in." }
                        </p>
                    </div>

                    <div class="feature-card">
                        <div class="feature-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <circle cx="12" cy="12" r="5"></circle>
                                <line x1="12" y1="1" x2="12" y2="3"></line>
                                <line x1="12" y1="21" x2="12" y2="23"></line>
                                <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                                <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                                <line x1="1" y1="12" x2="3" y2="12"></line>
                                <line x1="21" y1="12" x2="23" y2="12"></line>
                                <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                                <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                            </svg>
                        </div>
                        <h3 class="feature-title">{ "Dark Mode" }</h3>
                        <p class="feature-description">
                            { "CSS variable-based theming with built-in light and dark mode support." }
                        </p>
                    </div>

                    <div class="feature-card">
                        <div class="feature-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
                                <line x1="8" y1="21" x2="16" y2="21"></line>
                                <line x1="12" y1="17" x2="12" y2="21"></line>
                            </svg>
                        </div>
                        <h3 class="feature-title">{ "WebAssembly" }</h3>
                        <p class="feature-description">
                            { "Compile to WebAssembly for near-native performance in the browser." }
                        </p>
                    </div>

                    <div class="feature-card">
                        <div class="feature-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"></path>
                            </svg>
                        </div>
                        <h3 class="feature-title">{ "Customizable" }</h3>
                        <p class="feature-description">
                            { "CSS-based styling with CSS variables for easy customization and theming." }
                        </p>
                    </div>
                </div>
            </section>

            <section class="quick-start">
                <h2 class="section-title">{ "Quick Start" }</h2>
                <div class="quick-start-content">
                    <div class="quick-start-step">
                        <h3>{ "1. Add to Cargo.toml" }</h3>
                        <pre class="code-inline"><code>{ r#"[dependencies]
    shadcn-rs = "0.1""# }</code></pre>
                    </div>
                    <div class="quick-start-step">
                        <h3>{ "2. Include the CSS" }</h3>
                        <pre class="code-inline"><code>{ r#"<link rel="stylesheet" href="shadcn-rs.css">"# }</code></pre>
                    </div>
                    <div class="quick-start-step">
                        <h3>{ "3. Use components" }</h3>
                        <pre class="code-inline"><code>{ r#"use shadcn_rs::{Button, Variant};

html! {
    <Button variant={Variant::Primary}>
        { "Click me" }
    </Button>
}"# }</code></pre>
                    </div>
                </div>
            </section>

            <section class="component-preview">
                <h2 class="section-title">{ "Component Categories" }</h2>
                <div class="category-grid">
                    <Link<Route> to={Route::Button} classes="category-card">
                        <h3>{ "Foundational" }</h3>
                        <p>{ "Button, Badge, Alert, Card, Avatar..." }</p>
                    </Link<Route>>
                    <Link<Route> to={Route::Input} classes="category-card">
                        <h3>{ "Form Components" }</h3>
                        <p>{ "Input, Checkbox, Select, Slider, Switch..." }</p>
                    </Link<Route>>
                    <Link<Route> to={Route::Table} classes="category-card">
                        <h3>{ "Layout & Structure" }</h3>
                        <p>{ "Table, Card, Tabs, Resizable..." }</p>
                    </Link<Route>>
                    <Link<Route> to={Route::Dialog} classes="category-card">
                        <h3>{ "Overlays & Popups" }</h3>
                        <p>{ "Dialog, Popover, Tooltip, Sheet..." }</p>
                    </Link<Route>>
                    <Link<Route> to={Route::DropdownMenu} classes="category-card">
                        <h3>{ "Navigation" }</h3>
                        <p>{ "Menu, Breadcrumb, Pagination, Sidebar..." }</p>
                    </Link<Route>>
                    <Link<Route> to={Route::DataTable} classes="category-card">
                        <h3>{ "Complex" }</h3>
                        <p>{ "DataTable, Chart, Carousel, Toast..." }</p>
                    </Link<Route>>
                </div>
            </section>
        </div>
    }
}
