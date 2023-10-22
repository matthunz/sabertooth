use dioxus::prelude::*;
use dioxus_material::{Icon, IconKind};

#[component]
pub fn Status<'a>(
    cx: Scope<'a>,
    username: &'a str,
    timestamp: &'a str,
    content: &'a str,
) -> Element<'a> {
    render!(
        li { display: "flex", flex_direction: "row", gap: "10px", list_style: "none",
            div { display: "flex", flex_direction: "column", align_items: "center",
                div {
                    width: "30px",
                    height: "30px",
                    border_radius: "50%",
                    background_image: "url('https://avatars.githubusercontent.com/u/9288430?v=4')",
                    background_size: "contain"
                }
            }

            div { flex: 1, display: "flex", flex_direction: "column",
                div { flex: 1, display: "flex", flex_direction: "row", align_items: "center", margin_bottom: "5px",
                    span { flex: 1, "{username}" }
                    span {
                        display: "flex",
                        flex_direction: "row",
                        align_items: "center",
                        gap: "5px",
                        color: "#777",
                        "{timestamp}"
                        Icon { kind: IconKind::MoreHoriz }
                    }
                }
                "{content}"
                ul {
                    display: "flex",
                    flex_direction: "row",
                    align_items: "center",
                    margin: "10px 0",
                    padding: 0,
                    list_style: "none",
                    CountAction { icon: IconKind::Favorite, count: 295 }
                    CountAction { icon: IconKind::Reply, count: 9 }
                    CountAction { icon: IconKind::Forward, count: 76 }
                    Action { icon: IconKind::Bookmark }
                }
            }
        }
    )
}

#[component]
fn CountAction(cx: Scope, icon: IconKind, count: u32) -> Element {
    render!(
        li { flex: 1, display: "flex", flex_direction: "row", align_items: "center", gap: "5px",
            Icon { kind: *icon }
            "{count}"
        }
    )
}

#[component]
fn Action(cx: Scope, icon: IconKind) -> Element {
    render!(
        li { flex: 1, Icon { kind: *icon } }
    )
}

#[cfg(feature = "lookbook")]
#[lookbook::preview]
pub fn StatusPreview(cx: Scope) -> Element {
    render!(
        ul { width: "100%", max_width: "400px", Status { username: "matthunz", timestamp: "2d", content: "Hello World!" } }
    )
}
