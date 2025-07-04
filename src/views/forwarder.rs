use crate::{backend::forward_ddns, components::input::InputField};
use dioxus::prelude::*;

#[component]
pub fn Forwarder() -> Element {
    let ip: Signal<String> = use_signal(|| "".to_string());
    let domains: Signal<String> = use_signal(|| "".to_string());
    let token: Signal<String> = use_signal(|| "".to_string());
    let pass: Signal<String> = use_signal(|| "".to_string());
    let mut msg: Signal<String> = use_signal(|| "".to_string());

    let upload = move |_| async move {
        let response = forward_ddns(ip(), domains(), token(), pass()).await;
        msg.set(response.map_or_else(|e| e.to_string(), |v| format!("Remote: {v}").to_string()));
    };

    rsx!(
        InputField { name: "IP Address", input_type: "text", signal: ip }
        br {}
        InputField {
            name: "Domains (Separated by Comma)",
            input_type: "text",
            signal: domains,
        }
        br {}
        InputField { name: "Duckdns Token", input_type: "password", signal: token }
        br {}
        InputField { name: "Password", input_type: "password", signal: pass }
        div {
            br {}
            button { onclick: upload, "submit" }
            p { "{msg}" }
        }
    )
}
