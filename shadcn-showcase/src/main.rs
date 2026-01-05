use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="showcase-container">
            <header class="showcase-header">
                <h1>{ "shadcn-rs" }</h1>
                <p>{ "UI Components for Rust/WebAssembly" }</p>
            </header>
            <main class="showcase-main">
                <div class="welcome">
                    <h2>{ "Welcome to shadcn-rs Showcase" }</h2>
                    <p>
                        { "This showcase application will demonstrate all 59 components " }
                        { "once they are implemented." }
                    </p>
                    <p>
                        { "Current status: Project setup complete, ready for component implementation." }
                    </p>
                </div>
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
