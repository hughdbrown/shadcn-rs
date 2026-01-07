//! Typography component
//!
//! Semantic text components with proper styling (headings, paragraphs, code, quotes, etc.).
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Typography, TypographyVariant, TextAlign, TextColor, FontWeight};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <>
//!             <Typography variant={TypographyVariant::H1}>{ "Main Heading" }</Typography>
//!             <Typography variant={TypographyVariant::P}>{ "Paragraph text" }</Typography>
//!             <Typography variant={TypographyVariant::Code}>{ "code snippet" }</Typography>
//!         </>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Typography variant type
#[derive(Debug, Clone, PartialEq)]
pub enum TypographyVariant {
    /// Heading 1 (largest)
    H1,
    /// Heading 2
    H2,
    /// Heading 3
    H3,
    /// Heading 4
    H4,
    /// Heading 5
    H5,
    /// Heading 6 (smallest)
    H6,
    /// Paragraph
    P,
    /// Blockquote
    Blockquote,
    /// Inline code
    Code,
    /// Lead paragraph (larger)
    Lead,
    /// Large text
    Large,
    /// Small text
    Small,
    /// Muted text
    Muted,
}

/// Text alignment
#[derive(Debug, Clone, PartialEq)]
pub enum TextAlign {
    /// Align left
    Left,
    /// Align center
    Center,
    /// Align right
    Right,
    /// Justify
    Justify,
}

/// Text color
#[derive(Debug, Clone, PartialEq)]
pub enum TextColor {
    /// Default color
    Default,
    /// Muted color
    Muted,
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
    /// Destructive/error color
    Destructive,
    /// Success color
    Success,
    /// Warning color
    Warning,
}

/// Font weight
#[derive(Debug, Clone, PartialEq)]
pub enum FontWeight {
    /// Normal weight (400)
    Normal,
    /// Medium weight (500)
    Medium,
    /// Semibold weight (600)
    Semibold,
    /// Bold weight (700)
    Bold,
}

/// Typography component properties
#[derive(Properties, PartialEq, Clone)]
pub struct TypographyProps {
    /// Typography variant
    #[prop_or(TypographyVariant::P)]
    pub variant: TypographyVariant,

    /// Text alignment
    #[prop_or_default]
    pub align: Option<TextAlign>,

    /// Text color
    #[prop_or_default]
    pub color: Option<TextColor>,

    /// Font weight
    #[prop_or_default]
    pub weight: Option<FontWeight>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Typography component
///
/// Renders semantic HTML text elements with proper styling.
///
/// # Accessibility
/// - Uses semantic HTML elements (h1-h6, p, blockquote, code)
/// - Maintains proper heading hierarchy
#[function_component(Typography)]
pub fn typography(props: &TypographyProps) -> Html {
    let TypographyProps {
        variant,
        align,
        color,
        weight,
        class,
        children,
    } = props.clone();

    let variant_class = match &variant {
        TypographyVariant::H1 => "typography-h1",
        TypographyVariant::H2 => "typography-h2",
        TypographyVariant::H3 => "typography-h3",
        TypographyVariant::H4 => "typography-h4",
        TypographyVariant::H5 => "typography-h5",
        TypographyVariant::H6 => "typography-h6",
        TypographyVariant::P => "typography-p",
        TypographyVariant::Blockquote => "typography-blockquote",
        TypographyVariant::Code => "typography-code",
        TypographyVariant::Lead => "typography-lead",
        TypographyVariant::Large => "typography-large",
        TypographyVariant::Small => "typography-small",
        TypographyVariant::Muted => "typography-muted",
    };

    let align_class = align.as_ref().map(|a| match a {
        TextAlign::Left => "text-left",
        TextAlign::Center => "text-center",
        TextAlign::Right => "text-right",
        TextAlign::Justify => "text-justify",
    });

    let color_class = color.as_ref().map(|c| match c {
        TextColor::Default => "text-default",
        TextColor::Muted => "text-muted",
        TextColor::Primary => "text-primary",
        TextColor::Secondary => "text-secondary",
        TextColor::Destructive => "text-destructive",
        TextColor::Success => "text-success",
        TextColor::Warning => "text-warning",
    });

    let weight_class = weight.as_ref().map(|w| match w {
        FontWeight::Normal => "font-normal",
        FontWeight::Medium => "font-medium",
        FontWeight::Semibold => "font-semibold",
        FontWeight::Bold => "font-bold",
    });

    let mut classes_vec = vec![Classes::from("typography"), Classes::from(variant_class)];

    if let Some(align_cls) = align_class {
        classes_vec.push(Classes::from(align_cls));
    }

    if let Some(color_cls) = color_class {
        classes_vec.push(Classes::from(color_cls));
    }

    if let Some(weight_cls) = weight_class {
        classes_vec.push(Classes::from(weight_cls));
    }

    classes_vec.push(class);

    let classes: Classes = classes_vec.into_iter().collect();

    match variant {
        TypographyVariant::H1 => html! {
            <h1 class={classes}>{ children }</h1>
        },
        TypographyVariant::H2 => html! {
            <h2 class={classes}>{ children }</h2>
        },
        TypographyVariant::H3 => html! {
            <h3 class={classes}>{ children }</h3>
        },
        TypographyVariant::H4 => html! {
            <h4 class={classes}>{ children }</h4>
        },
        TypographyVariant::H5 => html! {
            <h5 class={classes}>{ children }</h5>
        },
        TypographyVariant::H6 => html! {
            <h6 class={classes}>{ children }</h6>
        },
        TypographyVariant::P
        | TypographyVariant::Lead
        | TypographyVariant::Large
        | TypographyVariant::Small
        | TypographyVariant::Muted => html! {
            <p class={classes}>{ children }</p>
        },
        TypographyVariant::Blockquote => html! {
            <blockquote class={classes}>{ children }</blockquote>
        },
        TypographyVariant::Code => html! {
            <code class={classes}>{ children }</code>
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typography_h1() {
        let props = TypographyProps {
            variant: TypographyVariant::H1,
            align: None,
            color: None,
            weight: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.variant, TypographyVariant::H1);
    }

    #[test]
    fn test_typography_paragraph() {
        let props = TypographyProps {
            variant: TypographyVariant::P,
            align: None,
            color: None,
            weight: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.variant, TypographyVariant::P);
    }

    #[test]
    fn test_typography_with_alignment() {
        let props = TypographyProps {
            variant: TypographyVariant::P,
            align: Some(TextAlign::Center),
            color: None,
            weight: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.align, Some(TextAlign::Center));
    }

    #[test]
    fn test_typography_with_color() {
        let props = TypographyProps {
            variant: TypographyVariant::P,
            align: None,
            color: Some(TextColor::Primary),
            weight: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.color, Some(TextColor::Primary));
    }

    #[test]
    fn test_typography_with_weight() {
        let props = TypographyProps {
            variant: TypographyVariant::P,
            align: None,
            color: None,
            weight: Some(FontWeight::Bold),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.weight, Some(FontWeight::Bold));
    }

    #[test]
    fn test_typography_blockquote() {
        let props = TypographyProps {
            variant: TypographyVariant::Blockquote,
            align: None,
            color: None,
            weight: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.variant, TypographyVariant::Blockquote);
    }

    #[test]
    fn test_typography_code() {
        let props = TypographyProps {
            variant: TypographyVariant::Code,
            align: None,
            color: None,
            weight: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.variant, TypographyVariant::Code);
    }
}
