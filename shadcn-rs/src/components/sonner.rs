//! Sonner component (deprecated)
//!
//! Upstream shadcn/ui folded Sonner into Toast. Use [`Toaster`] and
//! [`use_toast`] from the [`toast`](super::toast) module instead:
//!
//! | Deprecated            | Replacement                                    |
//! |-----------------------|------------------------------------------------|
//! | `SonnerPosition`      | [`ToastPosition`] (type alias, same variants)  |
//! | `SonnerType`          | [`ToastType`] (type alias, same variants)      |
//! | `Sonner`              | [`Toaster`]                                    |
//! | `SonnerToast`         | `use_toast().add(ToastOptions { .. })`         |
//!
//! `Sonner` and `SonnerToast` still render the old declarative markup (now
//! styled with the toaster CSS) so existing code keeps working for one
//! release.
//!
//! # Migration
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{use_toast, Button, ToastPosition, Toaster};
//!
//! #[function_component(Notify)]
//! fn notify() -> Html {
//!     let toast = use_toast();
//!     let onclick = Callback::from(move |_: MouseEvent| {
//!         toast.success("Your changes have been saved.");
//!     });
//!     html! { <Button {onclick}>{ "Save" }</Button> }
//! }
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Toaster position={ToastPosition::BottomRight}>
//!             <Notify />
//!         </Toaster>
//!     }
//! }
//! ```
//!
//! [`Toaster`]: super::toast::Toaster
//! [`use_toast`]: super::toast::use_toast
//! [`ToastPosition`]: super::toast::ToastPosition
//! [`ToastType`]: super::toast::ToastType

#![allow(deprecated)]

use gloo::timers::callback::Timeout;
use yew::prelude::*;

use super::toast::{ToastPosition, ToastType};

/// Sonner position
#[deprecated(since = "0.2.0", note = "use `ToastPosition` with `Toaster`")]
pub type SonnerPosition = ToastPosition;

/// Sonner toast type
#[deprecated(since = "0.2.0", note = "use `ToastType` with `use_toast()`")]
pub type SonnerType = ToastType;

/// Sonner container properties
#[deprecated(since = "0.2.0", note = "use `Toaster` and `ToasterProps`")]
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

/// Deprecated `Sonner` container; use [`Toaster`](super::toast::Toaster).
#[deprecated(since = "0.2.0", note = "use `Toaster` and `use_toast()`")]
pub type Sonner = LegacySonner;

/// Implementation behind the deprecated [`Sonner`] alias.
#[doc(hidden)]
#[function_component(LegacySonner)]
pub fn sonner(props: &SonnerProps) -> Html {
    let SonnerProps {
        position,
        expand,
        gap,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("sonner"),
        Classes::from(format!("sonner-{}", position.as_str())),
        Classes::from("toaster"),
        Classes::from(format!("toaster-{}", position.as_str())),
        if expand {
            Classes::from("sonner-expanded")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let style = format!("--toast-gap: {gap}px");

    html! {
        <div class={classes} style={style} aria-live="polite">
            { children }
        </div>
    }
}

/// Sonner toast properties
#[deprecated(since = "0.2.0", note = "use `ToastOptions` with `use_toast()`")]
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

    /// Show a close button
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

/// Deprecated `SonnerToast`; use [`use_toast`](super::toast::use_toast).
#[deprecated(since = "0.2.0", note = "use `use_toast().add(ToastOptions)`")]
pub type SonnerToast = LegacySonnerToast;

/// Implementation behind the deprecated [`SonnerToast`] alias.
#[doc(hidden)]
#[function_component(LegacySonnerToast)]
pub fn sonner_toast(props: &SonnerToastProps) -> Html {
    let SonnerToastProps {
        r#type,
        title,
        description,
        dismissible,
        duration,
        action,
        on_action,
        on_dismiss,
        class,
        children,
    } = props.clone();

    // Auto-dismiss timer; re-runs only when `duration` changes.
    {
        let on_dismiss = on_dismiss.clone();
        use_effect_with(duration, move |&duration| {
            let handle = if duration > 0 {
                let timeout = Timeout::new(duration, move || {
                    if let Some(cb) = on_dismiss.as_ref() {
                        cb.emit(());
                    }
                });
                Some(timeout)
            } else {
                None
            };
            move || drop(handle)
        });
    }

    let classes: Classes = vec![
        Classes::from("sonner-toast"),
        Classes::from(format!("sonner-toast-{}", r#type.as_str())),
        Classes::from("toast"),
        Classes::from(format!("toast-{}", r#type.as_str())),
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
        <div class={classes} role={r#type.role()} aria-live={r#type.aria_live()}>
            <div class="sonner-toast-content toast-content">
                if has_children {
                    { children }
                } else {
                    <>
                        if let Some(title_text) = title {
                            <div class="sonner-toast-title toast-title">
                                { title_text }
                            </div>
                        }
                        if let Some(desc_text) = description {
                            <div class="sonner-toast-description toast-description">
                                { desc_text }
                            </div>
                        }
                    </>
                }
            </div>
            <div class="sonner-toast-actions toast-actions">
                if let Some(action_text) = action {
                    <button
                        type="button"
                        class="sonner-toast-action toast-action"
                        onclick={on_action}
                    >
                        { action_text }
                    </button>
                }
                if dismissible {
                    <button
                        type="button"
                        class="sonner-toast-close toast-close"
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
    fn aliases_are_the_toast_types() {
        let position: ToastPosition = SonnerPosition::TopCenter;
        let kind: ToastType = SonnerType::Warning;
        assert_eq!(position, ToastPosition::TopCenter);
        assert_eq!(kind, ToastType::Warning);
    }
}
