use dioxus::prelude::*;
use dioxus_material::{use_theme, Icon, IconKind};

/// Status component.
#[component]
pub fn Status<'a>(
    cx: Scope<'a>,

    /// Username of the account who posted the status.
    username: &'a str,

    /// Avatar URI of the account who posted the status.
    avatar_uri: &'a str,

    /// Timestamp the status was posted.
    timestamp: &'a str,

    /// HTML content of the status.
    content: &'a str,

    /// Amount of favorites the status received.
    favorites_count: u32,

    /// True if the current user favorited the status.
    is_favorited: bool,

    /// Handler of click events for the favorite action.
    onfavorite: EventHandler<'a>,

    /// Amount of reblogs the status received.
    reblogs_count: u32,
    
    /// True if the current user reblogged the status.
    is_reblogged: bool,

    /// Handler of click events for the reblog action.
    onreblog: EventHandler<'a>,

    /// Amount of replies the status received.
    replies_count: u32,

    /// True if the current user replied to the status.
    is_replied: bool,

    /// Handler of click events for the reply action.
    onreply: EventHandler<'a>,

    /// True if the current user bookmarked the status.
    is_bookmarked: bool,

    /// Handler of click events for the bookmark action.
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
                    Action {
                        icon: IconKind::Favorite,
                        count: *favorites_count,
                        is_active: *is_favorited,
                        onclick: |_| onfavorite.call(())
                    }
                    Action {
                        icon: IconKind::Reply,
                        count: *replies_count,
                        is_active: *is_replied,
                        onclick: |_| onreply.call(())
                    }
                    Action {
                        icon: IconKind::Forward,
                        count: *reblogs_count,
                        is_active: *is_reblogged,
                        onclick: |_| onreblog.call(())
                    }

                    li { color: if *is_bookmarked { &theme.primary_color } else { "inherit" }, cursor: "pointer", onclick: |_| onbookmark.call(()),
                        Icon { kind: IconKind::Bookmark, is_filled: *is_bookmarked }
                    }
                }
            }
        }
    )
}

/// Action button component.
#[component]
fn Action<'a>(
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
                reblogs_count: 112,
                is_reblogged: **is_reposted,
                onreblog: move |_| is_reposted.set(!is_reposted),
                is_bookmarked: **is_bookmarked,
                onbookmark: move |_| is_bookmarked.set(!is_bookmarked)
            }
        }
    )
}
