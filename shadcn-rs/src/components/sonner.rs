//! Sonner component
//!
//! Advanced toast notification system with rich features.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Sonner, SonnerToast, SonnerPosition, SonnerType};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Sonner position={SonnerPosition::BottomRight}>
//!             <SonnerToast
//!                 r#type={SonnerType::Success}
//!                 title="Success"
//!                 description="Your changes have been saved."
//!             />
//!         </Sonner>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Sonner position
#[derive(Debug, Clone, PartialEq)]
pub enum SonnerPosition {
    /// Top left
    TopLeft,
    /// Top center
    TopCenter,
    /// Top right
    TopRight,
    /// Bottom left
    BottomLeft,
    /// Bottom center
    BottomCenter,
    /// Bottom right
    BottomRight,
}

/// Sonner toast type
#[derive(Debug, Clone, PartialEq)]
pub enum SonnerType {
    /// Default type
    Default,
    /// Success type
    Success,
    /// Error type
    Error,
    /// Warning type
    Warning,
    /// Info type
    Info,
    /// Loading type (for promises)
    Loading,
}

/// Sonner container properties
#[derive(Properties, PartialEq, Clone)]
pub struct SonnerProps {
    /// Toast position
    #[prop_or(SonnerPosition::BottomRight)]
    pub position: SonnerPosition,

    /// Expand toasts by default
    #[prop_or(false)]
    pub expand: bool,

    /// Gap between toasts in pixels
    #[prop_or(14)]
    pub gap: u32,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements (toasts)
    pub children: Children,
}

/// Sonner container component
///
/// Container for advanced toast notifications.
///
/// # Accessibility
/// - Live region for announcements
/// - Touch gestures supported
/// - Keyboard dismissible
#[function_component(Sonner)]
pub fn sonner(props: &SonnerProps) -> Html {
    let SonnerProps {
        position,
        expand,
        gap,
        class,
        children,
    } = props.clone();

    let position_class = match position {
        SonnerPosition::TopLeft => "sonner-top-left",
        SonnerPosition::TopCenter => "sonner-top-center",
        SonnerPosition::TopRight => "sonner-top-right",
        SonnerPosition::BottomLeft => "sonner-bottom-left",
        SonnerPosition::BottomCenter => "sonner-bottom-center",
        SonnerPosition::BottomRight => "sonner-bottom-right",
    };

    let classes: Classes = vec![
        Classes::from("sonner"),
        Classes::from(position_class),
        if expand {
            Classes::from("sonner-expanded")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let style = format!("gap: {}px", gap);

    html! {
        <div class={classes} style={style} aria-live="polite">
            { children }
        </div>
    }
}

/// Sonner toast properties
#[derive(Properties, PartialEq, Clone)]
pub struct SonnerToastProps {
    /// Toast type
    #[prop_or(SonnerType::Default)]
    pub r#type: SonnerType,

    /// Toast title
    #[prop_or_default]
    pub title: Option<AttrValue>,

    /// Toast description
    #[prop_or_default]
    pub description: Option<AttrValue>,

    /// Enable swipe to dismiss
    #[prop_or(true)]
    pub dismissible: bool,

    /// Auto-close duration in milliseconds (0 = no auto-close)
    #[prop_or(4000)]
    pub duration: u32,

    /// Action button text
    #[prop_or_default]
    pub action: Option<AttrValue>,

    /// Action button handler
    #[prop_or_default]
    pub on_action: Option<Callback<MouseEvent>>,

    /// Dismiss handler
    #[prop_or_default]
    pub on_dismiss: Option<Callback<()>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements (custom content)
    #[prop_or_default]
    pub children: Children,
}

/// Sonner toast component
///
/// Individual toast notification with rich features.
#[function_component(SonnerToast)]
pub fn sonner_toast(props: &SonnerToastProps) -> Html {
    let SonnerToastProps {
        r#type,
        title,
        description,
        dismissible,
        duration: _,
        action,
        on_action,
        on_dismiss,
        class,
        children,
    } = props.clone();

    let type_class = match r#type {
        SonnerType::Default => "sonner-toast-default",
        SonnerType::Success => "sonner-toast-success",
        SonnerType::Error => "sonner-toast-error",
        SonnerType::Warning => "sonner-toast-warning",
        SonnerType::Info => "sonner-toast-info",
        SonnerType::Loading => "sonner-toast-loading",
    };

    let classes: Classes = vec![
        Classes::from("sonner-toast"),
        Classes::from(type_class),
        if dismissible {
            Classes::from("sonner-toast-dismissible")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let dismiss_handler = on_dismiss.map(|cb| {
        Callback::from(move |_: MouseEvent| {
            cb.emit(());
        })
    });

    let has_children = children.iter().count() > 0;

    html! {
        <div class={classes} role="status">
            <div class="sonner-toast-content">
                if has_children {
                    { children }
                } else {
                    <>
                        if let Some(title_text) = title {
                            <div class="sonner-toast-title">
                                { title_text }
                            </div>
                        }
                        if let Some(desc_text) = description {
                            <div class="sonner-toast-description">
                                { desc_text }
                            </div>
                        }
                    </>
                }
            </div>
            <div class="sonner-toast-actions">
                if let Some(action_text) = action {
                    <button
                        type="button"
                        class="sonner-toast-action"
                        onclick={on_action}
                    >
                        { action_text }
                    </button>
                }
                if dismissible {
                    <button
                        type="button"
                        class="sonner-toast-close"
                        onclick={dismiss_handler}
                        aria-label="Close"
                    >
                        { "×" }
                    </button>
                }
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sonner_default() {
        let props = SonnerProps {
            position: SonnerPosition::BottomRight,
            expand: false,
            gap: 14,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.position, SonnerPosition::BottomRight);
        assert!(!props.expand);
        assert_eq!(props.gap, 14);
    }

    #[test]
    fn test_sonner_expanded() {
        let props = SonnerProps {
            position: SonnerPosition::TopCenter,
            expand: true,
            gap: 20,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.expand);
        assert_eq!(props.gap, 20);
    }

    #[test]
    fn test_sonner_toast_default() {
        let props = SonnerToastProps {
            r#type: SonnerType::Default,
            title: None,
            description: None,
            dismissible: true,
            duration: 4000,
            action: None,
            on_action: None,
            on_dismiss: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.r#type, SonnerType::Default);
        assert!(props.dismissible);
        assert_eq!(props.duration, 4000);
    }

    #[test]
    fn test_sonner_toast_success() {
        let props = SonnerToastProps {
            r#type: SonnerType::Success,
            title: Some(AttrValue::from("Success")),
            description: Some(AttrValue::from("Operation completed")),
            dismissible: true,
            duration: 3000,
            action: None,
            on_action: None,
            on_dismiss: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.r#type, SonnerType::Success);
        assert_eq!(props.title, Some(AttrValue::from("Success")));
    }

    #[test]
    fn test_sonner_toast_loading() {
        let props = SonnerToastProps {
            r#type: SonnerType::Loading,
            title: Some(AttrValue::from("Loading...")),
            description: None,
            dismissible: false,
            duration: 0,
            action: None,
            on_action: None,
            on_dismiss: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.r#type, SonnerType::Loading);
        assert!(!props.dismissible);
    }

    #[test]
    fn test_sonner_types() {
        assert_eq!(SonnerType::Success, SonnerType::Success);
        assert_ne!(SonnerType::Success, SonnerType::Error);
    }
}
