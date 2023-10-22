use dioxus::prelude::*;
use dioxus_material::{Button, TextField};

#[component]
pub fn Login(cx: Scope, server_uri: String) -> Element {
    render! {
        div {
            flex: 1,
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            justify_content: "center",
            gap: "20px",
            h5 {
                "Login to your mastodon account on "
                span { color: "#777", "{server_uri}" }
                "."
            }
            TextField { label: "Username", value: "", onchange: |_| {} }
            TextField { label: "Password", value: "", onchange: |_| {} }
            Button { onpress: |_| {}, "Login" }
        }
    }
}

#[cfg(feature = "lookbook")]
#[lookbook::preview]
pub fn LoginPreview(cx: Scope) -> Element {
    render!(Login {
        server_uri: String::from("https://example.com")
    })
}
