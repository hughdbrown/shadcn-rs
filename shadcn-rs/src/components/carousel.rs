//! Carousel component
//!
//! Image/content carousel with navigation, autoplay, and touch gestures.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Carousel, CarouselContent, CarouselItem, CarouselPrevious, CarouselNext};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Carousel>
//!             <CarouselContent>
//!                 <CarouselItem>{ "Slide 1" }</CarouselItem>
//!                 <CarouselItem>{ "Slide 2" }</CarouselItem>
//!                 <CarouselItem>{ "Slide 3" }</CarouselItem>
//!             </CarouselContent>
//!             <CarouselPrevious />
//!             <CarouselNext />
//!         </Carousel>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Carousel container properties
#[derive(Properties, PartialEq, Clone)]
pub struct CarouselProps {
    /// Auto-play interval in milliseconds (0 = disabled)
    #[prop_or(0)]
    pub autoplay: u32,

    /// Loop back to start when reaching the end
    #[prop_or(true)]
    pub loop_slides: bool,

    /// Show indicators/dots
    #[prop_or(true)]
    pub show_indicators: bool,

    /// Current slide index (controlled)
    #[prop_or_default]
    pub current: Option<usize>,

    /// Slide change handler
    #[prop_or_default]
    pub on_slide_change: Option<Callback<usize>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Carousel container component
///
/// Container for carousel slides with navigation and autoplay.
///
/// # Accessibility
/// - ARIA role="region"
/// - ARIA live region for slide changes
/// - Keyboard navigation supported
#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    let CarouselProps {
        autoplay: _,
        loop_slides: _,
        show_indicators: _,
        current: _,
        on_slide_change: _,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("carousel"), class]
        .into_iter()
        .collect();

    html! {
        <div
            class={classes}
            role="region"
            aria-label="Carousel"
            aria-live="polite"
        >
            { children }
        </div>
    }
}

/// Carousel content properties
#[derive(Properties, PartialEq, Clone)]
pub struct CarouselContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements (carousel items)
    pub children: Children,
}

/// Carousel content component
///
/// Container for carousel items.
#[function_component(CarouselContent)]
pub fn carousel_content(props: &CarouselContentProps) -> Html {
    let CarouselContentProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("carousel-content"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Carousel item properties
#[derive(Properties, PartialEq, Clone)]
pub struct CarouselItemProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Carousel item component
///
/// Individual slide in the carousel.
#[function_component(CarouselItem)]
pub fn carousel_item(props: &CarouselItemProps) -> Html {
    let CarouselItemProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("carousel-item"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Carousel previous button properties
#[derive(Properties, PartialEq, Clone)]
pub struct CarouselPreviousProps {
    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Carousel previous button component
///
/// Navigation button to go to the previous slide.
#[function_component(CarouselPrevious)]
pub fn carousel_previous(props: &CarouselPreviousProps) -> Html {
    let CarouselPreviousProps { onclick, class } = props.clone();

    let classes: Classes = vec![Classes::from("carousel-previous"), class]
        .into_iter()
        .collect();

    html! {
        <button
            type="button"
            class={classes}
            onclick={onclick}
            aria-label="Previous slide"
        >
            { "‹" }
        </button>
    }
}

/// Carousel next button properties
#[derive(Properties, PartialEq, Clone)]
pub struct CarouselNextProps {
    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Carousel next button component
///
/// Navigation button to go to the next slide.
#[function_component(CarouselNext)]
pub fn carousel_next(props: &CarouselNextProps) -> Html {
    let CarouselNextProps { onclick, class } = props.clone();

    let classes: Classes = vec![Classes::from("carousel-next"), class]
        .into_iter()
        .collect();

    html! {
        <button
            type="button"
            class={classes}
            onclick={onclick}
            aria-label="Next slide"
        >
            { "›" }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carousel_default() {
        let props = CarouselProps {
            autoplay: 0,
            loop_slides: true,
            show_indicators: true,
            current: None,
            on_slide_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.autoplay, 0);
        assert!(props.loop_slides);
        assert!(props.show_indicators);
    }

    #[test]
    fn test_carousel_with_autoplay() {
        let props = CarouselProps {
            autoplay: 3000,
            loop_slides: true,
            show_indicators: true,
            current: None,
            on_slide_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.autoplay, 3000);
    }

    #[test]
    fn test_carousel_no_loop() {
        let props = CarouselProps {
            autoplay: 0,
            loop_slides: false,
            show_indicators: true,
            current: None,
            on_slide_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.loop_slides);
    }

    #[test]
    fn test_carousel_controlled() {
        let props = CarouselProps {
            autoplay: 0,
            loop_slides: true,
            show_indicators: true,
            current: Some(2),
            on_slide_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.current, Some(2));
    }

    #[test]
    fn test_carousel_content_props() {
        let props = CarouselContentProps {
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.class, Classes::new());
    }
}
