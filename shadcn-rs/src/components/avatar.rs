//! Avatar component
//!
//! User profile picture or initials display with fallback support.
//!
//! # Examples
//!
//! Props-based usage:
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
//!
//! Compound component usage:
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Avatar, AvatarImage, AvatarFallback};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Avatar>
//!             <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
//!             <AvatarFallback>{ "CN" }</AvatarFallback>
//!         </Avatar>
//!     }
//! }
//! ```

use crate::types::Size;
use yew::prelude::*;

/// Avatar shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarShape {
    /// Circular avatar
    #[default]
    Circle,
    /// Square avatar
    Square,
}

impl AvatarShape {
    /// CSS class for this shape (`.avatar.shape-*` in the stylesheet)
    pub fn to_class(self) -> &'static str {
        match self {
            AvatarShape::Circle => "shape-circle",
            AvatarShape::Square => "shape-square",
        }
    }
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
    #[prop_or_default]
    pub shape: AvatarShape,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements (for compound AvatarImage / AvatarFallback usage)
    #[prop_or_default]
    pub children: Children,
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
        children,
    } = props.clone();

    let image_error = use_state(|| false);

    // These must match the `.avatar.size-*` / `.avatar.shape-*` rules in styles/components.css.
    let size_class = size.to_class();
    let shape_class = shape.to_class();

    let classes = classes!("avatar", size_class, shape_class, class);

    if children.iter().count() > 0 {
        let aria_label = alt.unwrap_or_else(|| AttrValue::from("Avatar"));
        return html! {
            <div class={classes} role="img" aria-label={aria_label}>
                { children }
            </div>
        };
    }

    let on_error = {
        let image_error = image_error.clone();
        Callback::from(move |_: Event| {
            image_error.set(true);
        })
    };

    let show_image = src.is_some() && !*image_error;
    let show_fallback = !show_image && initials.is_none();
    let aria_label_val = alt.clone().unwrap_or_else(|| AttrValue::from("Avatar"));

    html! {
        <div class={classes} role="img" aria-label={aria_label_val}>
            if show_image {
                <img
                    class="avatar-image"
                    src={src}
                    alt={alt.unwrap_or_else(|| AttrValue::from("Avatar"))}
                    onerror={on_error}
                />
            } else if let Some(initials_text) = initials {
                <span class="avatar-initials" aria-hidden="true">
                    { initials_text }
                </span>
            } else if show_fallback {
                <span class="avatar-fallback" aria-hidden="true">
                    { fallback_icon }
                </span>
            }
        </div>
    }
}

/// Properties for [`AvatarImage`]
#[derive(Properties, PartialEq, Clone)]
pub struct AvatarImageProps {
    /// Image source URL
    pub src: AttrValue,

    /// Alt text for image
    #[prop_or_default]
    pub alt: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Image component for compound [`Avatar`]
#[function_component(AvatarImage)]
pub fn avatar_image(props: &AvatarImageProps) -> Html {
    let classes = classes!("avatar-image", props.class.clone());
    let alt_val = props
        .alt
        .clone()
        .unwrap_or_else(|| AttrValue::from("Avatar"));

    html! {
        <img
            class={classes}
            src={props.src.clone()}
            alt={alt_val}
        />
    }
}

/// Properties for [`AvatarFallback`]
#[derive(Properties, PartialEq, Clone)]
pub struct AvatarFallbackProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Fallback content (initials, icon, etc.)
    pub children: Children,
}

/// Fallback component for compound [`Avatar`]
#[function_component(AvatarFallback)]
pub fn avatar_fallback(props: &AvatarFallbackProps) -> Html {
    let classes = classes!("avatar-fallback", props.class.clone());

    html! {
        <span class={classes} aria-hidden="true">
            { props.children.clone() }
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_classes_match_stylesheet() {
        // styles/components.css defines `.avatar.size-*` and `.avatar.shape-*`.
        let css = include_str!("../../styles/components.css");
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl, Size::Xl2] {
            let selector = format!(".avatar.{} {{", size.to_class());
            assert!(css.contains(&selector), "missing CSS rule {selector}");
        }
        for shape in [AvatarShape::Circle, AvatarShape::Square] {
            let selector = format!(".avatar.{} {{", shape.to_class());
            assert!(css.contains(&selector), "missing CSS rule {selector}");
        }
        assert!(css.contains(".avatar-initials {"));
        assert!(css.contains(".avatar-image {"));
        assert!(css.contains(".avatar-fallback {"));
    }

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
            children: Children::new(vec![]),
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
            children: Children::new(vec![]),
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
            children: Children::new(vec![]),
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
            children: Children::new(vec![]),
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
            children: Children::new(vec![]),
        };

        assert_eq!(props.shape, AvatarShape::Square);
    }
}
