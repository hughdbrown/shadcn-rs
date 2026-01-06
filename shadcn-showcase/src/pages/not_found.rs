//! 404 Not Found page

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

/// 404 Not Found page component
#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div class="not-found-page">
            <div class="not-found-content">
                <h1 class="not-found-title">{ "404" }</h1>
                <p class="not-found-message">{ "Page not found" }</p>
                <p class="not-found-description">
                    { "The page you're looking for doesn't exist or has been moved." }
                </p>
                <Link<Route> to={Route::Home} classes="btn btn-primary">
                    { "Go Home" }
                </Link<Route>>
            </div>
        </div>
    }
}
