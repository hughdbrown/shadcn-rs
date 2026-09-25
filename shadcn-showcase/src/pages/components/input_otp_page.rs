//! InputOTP component showcase page

use shadcn_rs::{Button, InputOTP, Size, Variant};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(InputOtpPage)]
pub fn input_otp_page() -> Html {
    let code = use_state(String::new);
    let on_change = {
        let code = code.clone();
        Callback::from(move |value: String| code.set(value))
    };
    let clear = {
        let code = code.clone();
        Callback::from(move |_: MouseEvent| code.set(String::new()))
    };

    let examples = vec![
        Example {
            title: "Controlled",
            description: "The parent owns the code and can clear it at any time.",
            demo: html! {
                <div class="space-y-2">
                    <InputOTP length={6} value={(*code).clone()} on_change={on_change.clone()} />
                    <div class="flex items-center space-x-2">
                        <Button variant={Variant::Outline} size={Size::Sm} onclick={clear.clone()}>
                            { "Clear" }
                        </Button>
                        <span class="text-sm text-muted-foreground">
                            { format!("Value: \"{}\"", *code) }
                        </span>
                    </div>
                </div>
            },
            code: r##"let code = use_state(String::new);
let on_change = {
    let code = code.clone();
    Callback::from(move |value: String| code.set(value))
};

<InputOTP length={6} value={(*code).clone()} {on_change} />
<Button onclick={move |_| code.set(String::new())}>{ "Clear" }</Button>"##,
        },
        Example {
            title: "Default",
            description: "A one-time password input with 6 digits.",
            demo: html! {
                <InputOTP length={6} />
            },
            code: r##"<InputOTP length={6} />"##,
        },
        Example {
            title: "Four Digits",
            description: "OTP input with 4 digits.",
            demo: html! {
                <InputOTP length={4} />
            },
            code: r##"<InputOTP length={4} />"##,
        },
        Example {
            title: "Masked",
            description: "OTP input with masked values (dots instead of characters).",
            demo: html! {
                <InputOTP length={6} masked={true} />
            },
            code: r##"<InputOTP length={6} masked={true} />"##,
        },
        Example {
            title: "Disabled",
            description: "Disabled OTP input.",
            demo: html! {
                <InputOTP length={6} disabled={true} />
            },
            code: r##"<InputOTP length={6} disabled={true} />"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "length",
            prop_type: "usize",
            default: "6",
            description: "Number of OTP input fields",
        },
        PropDoc {
            name: "value",
            prop_type: "Option<String>",
            default: "-",
            description: "Controlled value (followed on every change)",
        },
        PropDoc {
            name: "masked",
            prop_type: "bool",
            default: "false",
            description: "Show dots instead of characters",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Disabled state",
        },
        PropDoc {
            name: "pattern",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Input validation pattern",
        },
        PropDoc {
            name: "on_change",
            prop_type: "Option<Callback<String>>",
            default: "-",
            description: "Change handler",
        },
        PropDoc {
            name: "on_complete",
            prop_type: "Option<Callback<String>>",
            default: "-",
            description: "Completion handler",
        },
    ];

    html! { <ComponentPage name="Input OTP" description="Accessible one-time password component." {examples} {props} /> }
}
