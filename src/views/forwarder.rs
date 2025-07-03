use crate::components::input::InputField;
use dioxus::prelude::*;

#[component]
pub fn Forwarder() -> Element {
    let ip: Signal<String> = use_signal(|| "".to_string());
    let domains: Signal<String> = use_signal(|| "".to_string());
    let token: Signal<String> = use_signal(|| "".to_string());
    let pass: Signal<String> = use_signal(|| "".to_string());
    let msg: Signal<String> = use_signal(|| "".to_string());

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
            button { "submit" }
            p { "{msg}" }
        }
    )
}
