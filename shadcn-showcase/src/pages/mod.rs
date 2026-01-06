//! Pages for the showcase application

mod home;
mod not_found;

// Getting Started
mod getting_started;

// Component pages
mod components;

pub use home::HomePage;
pub use not_found::NotFoundPage;
pub use getting_started::*;
pub use components::*;
