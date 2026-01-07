//! Carousel component showcase page

use shadcn_rs::{
    Card, CardContent, Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious,
};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(CarouselPage)]
pub fn carousel_page() -> Html {
    let examples = vec![Example {
        title: "Default",
        description: "A basic carousel.",
        demo: html! {
            <Carousel class="w-full max-w-xs">
                <CarouselContent>
                    { for (1..=5).map(|i| html! {
                        <CarouselItem>
                            <Card>
                                <CardContent class="flex aspect-square items-center justify-center p-6">
                                    <span class="text-4xl font-semibold">{ i }</span>
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
        <CarouselItem>{ "Slide 1" }</CarouselItem>
        <CarouselItem>{ "Slide 2" }</CarouselItem>
        <CarouselItem>{ "Slide 3" }</CarouselItem>
    </CarouselContent>
    <CarouselPrevious />
    <CarouselNext />
</Carousel>"#,
    }];

    let props = vec![
        PropDoc {
            name: "orientation",
            prop_type: "&str",
            default: "\"horizontal\"",
            description: "Carousel orientation",
        },
        PropDoc {
            name: "loop_slides",
            prop_type: "bool",
            default: "false",
            description: "Enable infinite loop",
        },
        PropDoc {
            name: "auto_play",
            prop_type: "bool",
            default: "false",
            description: "Auto-advance slides",
        },
        PropDoc {
            name: "auto_play_interval",
            prop_type: "u32",
            default: "3000",
            description: "Auto-play interval (ms)",
        },
    ];

    html! { <ComponentPage name="Carousel" description="A carousel with motion and swipe gestures." {examples} {props} /> }
}
