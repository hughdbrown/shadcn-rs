//! Bubble component
//!
//! Displays conversational content in a message bubble. Supports variants,
//! alignment, grouping, reactions, and collapsible content.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Bubble, BubbleContent, BubbleReactions, BubbleVariant, BubbleAlign};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Bubble variant={BubbleVariant::Default} align={BubbleAlign::Start}>
//!             <BubbleContent>
//!                 { "Hello! How can I help you today?" }
//!             </BubbleContent>
//!             <BubbleReactions aria_label="Reactions">
//!                 <span>{ "👍" }</span>
//!             </BubbleReactions>
//!         </Bubble>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Visual variants for conversational bubbles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BubbleVariant {
    /// Default surface style
    #[default]
    Default,
    /// Secondary accent style
    Secondary,
    /// Muted background style
    Muted,
    /// Tinted accent style
    Tinted,
    /// Bordered outline style
    Outline,
    /// Borderless ghost style (removes max-width constraint)
    Ghost,
    /// Destructive error style
    Destructive,
}

impl BubbleVariant {
    /// Convert variant to CSS class name
    pub fn to_class(self) -> &'static str {
        match self {
            BubbleVariant::Default => "bubble-variant-default",
            BubbleVariant::Secondary => "bubble-variant-secondary",
            BubbleVariant::Muted => "bubble-variant-muted",
            BubbleVariant::Tinted => "bubble-variant-tinted",
            BubbleVariant::Outline => "bubble-variant-outline",
            BubbleVariant::Ghost => "bubble-variant-ghost",
            BubbleVariant::Destructive => "bubble-variant-destructive",
        }
    }
}

/// Alignment of the bubble in a conversation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BubbleAlign {
    /// Align to the start (typically assistant/incoming)
    #[default]
    Start,
    /// Align to the end (typically user/outgoing)
    End,
}

impl BubbleAlign {
    /// Convert alignment to CSS class name
    pub fn to_class(self) -> &'static str {
        match self {
            BubbleAlign::Start => "bubble-align-start",
            BubbleAlign::End => "bubble-align-end",
        }
    }
}

/// Properties for [`Bubble`]
#[derive(Properties, PartialEq, Clone)]
pub struct BubbleProps {
    /// Visual style variant
    #[prop_or_default]
    pub variant: BubbleVariant,

    /// Alignment within row
    #[prop_or_default]
    pub align: BubbleAlign,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Bubble content and subcomponents
    pub children: Children,
}

/// Conversational bubble surface
#[function_component(Bubble)]
pub fn bubble(props: &BubbleProps) -> Html {
    let classes = classes!(
        "bubble",
        props.variant.to_class(),
        props.align.to_class(),
        props.class.clone()
    );

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`BubbleContent`]
#[derive(Properties, PartialEq, Clone)]
pub struct BubbleContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Text or message nodes
    pub children: Children,
}

/// Inner content wrapper for a bubble
#[function_component(BubbleContent)]
pub fn bubble_content(props: &BubbleContentProps) -> Html {
    let classes = classes!("bubble-content", props.class.clone());

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`BubbleReactions`]
#[derive(Properties, PartialEq, Clone)]
pub struct BubbleReactionsProps {
    /// Optional ARIA role (default: "toolbar" or "img")
    #[prop_or(AttrValue::from("toolbar"))]
    pub role: AttrValue,

    /// Accessible label
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Reactions icons, badges, or buttons
    pub children: Children,
}

/// Container for reaction badges or action triggers attached to a bubble
#[function_component(BubbleReactions)]
pub fn bubble_reactions(props: &BubbleReactionsProps) -> Html {
    let classes = classes!("bubble-reactions", props.class.clone());

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

/// Properties for [`BubbleGroup`]
#[derive(Properties, PartialEq, Clone)]
pub struct BubbleGroupProps {
    /// Alignment of bubbles within group
    #[prop_or_default]
    pub align: BubbleAlign,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Stacked bubbles
    pub children: Children,
}

/// Group container for stacking consecutive bubbles
#[function_component(BubbleGroup)]
pub fn bubble_group(props: &BubbleGroupProps) -> Html {
    let classes = classes!(
        "bubble-group",
        match props.align {
            BubbleAlign::Start => "bubble-group-align-start",
            BubbleAlign::End => "bubble-group-align-end",
        },
        props.class.clone()
    );

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_classes_have_css() {
        let css = include_str!("../../styles/components.css");
        for class in [
            "bubble",
            "bubble-variant-default",
            "bubble-variant-secondary",
            "bubble-variant-muted",
            "bubble-variant-tinted",
            "bubble-variant-outline",
            "bubble-variant-ghost",
            "bubble-variant-destructive",
            "bubble-align-start",
            "bubble-align-end",
            "bubble-content",
            "bubble-reactions",
            "bubble-group",
            "bubble-group-align-start",
            "bubble-group-align-end",
        ] {
            assert!(css.contains(&format!(".{class} {{")), "missing .{class}");
        }
    }

    #[test]
    fn test_bubble_variant_to_class() {
        assert_eq!(BubbleVariant::Default.to_class(), "bubble-variant-default");
        assert_eq!(
            BubbleVariant::Secondary.to_class(),
            "bubble-variant-secondary"
        );
        assert_eq!(BubbleVariant::Muted.to_class(), "bubble-variant-muted");
        assert_eq!(BubbleVariant::Tinted.to_class(), "bubble-variant-tinted");
        assert_eq!(BubbleVariant::Outline.to_class(), "bubble-variant-outline");
        assert_eq!(BubbleVariant::Ghost.to_class(), "bubble-variant-ghost");
        assert_eq!(
            BubbleVariant::Destructive.to_class(),
            "bubble-variant-destructive"
        );
    }

    #[test]
    fn test_bubble_align_to_class() {
        assert_eq!(BubbleAlign::Start.to_class(), "bubble-align-start");
        assert_eq!(BubbleAlign::End.to_class(), "bubble-align-end");
    }
}
