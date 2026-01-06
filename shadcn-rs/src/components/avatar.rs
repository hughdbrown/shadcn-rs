//! Avatar component
//!
//! User profile picture or initials display with fallback support.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Avatar, AvatarShape};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <>
//!             <Avatar src="https://example.com/avatar.jpg" alt="John Doe" />
//!             <Avatar initials="JD" />
//!             <Avatar fallback_icon="👤" />
//!         </>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::types::Size;

/// Avatar shape
#[derive(Debug, Clone, PartialEq)]
pub enum AvatarShape {
    /// Circular avatar
    Circle,
    /// Square avatar
    Square,
}

/// Avatar component properties
#[derive(Properties, PartialEq, Clone)]
pub struct AvatarProps {
    /// Image source URL
    #[prop_or_default]
    pub src: Option<AttrValue>,

    /// Alt text for image
    #[prop_or_default]
    pub alt: Option<AttrValue>,

    /// Fallback initials (shown if image fails to load)
    #[prop_or_default]
    pub initials: Option<AttrValue>,

    /// Fallback icon (shown if image fails and no initials)
    #[prop_or(AttrValue::from("👤"))]
    pub fallback_icon: AttrValue,

    /// Size of the avatar
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Shape of the avatar
    #[prop_or(AvatarShape::Circle)]
    pub shape: AvatarShape,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Avatar component
///
/// Displays a user avatar with image, initials, or icon fallback.
///
/// # Accessibility
/// - Includes alt text for images
/// - Uses `aria-label` for non-image avatars
/// - Decorative fallback icon marked with `aria-hidden`
#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let AvatarProps {
        src,
        alt,
        initials,
        fallback_icon,
        size,
        shape,
        class,
    } = props.clone();

    let image_error = use_state(|| false);

    let size_class = match size {
        Size::Xs => "avatar-xs",
        Size::Sm => "avatar-sm",
        Size::Md => "avatar-md",
        Size::Lg => "avatar-lg",
        Size::Xl => "avatar-xl",
        Size::Xl2 => "avatar-2xl",
    };

    let shape_class = match shape {
        AvatarShape::Circle => "avatar-circle",
        AvatarShape::Square => "avatar-square",
    };

    let classes: Classes = vec![
        Classes::from("avatar"),
        Classes::from(size_class),
        Classes::from(shape_class),
        class,
    ]
    .into_iter()
    .collect();

    let on_error = {
        let image_error = image_error.clone();
        Callback::from(move |_: Event| {
            image_error.set(true);
        })
    };

    let show_image = src.is_some() && !*image_error;
    let show_initials = !show_image && initials.is_some();
    let show_fallback = !show_image && initials.is_none();

    html! {
        <div class={classes} role="img" aria-label={alt.clone().or_else(|| Some(AttrValue::from("Avatar")))}>
            if show_image {
                <img
                    class="avatar-image"
                    src={src.clone()}
                    alt={alt.unwrap_or_else(|| AttrValue::from("Avatar"))}
                    onerror={on_error}
                />
            } else if show_initials {
                <span class="avatar-initials" aria-hidden="true">
                    { initials.unwrap() }
                </span>
            } else if show_fallback {
                <span class="avatar-fallback" aria-hidden="true">
                    { fallback_icon }
                </span>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_with_image() {
        let props = AvatarProps {
            src: Some(AttrValue::from("https://example.com/avatar.jpg")),
            alt: Some(AttrValue::from("User")),
            initials: None,
            fallback_icon: AttrValue::from("👤"),
            size: Size::Md,
            shape: AvatarShape::Circle,
            class: Classes::new(),
        };

        assert_eq!(
            props.src,
            Some(AttrValue::from("https://example.com/avatar.jpg"))
        );
        assert_eq!(props.alt, Some(AttrValue::from("User")));
    }

    #[test]
    fn test_avatar_with_initials() {
        let props = AvatarProps {
            src: None,
            alt: None,
            initials: Some(AttrValue::from("JD")),
            fallback_icon: AttrValue::from("👤"),
            size: Size::Md,
            shape: AvatarShape::Circle,
            class: Classes::new(),
        };

        assert_eq!(props.initials, Some(AttrValue::from("JD")));
    }

    #[test]
    fn test_avatar_with_fallback_icon() {
        let props = AvatarProps {
            src: None,
            alt: None,
            initials: None,
            fallback_icon: AttrValue::from("🙂"),
            size: Size::Md,
            shape: AvatarShape::Circle,
            class: Classes::new(),
        };

        assert_eq!(props.fallback_icon, AttrValue::from("🙂"));
    }

    #[test]
    fn test_avatar_size_large() {
        let props = AvatarProps {
            src: None,
            alt: None,
            initials: Some(AttrValue::from("AB")),
            fallback_icon: AttrValue::from("👤"),
            size: Size::Lg,
            shape: AvatarShape::Circle,
            class: Classes::new(),
        };

        assert_eq!(props.size, Size::Lg);
    }

    #[test]
    fn test_avatar_shape_square() {
        let props = AvatarProps {
            src: None,
            alt: None,
            initials: Some(AttrValue::from("CD")),
            fallback_icon: AttrValue::from("👤"),
            size: Size::Md,
            shape: AvatarShape::Square,
            class: Classes::new(),
        };

        assert_eq!(props.shape, AvatarShape::Square);
    }
}
