//! Carousel component
//!
//! Image/content carousel with navigation, autoplay, and indicators.

use gloo::timers::callback::Interval;
use yew::prelude::*;

/// Shared carousel state
#[derive(Clone, PartialEq)]
pub struct CarouselContext {
    /// Current slide index
    pub current: usize,
    /// Total slide count
    pub count: usize,
    /// Whether slides wrap
    pub loop_slides: bool,
    /// Navigate to the previous slide
    pub previous: Callback<()>,
    /// Navigate to the next slide
    pub next: Callback<()>,
    /// Navigate to a specific slide
    pub go_to: Callback<usize>,
    /// Lets `CarouselContent` report how many slides it holds
    pub(crate) set_count: Callback<usize>,
}

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

fn clamp_index(index: usize, count: usize, loop_slides: bool) -> usize {
    if count == 0 {
        return 0;
    }
    if loop_slides {
        index % count
    } else {
        index.min(count - 1)
    }
}

/// Carousel container component
#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    // Slides live inside `CarouselContent`, not directly under `Carousel`
    // (which also holds the previous/next buttons), so the content reports
    // its own child count back through context.
    let slide_count_state = use_state_eq(|| 0usize);
    let slide_count = *slide_count_state;
    let internal_current = use_state(|| props.current.unwrap_or(0));
    let requested = props.current.unwrap_or(*internal_current);
    // Until the content has reported its size, keep the requested index
    // rather than snapping to 0 for one frame.
    let current = if slide_count == 0 {
        requested
    } else {
        clamp_index(requested, slide_count, props.loop_slides)
    };

    let set_current = {
        let internal_current = internal_current.clone();
        let on_slide_change = props.on_slide_change.clone();
        let externally_controlled = props.current.is_some();
        let loop_slides = props.loop_slides;
        Callback::from(move |index: usize| {
            let next = clamp_index(index, slide_count, loop_slides);
            if !externally_controlled {
                internal_current.set(next);
            }
            if let Some(callback) = on_slide_change.as_ref() {
                callback.emit(next);
            }
        })
    };

    {
        let set_current = set_current.clone();
        let autoplay = props.autoplay;
        let loop_slides = props.loop_slides;
        use_effect_with((autoplay, slide_count, current, loop_slides), move |_| {
            // Without looping, autoplay stops on the last slide instead of
            // re-emitting the same index on every tick.
            let can_advance = loop_slides || current + 1 < slide_count;
            let handle = if autoplay > 0 && slide_count > 1 && can_advance {
                Some(Interval::new(autoplay, move || {
                    set_current.emit((current + 1) % slide_count);
                }))
            } else {
                None
            };
            move || drop(handle)
        });
    }

    let previous = {
        let set_current = set_current.clone();
        let loop_slides = props.loop_slides;
        Callback::from(move |_: ()| {
            if slide_count == 0 {
                return;
            }
            let next = if current == 0 {
                if loop_slides { slide_count - 1 } else { 0 }
            } else {
                current - 1
            };
            set_current.emit(next);
        })
    };

    let next = {
        let set_current = set_current.clone();
        let loop_slides = props.loop_slides;
        Callback::from(move |_: ()| {
            if slide_count == 0 {
                return;
            }
            let next = if current + 1 >= slide_count {
                if loop_slides { 0 } else { slide_count - 1 }
            } else {
                current + 1
            };
            set_current.emit(next);
        })
    };

    let go_to = {
        let set_current = set_current.clone();
        Callback::from(move |index: usize| set_current.emit(index))
    };

    let onkeydown = {
        let previous = previous.clone();
        let next = next.clone();
        Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
            "ArrowLeft" => previous.emit(()),
            "ArrowRight" => next.emit(()),
            _ => {}
        })
    };

    let classes: Classes = vec![
        Classes::from("carousel"),
        Classes::from("orientation-horizontal"),
        props.class.clone(),
    ]
    .into_iter()
    .collect();

    let context = CarouselContext {
        current,
        count: slide_count,
        loop_slides: props.loop_slides,
        previous: previous.clone(),
        next: next.clone(),
        go_to: go_to.clone(),
        set_count: Callback::from(move |count: usize| slide_count_state.set(count)),
    };

    html! {
        <ContextProvider<CarouselContext> context={context}>
            <div
                class={classes}
                role="region"
                aria-label="Carousel"
                aria-live="polite"
                tabindex="0"
                onkeydown={onkeydown}
            >
                { for props.children.iter() }
                if props.show_indicators && slide_count > 1 {
                    <div class="carousel-indicators" role="tablist" aria-label="Carousel slide selectors">
                        {
                            (0..slide_count).map(|index| {
                                let is_active = index == current;
                                html! {
                                    <button
                                        key={index}
                                        type="button"
                                        class={classes!("carousel-indicator", is_active.then_some("carousel-indicator-active"))}
                                        aria-label={format!("Go to slide {}", index + 1)}
                                        aria-selected={is_active.to_string()}
                                        onclick={{
                                            let go_to = go_to.clone();
                                            Callback::from(move |_: MouseEvent| go_to.emit(index))
                                        }}
                                    />
                                }
                            }).collect::<Html>()
                        }
                    </div>
                }
            </div>
        </ContextProvider<CarouselContext>>
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
#[function_component(CarouselContent)]
pub fn carousel_content(props: &CarouselContentProps) -> Html {
    let context = use_context::<CarouselContext>();
    let current = context.as_ref().map(|ctx| ctx.current).unwrap_or(0);
    let child_count = props.children.len();
    {
        let set_count = context.as_ref().map(|ctx| ctx.set_count.clone());
        use_effect_with(child_count, move |count| {
            if let Some(set_count) = set_count {
                set_count.emit(*count);
            }
        });
    }
    let style = format!(
        "transform: translateX(-{}%); transition: transform 0.3s ease;",
        current * 100
    );
    let classes: Classes = vec![Classes::from("carousel-content"), props.class.clone()]
        .into_iter()
        .collect();

    html! {
        <div class="carousel-viewport">
            <div class={classes} style={style}>
                { for props.children.iter() }
            </div>
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
#[function_component(CarouselItem)]
pub fn carousel_item(props: &CarouselItemProps) -> Html {
    let classes: Classes = vec![Classes::from("carousel-item"), props.class.clone()]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { for props.children.iter() }
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
#[function_component(CarouselPrevious)]
pub fn carousel_previous(props: &CarouselPreviousProps) -> Html {
    let context = use_context::<CarouselContext>();
    let is_disabled = context
        .as_ref()
        .is_some_and(|ctx| !ctx.loop_slides && ctx.current == 0);
    let onclick = if let Some(handler) = props.onclick.clone() {
        handler
    } else {
        let previous = context.as_ref().map(|ctx| ctx.previous.clone());
        Callback::from(move |_: MouseEvent| {
            if let Some(previous) = previous.as_ref() {
                previous.emit(());
            }
        })
    };

    let classes: Classes = vec![Classes::from("carousel-previous"), props.class.clone()]
        .into_iter()
        .collect();

    html! {
        <button
            type="button"
            class={classes}
            onclick={onclick}
            aria-label="Previous slide"
            disabled={is_disabled}
        >
            { "<" }
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
#[function_component(CarouselNext)]
pub fn carousel_next(props: &CarouselNextProps) -> Html {
    let context = use_context::<CarouselContext>();
    let is_disabled = context
        .as_ref()
        .is_some_and(|ctx| !ctx.loop_slides && ctx.count > 0 && ctx.current + 1 >= ctx.count);
    let onclick = if let Some(handler) = props.onclick.clone() {
        handler
    } else {
        let next = context.as_ref().map(|ctx| ctx.next.clone());
        Callback::from(move |_: MouseEvent| {
            if let Some(next) = next.as_ref() {
                next.emit(());
            }
        })
    };

    let classes: Classes = vec![Classes::from("carousel-next"), props.class.clone()]
        .into_iter()
        .collect();

    html! {
        <button
            type="button"
            class={classes}
            onclick={onclick}
            aria-label="Next slide"
            disabled={is_disabled}
        >
            { ">" }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_index_looping() {
        assert_eq!(clamp_index(4, 3, true), 1);
    }

    #[test]
    fn test_clamp_index_non_looping() {
        assert_eq!(clamp_index(4, 3, false), 2);
    }

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
