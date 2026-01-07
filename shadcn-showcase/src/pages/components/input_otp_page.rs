//! InputOTP component showcase page

use shadcn_rs::InputOTP;
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(InputOtpPage)]
pub fn input_otp_page() -> Html {
    let examples = vec![
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
            description: "Current value",
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
