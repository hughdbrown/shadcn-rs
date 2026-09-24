//! shadcn-icons: generated Lucide-style icons for Rust/Yew.

#![warn(missing_docs)]

use yew::prelude::*;

/// Properties shared by icon components.
#[derive(Properties, PartialEq, Clone)]
pub struct IconProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// Icon width
    #[prop_or(24)]
    pub width: u32,

    /// Icon height
    #[prop_or(24)]
    pub height: u32,

    /// Stroke width
    #[prop_or(2)]
    pub stroke_width: u32,

    /// Icon color
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
}

mod generated;

pub use generated::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_props_default() {
        let props = IconProps {
            class: Classes::new(),
            style: None,
            width: 24,
            height: 24,
            stroke_width: 2,
            color: AttrValue::from("currentColor"),
        };

        assert_eq!(props.width, 24);
        assert_eq!(props.height, 24);
    }
}
