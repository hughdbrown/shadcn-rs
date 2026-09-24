//! Carousel component showcase page

use shadcn_rs::{
    Card, CardContent, Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious,
};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(CarouselPage)]
pub fn carousel_page() -> Html {
    let controlled_index = use_state(|| 1usize);
    let on_slide_change = {
        let controlled_index = controlled_index.clone();
        Callback::from(move |index: usize| controlled_index.set(index))
    };

    let slides = ["Alpha", "Beta", "Gamma", "Delta"];

    let examples = vec![
        Example {
            title: "Navigation And Indicators",
            description: "The carousel now owns slide state, previous/next controls, keyboard navigation, and indicator buttons.",
            demo: html! {
                <Carousel class="w-full max-w-xl">
                    <CarouselContent>
                        { for slides.iter().map(|label| html! {
                            <CarouselItem>
                                <Card>
                                    <CardContent class="flex aspect-[16/7] items-center justify-center p-6">
                                        <span class="text-3xl font-semibold">{ label }</span>
                                    </CardContent>
                                </Card>
                            </CarouselItem>
                        })}
                    </CarouselContent>
                    <CarouselPrevious />
                    <CarouselNext />
                </Carousel>
            },
            code: r#"<Carousel>
    <CarouselContent>
        <CarouselItem>{ "Alpha" }</CarouselItem>
        <CarouselItem>{ "Beta" }</CarouselItem>
        <CarouselItem>{ "Gamma" }</CarouselItem>
    </CarouselContent>
    <CarouselPrevious />
    <CarouselNext />
</Carousel>"#,
        },
        Example {
            title: "Controlled And Autoplay",
            description: "Use `current`, `on_slide_change`, and `autoplay` when the surrounding UI needs to react to slide state.",
            demo: html! {
                <div class="space-y-4">
                    <p class="text-sm text-muted-foreground">
                        { format!("Controlled slide index: {}", *controlled_index) }
                    </p>
                    <Carousel
                        class="w-full max-w-xl"
                        current={Some(*controlled_index)}
                        on_slide_change={Some(on_slide_change.clone())}
                        autoplay={2500}
                        loop_slides={true}
                    >
                        <CarouselContent>
                            { for slides.iter().enumerate().map(|(index, label)| html! {
                                <CarouselItem>
                                    <Card>
                                        <CardContent class="flex aspect-[16/7] flex-col items-center justify-center gap-2 p-6">
                                            <span class="text-sm uppercase tracking-[0.2em] text-muted-foreground">{ format!("Slide {}", index + 1) }</span>
                                            <span class="text-3xl font-semibold">{ label }</span>
                                        </CardContent>
                                    </Card>
                                </CarouselItem>
                            })}
                        </CarouselContent>
                        <CarouselPrevious />
                        <CarouselNext />
                    </Carousel>
                </div>
            },
            code: r#"<Carousel
    current={Some(*controlled_index)}
    on_slide_change={Some(on_slide_change)}
    autoplay={2500}
    loop_slides={true}
>
    ...
</Carousel>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "autoplay",
            prop_type: "u32",
            default: "0",
            description: "Auto-advance interval in milliseconds. `0` disables autoplay.",
        },
        PropDoc {
            name: "loop_slides",
            prop_type: "bool",
            default: "true",
            description: "Wraps navigation back to the first slide when the carousel reaches the end.",
        },
        PropDoc {
            name: "show_indicators",
            prop_type: "bool",
            default: "true",
            description: "Shows clickable indicator buttons below the viewport.",
        },
        PropDoc {
            name: "current",
            prop_type: "Option<usize>",
            default: "None",
            description: "Controlled current slide index.",
        },
        PropDoc {
            name: "on_slide_change",
            prop_type: "Option<Callback<usize>>",
            default: "None",
            description: "Receives the next slide index whenever navigation changes it.",
        },
    ];

    html! {
        <ComponentPage
            name="Carousel"
            description="A stateful carousel with internal navigation, indicators, keyboard controls, and optional autoplay."
            {examples}
            {props}
        />
    }
}
