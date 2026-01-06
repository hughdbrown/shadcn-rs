//! Form component showcase page

use yew::prelude::*;
use shadcn_rs::{Form, FormItem, FormLabel, FormControl, FormDescription, FormMessage, FormMessageType, Input, Button, Variant};

use crate::components::{ComponentPage, Example, PropDoc};

/// Form showcase page
#[function_component(FormPage)]
pub fn form_page() -> Html {
    let examples = vec![
        Example {
            title: "Basic Form",
            description: "A simple form with validation.",
            demo: html! {
                <Form class="space-y-4 w-[350px]">
                    <FormItem>
                        <FormLabel html_for="username">{ "Username" }</FormLabel>
                        <FormControl>
                            <Input id="username" placeholder="shadcn" />
                        </FormControl>
                        <FormDescription>
                            { "This is your public display name." }
                        </FormDescription>
                    </FormItem>
                    <Button variant={Variant::Primary} r#type="submit">
                        { "Submit" }
                    </Button>
                </Form>
            },
            code: r##"<Form class="space-y-4">
    <FormItem>
        <FormLabel html_for="username">{ "Username" }</FormLabel>
        <FormControl>
            <Input id="username" placeholder="shadcn" />
        </FormControl>
        <FormDescription>
            { "This is your public display name." }
        </FormDescription>
    </FormItem>
    <Button r#type="submit">{ "Submit" }</Button>
</Form>"##,
        },
        Example {
            title: "With Error",
            description: "Form field with validation error.",
            demo: html! {
                <Form class="space-y-4 w-[350px]">
                    <FormItem>
                        <FormLabel html_for="email">{ "Email" }</FormLabel>
                        <FormControl error={true}>
                            <Input id="email" r#type="email" placeholder="email@example.com" />
                        </FormControl>
                        <FormMessage message_type={FormMessageType::Error}>
                            { "Please enter a valid email address." }
                        </FormMessage>
                    </FormItem>
                </Form>
            },
            code: r##"<FormItem>
    <FormLabel html_for="email">{ "Email" }</FormLabel>
    <FormControl error={true}>
        <Input id="email" input_type="email" />
    </FormControl>
    <FormMessage message_type={FormMessageType::Error}>
        { "Please enter a valid email address." }
    </FormMessage>
</FormItem>"##,
        },
        Example {
            title: "Complete Form",
            description: "A complete registration form example.",
            demo: html! {
                <Form class="space-y-4 w-[350px]">
                    <FormItem>
                        <FormLabel html_for="name">{ "Name" }</FormLabel>
                        <FormControl>
                            <Input id="name" placeholder="John Doe" />
                        </FormControl>
                    </FormItem>
                    <FormItem>
                        <FormLabel html_for="email2">{ "Email" }</FormLabel>
                        <FormControl>
                            <Input id="email2" r#type="email" placeholder="john@example.com" />
                        </FormControl>
                    </FormItem>
                    <FormItem>
                        <FormLabel html_for="password">{ "Password" }</FormLabel>
                        <FormControl>
                            <Input id="password" r#type="password" placeholder="Enter password" />
                        </FormControl>
                        <FormDescription>
                            { "Must be at least 8 characters." }
                        </FormDescription>
                    </FormItem>
                    <Button variant={Variant::Primary} r#type="submit" class="w-full">
                        { "Create Account" }
                    </Button>
                </Form>
            },
            code: r##"<Form class="space-y-4">
    <FormItem>
        <FormLabel html_for="name">{ "Name" }</FormLabel>
        <FormControl><Input id="name" /></FormControl>
    </FormItem>
    <FormItem>
        <FormLabel html_for="email">{ "Email" }</FormLabel>
        <FormControl><Input id="email" r#type="email" /></FormControl>
    </FormItem>
    <FormItem>
        <FormLabel html_for="password">{ "Password" }</FormLabel>
        <FormControl><Input id="password" r#type="password" /></FormControl>
        <FormDescription>{ "Must be at least 8 characters." }</FormDescription>
    </FormItem>
    <Button r#type="submit">{ "Create Account" }</Button>
</Form>"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "onsubmit",
            prop_type: "Option<Callback<SubmitEvent>>",
            default: "-",
            description: "Form submit handler",
        },
        PropDoc {
            name: "class",
            prop_type: "Classes",
            default: "-",
            description: "Additional CSS classes",
        },
        PropDoc {
            name: "children",
            prop_type: "Children",
            default: "-",
            description: "Form fields and content",
        },
    ];

    let notes = html! {
        <div class="space-y-4">
            <h3 class="font-medium">{ "Form Components" }</h3>
            <ul class="list-disc pl-6 space-y-2">
                <li><code>{ "Form" }</code>{ " - Form container with submit handling" }</li>
                <li><code>{ "FormItem" }</code>{ " - Groups label, control, description, and message" }</li>
                <li><code>{ "FormLabel" }</code>{ " - Label for the form field" }</li>
                <li><code>{ "FormControl" }</code>{ " - Wraps the actual input component" }</li>
                <li><code>{ "FormDescription" }</code>{ " - Help text for the field" }</li>
                <li><code>{ "FormMessage" }</code>{ " - Displays validation errors" }</li>
            </ul>
        </div>
    };

    html! {
        <ComponentPage
            name="Form"
            description="Building forms with validation and accessibility."
            {examples}
            {props}
            notes={notes}
        />
    }
}
