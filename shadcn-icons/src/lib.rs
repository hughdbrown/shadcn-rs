//! shadcn-icons: Lucide icons for Rust/Yew
//!
//! This library provides Lucide icons as Yew components for use with shadcn-rs.
//!
//! # Usage
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_icons::{Check, X, ChevronDown};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <div>
//!             <Check />
//!             <X />
//!             <ChevronDown />
//!         </div>
//!     }
//! }
//! ```
//!
//! # Icon Components
//!
//! All icons are exported as individual Yew components that render inline SVG.
//! Icons accept optional `class` and `style` props for customization.

#![warn(missing_docs)]

use yew::prelude::*;

/// Properties for icon components
#[derive(Properties, PartialEq, Clone)]
pub struct IconProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// Icon width (defaults to 24)
    #[prop_or(24)]
    pub width: u32,

    /// Icon height (defaults to 24)
    #[prop_or(24)]
    pub height: u32,

    /// Stroke width (defaults to 2)
    #[prop_or(2)]
    pub stroke_width: u32,

    /// Icon color (defaults to "currentColor")
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
}

// Placeholder icons - will be replaced with generated code

/// Check icon (checkmark)
#[function_component(Check)]
pub fn check(props: &IconProps) -> Html {
    html! {
        <svg
            class={classes!("icon", "icon-check", props.class.clone())}
            style={props.style.clone()}
            width={props.width.to_string()}
            height={props.height.to_string()}
            viewBox="0 0 24 24"
            fill="none"
            stroke={props.color.clone()}
            stroke-width={props.stroke_width.to_string()}
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M20 6 9 17l-5-5" />
        </svg>
    }
}

/// X icon (close/remove)
#[function_component(X)]
pub fn x(props: &IconProps) -> Html {
    html! {
        <svg
            class={classes!("icon", "icon-x", props.class.clone())}
            style={props.style.clone()}
            width={props.width.to_string()}
            height={props.height.to_string()}
            viewBox="0 0 24 24"
            fill="none"
            stroke={props.color.clone()}
            stroke-width={props.stroke_width.to_string()}
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M18 6 6 18" />
            <path d="m6 6 12 12" />
        </svg>
    }
}

/// ChevronDown icon
#[function_component(ChevronDown)]
pub fn chevron_down(props: &IconProps) -> Html {
    html! {
        <svg
            class={classes!("icon", "icon-chevron-down", props.class.clone())}
            style={props.style.clone()}
            width={props.width.to_string()}
            height={props.height.to_string()}
            viewBox="0 0 24 24"
            fill="none"
            stroke={props.color.clone()}
            stroke-width={props.stroke_width.to_string()}
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="m6 9 6 6 6-6" />
        </svg>
    }
}

/// ChevronUp icon
#[function_component(ChevronUp)]
pub fn chevron_up(props: &IconProps) -> Html {
    html! {
        <svg
            class={classes!("icon", "icon-chevron-up", props.class.clone())}
            style={props.style.clone()}
            width={props.width.to_string()}
            height={props.height.to_string()}
            viewBox="0 0 24 24"
            fill="none"
            stroke={props.color.clone()}
            stroke-width={props.stroke_width.to_string()}
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="m18 15-6-6-6 6" />
        </svg>
    }
}

/// ChevronLeft icon
#[function_component(ChevronLeft)]
pub fn chevron_left(props: &IconProps) -> Html {
    html! {
        <svg
            class={classes!("icon", "icon-chevron-left", props.class.clone())}
            style={props.style.clone()}
            width={props.width.to_string()}
            height={props.height.to_string()}
            viewBox="0 0 24 24"
            fill="none"
            stroke={props.color.clone()}
            stroke-width={props.stroke_width.to_string()}
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="m15 18-6-6 6-6" />
        </svg>
    }
}

/// ChevronRight icon
#[function_component(ChevronRight)]
pub fn chevron_right(props: &IconProps) -> Html {
    html! {
        <svg
            class={classes!("icon", "icon-chevron-right", props.class.clone())}
            style={props.style.clone()}
            width={props.width.to_string()}
            height={props.height.to_string()}
            viewBox="0 0 24 24"
            fill="none"
            stroke={props.color.clone()}
            stroke-width={props.stroke_width.to_string()}
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="m9 18 6-6-6-6" />
        </svg>
    }
}

// TODO: Generate remaining Lucide icons
// This is a placeholder implementation with common icons
// A code generation script will be created to port all Lucide icons
