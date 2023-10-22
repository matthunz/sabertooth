use crate::{ui::Status, api::StatusData};
use dioxus::prelude::*;
use dioxus_signals::{use_signal, Signal};

#[component]
pub fn Timeline<'a>(cx: Scope, statuses: &'a [StatusData]) -> Element {
    cx.render(rsx! {
        ul { width: "100%", max_width: "400px", margin: "auto", padding: 0, overflow_y: "auto",
            statuses.into_iter().map(|status| render!(Status {
                username: "{status.account.username}",
                avatar_uri: "{status.account.avatar}",
                timestamp: "2d",
                content: "{status.content}",
                favorites_count: status.favorites_count,
                is_favorited: false,
                onfavorite: |_| {},
                replies_count: status.replies_count,
                is_replied: false,
                onreply:  |_| {},
                reposts_count: status.reblogs_count,
                is_reposted: false,
                onrepost: |_| {},
                is_bookmarked: false,
                onbookmark:  |_| {},
            }))
        }
    })
}
