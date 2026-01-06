//! shadcn-rs: A comprehensive UI component library for Rust/WebAssembly
//!
//! This library provides shadcn/ui compatible components for building modern web
//! applications with Rust and Yew.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Button, Variant, Size};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let onclick = Callback::from(|_| {
//!         web_sys::console::log_1(&"Button clicked!".into());
//!     });
//!
//!     html! {
//!         <Button variant={Variant::Primary} size={Size::Lg} onclick={onclick}>
//!             { "Click me" }
//!         </Button>
//!     }
//! }
//! ```
//!
//! # Features
//!
//! - 59+ accessible, customizable components
//! - Full keyboard navigation support
//! - Dark mode support via CSS variables
//! - Touch gesture support for mobile
//! - Type-safe component APIs with Rust enums
//!
//! # CSS
//!
//! Include the shadcn-rs CSS file in your HTML:
//!
//! ```html
//! <link rel="stylesheet" href="shadcn-rs.css">
//! ```

#![warn(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod components;
pub mod hooks;
pub mod types;
pub mod utils;

// Re-export commonly used types
pub use types::{Size, Variant, Color, Position, Alignment};

// Re-export commonly used utilities
pub use utils::{generate_id, class_names, class_if, Portal, use_portal};

// Re-export commonly used hooks
pub use hooks::{
    use_toggle, use_toggle_with_controls,
    use_click_outside, use_click_outside_conditional,
    use_escape_key, use_escape_key_conditional, use_key_press,
    use_controllable_state, use_controllable_state_optional, use_controllable_bool,
};

// Re-export all components
pub use components::*;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
