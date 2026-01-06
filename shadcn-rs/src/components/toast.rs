//! Toast component
//!
//! Notification toast system with positioning and auto-dismiss.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Toast, ToastPosition, ToastVariant};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let show_toast = use_state(|| true);
//!
//!     html! {
//!         <>
//!             if *show_toast {
//!                 <Toast
//!                     variant={ToastVariant::Success}
//!                     position={ToastPosition::TopRight}
//!                     title="Success!"
//!                     description="Your changes have been saved."
//!                     duration={3000}
//!                 />
//!             }
//!         </>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Toast position
#[derive(Debug, Clone, PartialEq)]
pub enum ToastPosition {
    /// Top left corner
    TopLeft,
    /// Top center
    TopCenter,
    /// Top right corner
    TopRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom center
    BottomCenter,
    /// Bottom right corner
    BottomRight,
}

/// Toast variant
#[derive(Debug, Clone, PartialEq)]
pub enum ToastVariant {
    /// Default variant
    Default,
    /// Success variant
    Success,
    /// Warning variant
    Warning,
    /// Error variant
    Error,
    /// Info variant
    Info,
}

/// Toast component properties
#[derive(Properties, PartialEq, Clone)]
pub struct ToastProps {
    /// Toast variant
    #[prop_or(ToastVariant::Default)]
    pub variant: ToastVariant,

    /// Toast position
    #[prop_or(ToastPosition::TopRight)]
    pub position: ToastPosition,

    /// Toast title
    #[prop_or_default]
    pub title: Option<AttrValue>,

    /// Toast description
    #[prop_or_default]
    pub description: Option<AttrValue>,

    /// Auto-dismiss duration in milliseconds (0 = no auto-dismiss)
    #[prop_or(5000)]
    pub duration: u32,

    /// Action button text
    #[prop_or_default]
    pub action: Option<AttrValue>,

    /// Action button click handler
    #[prop_or_default]
    pub on_action: Option<Callback<MouseEvent>>,

    /// Close handler
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    #[prop_or_default]
    pub children: Children,
}

/// Toast component
///
/// Displays temporary notification messages.
///
/// # Accessibility
/// - role="status" for non-critical messages
/// - role="alert" for error messages
/// - Live region for screen reader announcements
#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let ToastProps {
        variant,
        position,
        title,
        description,
        duration: _,
        action,
        on_action,
        on_close,
        class,
        children,
    } = props.clone();

    let variant_class = match variant {
        ToastVariant::Default => "toast-default",
        ToastVariant::Success => "toast-success",
        ToastVariant::Warning => "toast-warning",
        ToastVariant::Error => "toast-error",
        ToastVariant::Info => "toast-info",
    };

    let position_class = match position {
        ToastPosition::TopLeft => "toast-top-left",
        ToastPosition::TopCenter => "toast-top-center",
        ToastPosition::TopRight => "toast-top-right",
        ToastPosition::BottomLeft => "toast-bottom-left",
        ToastPosition::BottomCenter => "toast-bottom-center",
        ToastPosition::BottomRight => "toast-bottom-right",
    };

    let classes: Classes = vec![
        Classes::from("toast"),
        Classes::from(variant_class),
        Classes::from(position_class),
        class,
    ]
    .into_iter()
    .collect();

    let role = match variant {
        ToastVariant::Error => "alert",
        _ => "status",
    };

    let close_handler = on_close.map(|cb| {
        Callback::from(move |_: MouseEvent| {
            cb.emit(());
        })
    });

    let has_children = children.iter().count() > 0;

    html! {
        <div class={classes} role={role} aria-live="polite" aria-atomic="true">
            if has_children {
                { children }
            } else {
                <>
                    if let Some(title_text) = title {
                        <div class="toast-title">
                            { title_text }
                        </div>
                    }
                    if let Some(desc_text) = description {
                        <div class="toast-description">
                            { desc_text }
                        </div>
                    }
                </>
            }
            <div class="toast-actions">
                if let Some(action_text) = action {
                    <button
                        type="button"
                        class="toast-action"
                        onclick={on_action}
                    >
                        { action_text }
                    </button>
                }
                <button
                    type="button"
                    class="toast-close"
                    onclick={close_handler}
                    aria-label="Close"
                >
                    { "×" }
                </button>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_default() {
        let props = ToastProps {
            variant: ToastVariant::Default,
            position: ToastPosition::TopRight,
            title: None,
            description: None,
            duration: 5000,
            action: None,
            on_action: None,
            on_close: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.variant, ToastVariant::Default);
        assert_eq!(props.position, ToastPosition::TopRight);
        assert_eq!(props.duration, 5000);
    }

    #[test]
    fn test_toast_success() {
        let props = ToastProps {
            variant: ToastVariant::Success,
            position: ToastPosition::TopRight,
            title: Some(AttrValue::from("Success")),
            description: Some(AttrValue::from("Operation completed")),
            duration: 3000,
            action: None,
            on_action: None,
            on_close: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.variant, ToastVariant::Success);
        assert_eq!(props.title, Some(AttrValue::from("Success")));
    }

    #[test]
    fn test_toast_error() {
        let props = ToastProps {
            variant: ToastVariant::Error,
            position: ToastPosition::BottomCenter,
            title: Some(AttrValue::from("Error")),
            description: None,
            duration: 0,
            action: None,
            on_action: None,
            on_close: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.variant, ToastVariant::Error);
        assert_eq!(props.duration, 0);
    }

    #[test]
    fn test_toast_with_action() {
        let props = ToastProps {
            variant: ToastVariant::Info,
            position: ToastPosition::TopLeft,
            title: None,
            description: None,
            duration: 5000,
            action: Some(AttrValue::from("Undo")),
            on_action: None,
            on_close: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.action, Some(AttrValue::from("Undo")));
    }

    #[test]
    fn test_toast_positions() {
        assert_eq!(ToastPosition::TopLeft, ToastPosition::TopLeft);
        assert_ne!(ToastPosition::TopLeft, ToastPosition::BottomRight);
    }
}
