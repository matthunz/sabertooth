use dioxus::prelude::*;
use dioxus_material::{Button, TextField};

#[component]
pub fn Server<'a>(cx: Scope<'a>, onselect: EventHandler<'a, &'a str>) -> Element<'a> {
    let server_uri = use_state(cx, || String::new());

    render! {
        div {
            flex: 1,
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            justify_content: "center",
            gap: "20px",
            h5 { font_size: "24px", "Pick your mastodon server" }
            TextField {
                label: "Server address",
                value: &*server_uri,
                onchange: |event: FormEvent| {
                    server_uri.set(event.value.clone());
                }
            }
            Button { onpress: move |_| onselect.call(&*server_uri), "Login" }
        }
    }
}

#[cfg(feature = "lookbook")]
#[lookbook::preview]
pub fn ServerPreview(cx: Scope) -> Element {
    render!(Server { onselect: |_| {} })
}
