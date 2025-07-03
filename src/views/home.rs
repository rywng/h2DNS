use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home(route: Vec<String>) -> Element {
    rsx! {
        div { "Accessing: {route:?}" }
        footer { "Server running h2dns" }
    }
}
