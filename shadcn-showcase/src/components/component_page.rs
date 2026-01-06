//! Component page template for showcasing individual components

use yew::prelude::*;

use super::CodeBlock;

/// Documentation for a single prop
#[derive(Clone, PartialEq)]
pub struct PropDoc {
    /// Property name
    pub name: &'static str,
    /// Property type
    pub prop_type: &'static str,
    /// Default value
    pub default: &'static str,
    /// Description
    pub description: &'static str,
}

/// An example showing a variant of the component
#[derive(Clone, PartialEq)]
pub struct Example {
    /// Example title
    pub title: &'static str,
    /// Description of this example
    pub description: &'static str,
    /// The demo content
    pub demo: Html,
    /// The code for this example
    pub code: &'static str,
}

/// Properties for ComponentPage
#[derive(Properties, PartialEq)]
pub struct ComponentPageProps {
    /// Component name
    pub name: AttrValue,
    /// Component description
    pub description: AttrValue,
    /// Installation/import code
    #[prop_or_default]
    pub import_code: Option<AttrValue>,
    /// Examples to display
    #[prop_or_default]
    pub examples: Vec<Example>,
    /// Props documentation
    #[prop_or_default]
    pub props: Vec<PropDoc>,
    /// Additional notes
    #[prop_or_default]
    pub notes: Option<Html>,
}

/// Component page template
#[function_component(ComponentPage)]
pub fn component_page(props: &ComponentPageProps) -> Html {
    let import_code = props.import_code.clone().unwrap_or_else(|| {
        format!("use shadcn_rs::{};", props.name).into()
    });

    html! {
        <div class="component-page">
            <header class="component-header">
                <h1 class="component-title">{ &props.name }</h1>
                <p class="component-description">{ &props.description }</p>
            </header>

            <section class="component-section">
                <h2 class="section-title">{ "Installation" }</h2>
                <CodeBlock
                    code={import_code}
                    language="rust"
                    title="Import"
                />
            </section>

            if !props.examples.is_empty() {
                <section class="component-section">
                    <h2 class="section-title">{ "Examples" }</h2>
                    { for props.examples.iter().map(|example| {
                        html! {
                            <div class="example">
                                <h3 class="example-title">{ example.title }</h3>
                                <p class="example-description">{ example.description }</p>
                                <div class="example-demo">
                                    { example.demo.clone() }
                                </div>
                                <CodeBlock
                                    code={example.code}
                                    language="rust"
                                />
                            </div>
                        }
                    })}
                </section>
            }

            if !props.props.is_empty() {
                <section class="component-section">
                    <h2 class="section-title">{ "API Reference" }</h2>
                    <div class="props-table-container">
                        <table class="props-table">
                            <thead>
                                <tr>
                                    <th>{ "Property" }</th>
                                    <th>{ "Type" }</th>
                                    <th>{ "Default" }</th>
                                    <th>{ "Description" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for props.props.iter().map(|prop| {
                                    html! {
                                        <tr>
                                            <td><code>{ prop.name }</code></td>
                                            <td><code>{ prop.prop_type }</code></td>
                                            <td><code>{ prop.default }</code></td>
                                            <td>{ prop.description }</td>
                                        </tr>
                                    }
                                })}
                            </tbody>
                        </table>
                    </div>
                </section>
            }

            if let Some(notes) = &props.notes {
                <section class="component-section">
                    <h2 class="section-title">{ "Notes" }</h2>
                    <div class="component-notes">
                        { notes.clone() }
                    </div>
                </section>
            }
        </div>
    }
}
