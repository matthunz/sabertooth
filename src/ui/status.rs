use dioxus::prelude::*;
use dioxus_material::{use_theme, Icon, IconKind};

#[component]
pub fn Status<'a>(
    cx: Scope<'a>,
    username: &'a str,
    avatar_uri: &'a str,
    timestamp: &'a str,
    content: &'a str,
    favorites_count: u32,
    is_favorited: bool,
    onfavorite: EventHandler<'a>,

    reposts_count: u32,
    is_reposted: bool,
    onrepost: EventHandler<'a>,

    replies_count: u32,
    is_replied: bool,
    onreply: EventHandler<'a>,

    is_bookmarked: bool,
    onbookmark: EventHandler<'a>,
) -> Element<'a> {
    let theme = use_theme(cx);

    render!(
        li {
            display: "flex",
            flex_direction: "row",
            gap: "10px",
            list_style: "none",
            padding: "10px 0",
            border_bottom: "2px solid #eee",
            overflow: "hidden",
            div { display: "flex", flex_direction: "column", align_items: "center", div {
                width: "30px",
                height: "30px",
                border_radius: "50%",
                background_image: "url('{avatar_uri}')",
                background_size: "contain"
            } }

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

                div { dangerous_inner_html: "{content}" }
                ul {
                    display: "flex",
                    flex_direction: "row",
                    align_items: "center",
                    margin: "10px 0",
                    padding: 0,
                    list_style: "none",
                    CountAction {
                        icon: IconKind::Favorite,
                        count: *favorites_count,
                        is_active: *is_favorited,
                        onclick: |_| onfavorite.call(())
                    }
                    CountAction {
                        icon: IconKind::Reply,
                        count: *replies_count,
                        is_active: *is_replied,
                        onclick: |_| onreply.call(())
                    }
                    CountAction {
                        icon: IconKind::Forward,
                        count: *reposts_count,
                        is_active: *is_reposted,
                        onclick: |_| onrepost.call(())
                    }

                    li { color: if *is_bookmarked { &theme.primary_color } else { "inherit" }, cursor: "pointer", onclick: |_| onbookmark.call(()),
                        Icon { kind: IconKind::Bookmark, is_filled: *is_bookmarked }
                    }
                }
            }
        }
    )
}

#[component]
fn CountAction<'a>(
    cx: Scope<'a>,
    icon: IconKind,
    count: u32,
    is_active: bool,
    onclick: EventHandler<'a>,
) -> Element<'a> {
    let theme = use_theme(cx);

    render!(
        li {
            flex: 1,
            display: "flex",
            flex_direction: "row",
            align_items: "center",
            gap: "5px",
            onclick: |_| onclick.call(()),
            span { color: if *is_active { &theme.primary_color } else { "inherit" }, cursor: "pointer", Icon { kind: *icon, is_filled: *is_active } }
            "{count}"
        }
    )
}

#[cfg(feature = "lookbook")]
#[lookbook::preview]
pub fn StatusPreview(cx: Scope) -> Element {
    let is_favorited = use_state(cx, || true);
    let is_replied = use_state(cx, || false);
    let is_reposted = use_state(cx, || false);
    let is_bookmarked = use_state(cx, || true);

    render!(
        ul { width: "100%", max_width: "400px",
            Status {
                username: "matthunz",
                avatar_uri: "https://avatars.githubusercontent.com/u/9288430?v=4",
                timestamp: "2d",
                content: "Hello World!",
                favorites_count: 827,
                is_favorited: **is_favorited,
                onfavorite: move |_| is_favorited.set(!is_favorited),
                replies_count: 32,
                is_replied: **is_replied,
                onreply: move |_| is_replied.set(!is_replied),
                reposts_count: 112,
                is_reposted: **is_reposted,
                onrepost: move |_| is_reposted.set(!is_reposted),
                is_bookmarked: **is_bookmarked,
                onbookmark: move |_| is_bookmarked.set(!is_bookmarked)
            }
        }
    )
}
