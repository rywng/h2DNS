use crate::backend::server::resolve_domain;
use dioxus::prelude::*;

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Resolver(domain: String) -> Element {
    let domain = use_server_future(|| resolve_domain("google".into()))?.unwrap();
    let result: String = match domain {
        Ok(ip) => ip.to_string(),
        Err(error) => error.to_string(),
    };

    rsx!(
        div { "{result}" }
    )
}
