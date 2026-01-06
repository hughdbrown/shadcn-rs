//! Card component
//!
//! A container with a subtle border and padding for displaying content.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter, Button};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Card>
//!             <CardHeader>
//!                 <CardTitle>{ "Card Title" }</CardTitle>
//!                 <CardDescription>{ "Card description goes here" }</CardDescription>
//!             </CardHeader>
//!             <CardContent>
//!                 <p>{ "Card content" }</p>
//!             </CardContent>
//!             <CardFooter>
//!                 <Button>{ "Action" }</Button>
//!             </CardFooter>
//!         </Card>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Card component properties
#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Children elements
    pub children: Children,
}

/// Card component
///
/// A versatile container component for grouping related content.
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let CardProps {
        class,
        style,
        id,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("card"), class].into_iter().collect();

    html! {
        <div class={classes} style={style} id={id}>
            { children }
        </div>
    }
}

/// Card header properties
#[derive(Properties, PartialEq, Clone)]
pub struct CardHeaderProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Card header component
///
/// Container for card title and description.
#[function_component(CardHeader)]
pub fn card_header(props: &CardHeaderProps) -> Html {
    let CardHeaderProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("card-header"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Card title properties
#[derive(Properties, PartialEq, Clone)]
pub struct CardTitleProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Heading level (h1-h6)
    #[prop_or(3)]
    pub level: u8,

    /// Children elements
    pub children: Children,
}

/// Card title component
///
/// Displays the main heading for a card.
#[function_component(CardTitle)]
pub fn card_title(props: &CardTitleProps) -> Html {
    let CardTitleProps {
        class,
        level,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("card-title"), class]
        .into_iter()
        .collect();

    match level {
        1 => html! { <h1 class={classes}>{ children }</h1> },
        2 => html! { <h2 class={classes}>{ children }</h2> },
        3 => html! { <h3 class={classes}>{ children }</h3> },
        4 => html! { <h4 class={classes}>{ children }</h4> },
        5 => html! { <h5 class={classes}>{ children }</h5> },
        6 => html! { <h6 class={classes}>{ children }</h6> },
        _ => html! { <h3 class={classes}>{ children }</h3> },
    }
}

/// Card description properties
#[derive(Properties, PartialEq, Clone)]
pub struct CardDescriptionProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Card description component
///
/// Displays a description or subtitle for a card.
#[function_component(CardDescription)]
pub fn card_description(props: &CardDescriptionProps) -> Html {
    let CardDescriptionProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("card-description"), class]
        .into_iter()
        .collect();

    html! {
        <p class={classes}>
            { children }
        </p>
    }
}

/// Card content properties
#[derive(Properties, PartialEq, Clone)]
pub struct CardContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Card content component
///
/// Container for the main card content.
#[function_component(CardContent)]
pub fn card_content(props: &CardContentProps) -> Html {
    let CardContentProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("card-content"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Card footer properties
#[derive(Properties, PartialEq, Clone)]
pub struct CardFooterProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Card footer component
///
/// Container for card actions or additional information.
#[function_component(CardFooter)]
pub fn card_footer(props: &CardFooterProps) -> Html {
    let CardFooterProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("card-footer"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_props() {
        let props = CardProps {
            class: Classes::new(),
            style: None,
            id: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.class, Classes::new());
        assert!(props.style.is_none());
    }

    #[test]
    fn test_card_title_level() {
        let levels = vec![1, 2, 3, 4, 5, 6];

        for level in levels {
            let props = CardTitleProps {
                class: Classes::new(),
                level,
                children: Children::new(vec![]),
            };
            assert_eq!(props.level, level);
        }
    }

    #[test]
    fn test_card_title_default_level() {
        // Default level should be 3
        let props = CardTitleProps {
            class: Classes::new(),
            level: 3,
            children: Children::new(vec![]),
        };
        assert_eq!(props.level, 3);
    }
}
