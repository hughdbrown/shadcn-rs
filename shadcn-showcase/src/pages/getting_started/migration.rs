//! Migration guide page

use yew::prelude::*;

use crate::components::CodeBlock;

/// Migration page component
#[function_component(MigrationPage)]
pub fn migration_page() -> Html {
    html! {
        <div class="guide-page">
            <header class="guide-header">
                <h1 class="guide-title">{ "Migration Guide" }</h1>
                <p class="guide-description">
                    { "Migrating from React shadcn/ui to Rust shadcn-rs." }
                </p>
            </header>

            <section class="guide-section">
                <h2>{ "Overview" }</h2>
                <p>
                    { "shadcn-rs is a Rust/Yew port of the popular React " }
                    <a href="https://ui.shadcn.com" target="_blank">{ "shadcn/ui" }</a>
                    { " component library. While the API is similar, there are some key differences " }
                    { "due to the language and framework differences." }
                </p>
            </section>

            <section class="guide-section">
                <h2>{ "Component Mapping" }</h2>
                <p>{ "Most components have a direct equivalent:" }</p>

                <div class="guide-table">
                    <table>
                        <thead>
                            <tr>
                                <th>{ "React shadcn/ui" }</th>
                                <th>{ "Rust shadcn-rs" }</th>
                                <th>{ "Notes" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><code>{ "<Button>" }</code></td>
                                <td><code>{ "<Button>" }</code></td>
                                <td>{ "Same API" }</td>
                            </tr>
                            <tr>
                                <td><code>{ "<Input>" }</code></td>
                                <td><code>{ "<Input>" }</code></td>
                                <td>{ "type → input_type" }</td>
                            </tr>
                            <tr>
                                <td><code>{ "<Dialog>" }</code></td>
                                <td><code>{ "<Dialog>" }</code></td>
                                <td>{ "Similar compound pattern" }</td>
                            </tr>
                            <tr>
                                <td><code>{ "<Select>" }</code></td>
                                <td><code>{ "<Select>" }</code></td>
                                <td>{ "Compound components" }</td>
                            </tr>
                            <tr>
                                <td><code>{ "className" }</code></td>
                                <td><code>{ "class" }</code></td>
                                <td>{ "Yew convention" }</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </section>

            <section class="guide-section">
                <h2>{ "JSX to Yew HTML" }</h2>
                <p>{ "Convert JSX syntax to Yew's html! macro:" }</p>

                <h3>{ "React (JSX)" }</h3>
                <CodeBlock
                    code={r#"import { Button } from "@/components/ui/button"

function MyComponent() {
    return (
        <Button
            variant="primary"
            size="lg"
            onClick={() => console.log("clicked")}
        >
            Click me
        </Button>
    )
}"#}
                    language="javascript"
                    title="React JSX"
                />

                <h3>{ "Rust (Yew)" }</h3>
                <CodeBlock
                    code={r#"use yew::prelude::*;
use shadcn_rs::{Button, Variant, Size};

#[function_component(MyComponent)]
fn my_component() -> Html {
    let onclick = Callback::from(|_| {
        web_sys::console::log_1(&"clicked".into());
    });

    html! {
        <Button
            variant={Variant::Primary}
            size={Size::Lg}
            {onclick}
        >
            { "Click me" }
        </Button>
    }
}"#}
                    language="rust"
                    title="Rust Yew"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Key Differences" }</h2>

                <h3>{ "1. Props are typed enums" }</h3>
                <CodeBlock
                    code={r#"// React: string props
<Button variant="primary" size="lg" />

// Rust: typed enum props
<Button variant={Variant::Primary} size={Size::Lg} />"#}
                    language="rust"
                />

                <h3>{ "2. Event handlers use Callbacks" }</h3>
                <CodeBlock
                    code={r#"// React: inline functions
<Button onClick={() => setCount(count + 1)}>

// Rust: Callback type
let onclick = {
    let count = count.clone();
    Callback::from(move |_| {
        count.set(*count + 1);
    })
};
<Button {onclick}>"#}
                    language="rust"
                />

                <h3>{ "3. State uses hooks differently" }</h3>
                <CodeBlock
                    code={r#"// React
const [count, setCount] = useState(0);

// Rust
let count = use_state(|| 0);
count.set(*count + 1);"#}
                    language="rust"
                />

                <h3>{ "4. Children are explicit" }</h3>
                <CodeBlock
                    code={r#"// React: implicit children
<Card>{content}</Card>

// Rust: explicit children prop or inline
<Card>
    { "content" }
</Card>"#}
                    language="rust"
                />

                <h3>{ "5. Reserved words are renamed" }</h3>
                <CodeBlock
                    code={r#"// React
<input type="email" for="email-field" />

// Rust (type and for are reserved)
<Input input_type="email" html_for="email-field" />"#}
                    language="rust"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Controlled vs Uncontrolled" }</h2>
                <p>{ "The pattern is similar but with Rust syntax:" }</p>

                <CodeBlock
                    code={r#"// Controlled component
let value = use_state(String::new);

let onchange = {
    let value = value.clone();
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        value.set(input.value());
    })
};

html! {
    <Input value={(*value).clone()} oninput={onchange} />
}

// Uncontrolled with default value
html! {
    <Input default_value="initial" />
}"#}
                    language="rust"
                    title="Controlled components"
                />
            </section>

            <section class="guide-section">
                <h2>{ "CSS Classes" }</h2>
                <p>{ "The CSS class names are largely the same:" }</p>

                <CodeBlock
                    code={r#"// React with cn() utility
<Button className={cn("custom-class", isActive && "active")}>

// Rust with classes! macro
<Button class={classes!("custom-class", is_active.then_some("active"))}>"#}
                    language="rust"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Compound Components" }</h2>
                <p>{ "Compound component patterns work the same way:" }</p>

                <CodeBlock
                    code={r#"// React
<Dialog>
    <DialogTrigger>Open</DialogTrigger>
    <DialogContent>
        <DialogHeader>
            <DialogTitle>Title</DialogTitle>
        </DialogHeader>
    </DialogContent>
</Dialog>

// Rust
html! {
    <Dialog>
        <DialogTrigger>{ "Open" }</DialogTrigger>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>{ "Title" }</DialogTitle>
            </DialogHeader>
        </DialogContent>
    </Dialog>
}"#}
                    language="rust"
                    title="Compound components"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Form Handling" }</h2>
                <p>{ "Form handling in Rust requires explicit event handling:" }</p>

                <CodeBlock
                    code={r#"// React with react-hook-form
const { register, handleSubmit } = useForm();

<form onSubmit={handleSubmit(onSubmit)}>
    <Input {...register("email")} />
</form>

// Rust manual handling
let email = use_state(String::new);

let onsubmit = {
    let email = email.clone();
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        // Handle form submission
        let email_value = (*email).clone();
    })
};

html! {
    <form {onsubmit}>
        <Input
            value={(*email).clone()}
            oninput={on_email_change}
        />
    </form>
}"#}
                    language="rust"
                    title="Form handling"
                />
            </section>

            <section class="guide-section">
                <h2>{ "Common Gotchas" }</h2>
                <ul class="guide-list">
                    <li>
                        <strong>{ "Clone state handles: " }</strong>
                        { "Always clone state handles before moving into closures" }
                    </li>
                    <li>
                        <strong>{ "Dereference state: " }</strong>
                        { "Use *state to get the value, state.set() to update" }
                    </li>
                    <li>
                        <strong>{ "String literals: " }</strong>
                        { "Wrap text in { \"text\" } inside html! macro" }
                    </li>
                    <li>
                        <strong>{ "Boolean attributes: " }</strong>
                        { "Use disabled={true} not just disabled" }
                    </li>
                    <li>
                        <strong>{ "Optional props: " }</strong>
                        { "Use Option<T> and pass None for optional values" }
                    </li>
                </ul>
            </section>
        </div>
    }
}
