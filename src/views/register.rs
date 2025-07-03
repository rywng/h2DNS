use dioxus::prelude::*;

use crate::{backend::register_domain, components::input::InputField};

#[component]
pub fn Register() -> Element {
    let domain: Signal<String> = use_signal(|| "".to_string());
    let ip: Signal<String> = use_signal(|| "".to_string());
    let pass: Signal<String> = use_signal(|| "".to_string());
    let mut msg = use_signal(|| "".to_string());
    let upload = move |_| async move {
        let response = register_domain(domain(), ip(), pass()).await;
        match response {
            Ok(_) => {
                msg.set("Register successful".to_string());
            }
            Err(e) => {
                msg.set(e.to_string());
            }
        }
    };
    rsx! {
        div {
            InputField { name: "Domain name", input_type: "text", signal: domain }
            br {}
            InputField { name: "IP Address", input_type: "text", signal: ip }
            br {}
            InputField { name: "Password", input_type: "password", signal: pass }
            br {}
            div {
                br {}
                button { onclick: upload, "submit" }
                p { "{msg}" }
            }
        }
    }
}
