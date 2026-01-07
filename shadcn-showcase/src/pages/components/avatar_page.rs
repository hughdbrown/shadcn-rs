//! Avatar component showcase page

use shadcn_rs::{Avatar, AvatarShape, Size};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

/// Avatar showcase page
#[function_component(AvatarPage)]
pub fn avatar_page() -> Html {
    let examples = vec![
        Example {
            title: "With Image",
            description: "Avatar displaying an image.",
            demo: html! {
                <Avatar src="https://github.com/shadcn.png" alt="User" />
            },
            code: r##"<Avatar src="https://github.com/shadcn.png" alt="User" />"##,
        },
        Example {
            title: "With Initials",
            description: "Avatar showing initials when no image is provided.",
            demo: html! {
                <Avatar initials="JD" />
            },
            code: r##"<Avatar initials="JD" />"##,
        },
        Example {
            title: "Fallback Icon",
            description: "Avatar showing fallback icon when image fails to load.",
            demo: html! {
                <Avatar src="/broken-image.jpg" fallback_icon="👤" />
            },
            code: r##"<Avatar src="/broken-image.jpg" fallback_icon="👤" />"##,
        },
        Example {
            title: "Sizes",
            description: "Avatars in different sizes.",
            demo: html! {
                <div class="flex gap-4 items-center">
                    <Avatar initials="XS" size={Size::Xs} />
                    <Avatar initials="SM" size={Size::Sm} />
                    <Avatar initials="MD" size={Size::Md} />
                    <Avatar initials="LG" size={Size::Lg} />
                    <Avatar initials="XL" size={Size::Xl} />
                </div>
            },
            code: r##"<Avatar initials="XS" size={Size::Xs} />
<Avatar initials="SM" size={Size::Sm} />
<Avatar initials="MD" size={Size::Md} />
<Avatar initials="LG" size={Size::Lg} />
<Avatar initials="XL" size={Size::Xl} />"##,
        },
        Example {
            title: "Shapes",
            description: "Circle and square avatars.",
            demo: html! {
                <div class="flex gap-4 items-center">
                    <Avatar initials="C" shape={AvatarShape::Circle} />
                    <Avatar initials="S" shape={AvatarShape::Square} />
                </div>
            },
            code: r##"<Avatar initials="C" shape={AvatarShape::Circle} />
<Avatar initials="S" shape={AvatarShape::Square} />"##,
        },
        Example {
            title: "Group",
            description: "Multiple avatars displayed together.",
            demo: html! {
                <div class="flex -space-x-4">
                    <Avatar initials="A" class="border-2 border-background" />
                    <Avatar initials="B" class="border-2 border-background" />
                    <Avatar initials="C" class="border-2 border-background" />
                    <Avatar initials="+3" class="border-2 border-background" />
                </div>
            },
            code: r##"<div class="flex -space-x-4">
    <Avatar initials="A" class="border-2 border-background" />
    <Avatar initials="B" class="border-2 border-background" />
    <Avatar initials="C" class="border-2 border-background" />
    <Avatar initials="+3" class="border-2 border-background" />
</div>"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "src",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Image source URL",
        },
        PropDoc {
            name: "alt",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Alt text for image",
        },
        PropDoc {
            name: "initials",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Fallback initials",
        },
        PropDoc {
            name: "fallback_icon",
            prop_type: "AttrValue",
            default: "👤",
            description: "Fallback icon when no image or initials",
        },
        PropDoc {
            name: "size",
            prop_type: "Size",
            default: "Md",
            description: "The size of the avatar",
        },
        PropDoc {
            name: "shape",
            prop_type: "AvatarShape",
            default: "Circle",
            description: "Shape of the avatar (Circle or Square)",
        },
        PropDoc {
            name: "class",
            prop_type: "Classes",
            default: "-",
            description: "Additional CSS classes",
        },
    ];

    html! {
        <ComponentPage
            name="Avatar"
            description="An image element with a fallback for representing the user."
            {examples}
            {props}
        />
    }
}
