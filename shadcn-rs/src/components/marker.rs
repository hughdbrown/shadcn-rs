//! Marker component
//!
//! Displays an inline status, system note, bordered row, or labeled separator in a conversation.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Marker, MarkerIcon, MarkerContent, MarkerVariant};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Marker variant={MarkerVariant::Default}>
//!             <MarkerIcon><span>{ "✓" }</span></MarkerIcon>
//!             <MarkerContent>{ "Explored 4 files" }</MarkerContent>
//!         </Marker>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Style variants for the Marker component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkerVariant {
    /// Default inline chip style
    #[default]
    Default,
    /// Subtle muted background style
    Subtle,
    /// Bordered card/callout style
    Border,
    /// Horizontal divider separator with label
    Separator,
}

impl MarkerVariant {
    /// Convert variant to CSS class name
    pub fn to_class(self) -> &'static str {
        match self {
            MarkerVariant::Default => "marker-variant-default",
            MarkerVariant::Subtle => "marker-variant-subtle",
            MarkerVariant::Border => "marker-variant-border",
            MarkerVariant::Separator => "marker-variant-separator",
        }
    }
}

/// Properties for [`Marker`]
#[derive(Properties, PartialEq, Clone)]
pub struct MarkerProps {
    /// Style variant
    #[prop_or_default]
    pub variant: MarkerVariant,

    /// Whether to display animated shimmer effect (e.g. while processing)
    #[prop_or(false)]
    pub shimmer: bool,

    /// Optional ARIA role (e.g. "status", "note")
    #[prop_or_default]
    pub role: Option<AttrValue>,

    /// Accessible label
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Inline status, system event, or separator in a conversation
#[function_component(Marker)]
pub fn marker(props: &MarkerProps) -> Html {
    let classes = classes!(
        "marker",
        props.variant.to_class(),
        if props.shimmer { "marker-shimmer" } else { "" },
        props.class.clone()
    );

    html! {
        <div
            class={classes}
            role={props.role.clone()}
            aria-label={props.aria_label.clone()}
        >
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`MarkerIcon`]
#[derive(Properties, PartialEq, Clone)]
pub struct MarkerIconProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Icon element
    pub children: Children,
}

/// Leading icon inside a marker
#[function_component(MarkerIcon)]
pub fn marker_icon(props: &MarkerIconProps) -> Html {
    let classes = classes!("marker-icon", props.class.clone());

    html! {
        <span class={classes} aria-hidden="true">
            { props.children.clone() }
        </span>
    }
}

/// Properties for [`MarkerContent`]
#[derive(Properties, PartialEq, Clone)]
pub struct MarkerContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Text or label content
    pub children: Children,
}

/// Content or label inside a marker
#[function_component(MarkerContent)]
pub fn marker_content(props: &MarkerContentProps) -> Html {
    let classes = classes!("marker-content", props.class.clone());

    html! {
        <span class={classes}>
            { props.children.clone() }
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_classes_have_css() {
        let css = include_str!("../../styles/components.css");
        for class in [
            "marker",
            "marker-variant-default",
            "marker-variant-subtle",
            "marker-variant-border",
            "marker-variant-separator",
            "marker-shimmer",
            "marker-icon",
            "marker-content",
        ] {
            assert!(css.contains(&format!(".{class} {{")), "missing .{class}");
        }
    }

    #[test]
    fn test_marker_variant_to_class() {
        assert_eq!(MarkerVariant::Default.to_class(), "marker-variant-default");
        assert_eq!(MarkerVariant::Subtle.to_class(), "marker-variant-subtle");
        assert_eq!(MarkerVariant::Border.to_class(), "marker-variant-border");
        assert_eq!(
            MarkerVariant::Separator.to_class(),
            "marker-variant-separator"
        );
    }
}
