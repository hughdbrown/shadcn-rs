//! Button component
//!
//! Displays a button or a component that looks like a button.
//!
//! # Examples
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
//!         <Button variant={Variant::Primary} size={Size::Md} {onclick}>
//!             { "Click me" }
//!         </Button>
//!     }
//! }
//! ```

use crate::types::{Size, Variant};
use crate::utils::class_names;
use yew::prelude::*;

/// Button component properties
#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    /// Button variant style
    #[prop_or(Variant::Default)]
    pub variant: Variant,

    /// Button size
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Loading state (shows spinner, disables interaction)
    #[prop_or(false)]
    pub loading: bool,

    /// Full width button
    #[prop_or(false)]
    pub full_width: bool,

    /// Button type attribute
    #[prop_or(AttrValue::from("button"))]
    pub r#type: AttrValue,

    /// Click event handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// ARIA label for accessibility
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Children elements
    #[prop_or_default]
    pub children: Children,
}

/// Button component
///
/// A versatile button component with multiple variants, sizes, and states.
///
/// # Variants
/// - `Default`: Standard button
/// - `Primary`: Primary action button
/// - `Secondary`: Secondary action button
/// - `Destructive`: Destructive action button
/// - `Outline`: Outlined button
/// - `Ghost`: Minimal ghost button
/// - `Link`: Link-styled button
///
/// # Sizes
/// - `Xs`: Extra small
/// - `Sm`: Small
/// - `Md`: Medium (default)
/// - `Lg`: Large
/// - `Xl`: Extra large
/// - `Xl2`: 2X large
///
/// # States
/// - Normal: Default interactive state
/// - Disabled: Non-interactive, visually muted
/// - Loading: Shows loading indicator, non-interactive
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        variant,
        size,
        disabled,
        loading,
        full_width,
        r#type,
        onclick,
        class,
        style,
        aria_label,
        id,
        children,
    } = props.clone();

    // Build class names
    let classes = class_names(&[
        Some("btn"),
        Some(variant.to_class()),
        Some(size.to_class()),
        if full_width {
            Some("btn-full-width")
        } else {
            None
        },
        if loading { Some("btn-loading") } else { None },
    ]);

    // Merge with custom classes
    let final_classes: Classes = vec![classes, class].into_iter().collect();

    // Handle click events (disabled if loading or disabled)
    let click_handler = onclick.map(|callback| {
        Callback::from(move |e: MouseEvent| {
            if !disabled && !loading {
                callback.emit(e);
            }
        })
    });

    // Determine actual disabled state
    let is_disabled = disabled || loading;

    html! {
        <button
            type={r#type}
            class={final_classes}
            disabled={is_disabled}
            onclick={click_handler}
            style={style}
            aria-label={aria_label}
            aria-busy={loading.to_string()}
            id={id}
        >
            if loading {
                <span class="btn-spinner" aria-hidden="true"></span>
            }
            <span class={if loading { "btn-content-loading" } else { "btn-content" }}>
                { children }
            </span>
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_props_default() {
        // Test that button props can be created with defaults
        let props = ButtonProps {
            variant: Variant::Default,
            size: Size::Md,
            disabled: false,
            loading: false,
            full_width: false,
            r#type: AttrValue::from("button"),
            onclick: None,
            class: Classes::new(),
            style: None,
            aria_label: None,
            id: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.variant, Variant::Default);
        assert_eq!(props.size, Size::Md);
        assert!(!props.disabled);
    }

    #[test]
    fn test_button_variants() {
        // Test different button variants
        let variants = vec![
            Variant::Default,
            Variant::Primary,
            Variant::Secondary,
            Variant::Destructive,
            Variant::Outline,
            Variant::Ghost,
            Variant::Link,
        ];

        for variant in variants {
            let props = ButtonProps {
                variant: variant.clone(),
                size: Size::Md,
                disabled: false,
                loading: false,
                full_width: false,
                r#type: AttrValue::from("button"),
                onclick: None,
                class: Classes::new(),
                style: None,
                aria_label: None,
                id: None,
                children: Children::new(vec![]),
            };
            assert_eq!(props.variant, variant);
        }
    }

    #[test]
    fn test_button_disabled_state() {
        let props = ButtonProps {
            variant: Variant::Primary,
            size: Size::Md,
            disabled: true,
            loading: false,
            full_width: false,
            r#type: AttrValue::from("button"),
            onclick: None,
            class: Classes::new(),
            style: None,
            aria_label: None,
            id: None,
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_button_loading_state() {
        let props = ButtonProps {
            variant: Variant::Primary,
            size: Size::Md,
            disabled: false,
            loading: true,
            full_width: false,
            r#type: AttrValue::from("button"),
            onclick: None,
            class: Classes::new(),
            style: None,
            aria_label: None,
            id: None,
            children: Children::new(vec![]),
        };

        assert!(props.loading);
    }
}
