use crate::{api::StatusData, ui::Status};
use dioxus::prelude::*;

#[component]
pub fn Timeline<'a>(
    cx: Scope,
    statuses: &'a [StatusData],
    onfavorite: EventHandler<'a, String>,
    onreply: EventHandler<'a, String>,
    onreblog: EventHandler<'a, String>,
    onbookmark: EventHandler<'a, String>,
) -> Element {
    render!(
        ul { width: "100%", max_width: "400px", margin: "auto", padding: 0, overflow_y: "auto",
            statuses.into_iter().map(|status| render!(Status {
                key: "{status.id}",
                username: "{status.account.username}",
                avatar_uri: "{status.account.avatar}",
                timestamp: "2d",
                content: "{status.content}",
                favorites_count: status.favorites_count,
                is_favorited: status.is_favorited,
                onfavorite: |_| onfavorite.call(status.id.clone()),
                replies_count: status.replies_count,
                is_replied: status.is_replied,
                onreply:  |_| onreply.call(status.id.clone()),
                reblogs_count: status.reblogs_count,
                is_reblogged: status.is_reblogged,
                onreblog: |_| onreblog.call(status.id.clone()),
                is_bookmarked: status.is_bookmarked,
                onbookmark:  |_| onbookmark.call(status.id.clone()),
            }))
        }
    )
}
