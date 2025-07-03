use dioxus::prelude::*;

#[component]
pub(crate) fn InputField(name: String, input_type: String, signal: Signal<String>) -> Element {
    rsx!(
        label { "{name}" }
        br {}
        input {
            maxlength: "48",
            minlength: "2",
            r#type: "{input_type}",
            required: "true",
            oninput: move |evt| {
                signal.set(evt.value());
            },
            value: "{signal()}"
        }
    )
}
