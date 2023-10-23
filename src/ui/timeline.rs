use crate::{
    api::{get_timeline, StatusData},
    ui::StatusList,
};
use dioxus::prelude::*;
use dioxus_signals::{use_signal, Signal};
use std::collections::HashMap;

#[component]
pub fn Timeline(cx: Scope, statuses: Signal<HashMap<String, StatusData>>, id: String) -> Element {
    let timeline_signal = use_signal(cx, || None);

    use_effect(cx, (), move |()| {
        to_owned![id, statuses];
        async move {
            let timeline = get_timeline(&id).await;
            let mut status_ids = Vec::new();
            for status in timeline.into_iter() {
                status_ids.push(status.id.clone());
                statuses.write().insert(status.id.clone(), status);
            }
            timeline_signal.set(Some(status_ids));
        }
    });

    let timeline = use_memo(
        cx,
        (&*statuses.read(), &*timeline_signal.read()),
        move |_| {
            let statuses_ref = statuses.read();
            timeline_signal.read().as_ref().map(|ids| {
                ids.iter()
                    .map(|id| statuses_ref.get(id).unwrap().clone())
                    .collect::<Vec<_>>()
            })
        },
    );

    if let Some(timeline) = timeline {
        render!(StatusList {
            statuses: timeline,
            onfavorite: move |id| {
                let mut statuses = statuses.write();
                let status = statuses.get_mut(&id).unwrap();
                toggle(&mut status.is_favorited, &mut status.favorites_count);
            },
            onreply: move |id| {
                let mut statuses = statuses.write();
                let status = statuses.get_mut(&id).unwrap();
                toggle(&mut status.is_replied, &mut status.replies_count);
            },
            onreblog: move |id| {
                let mut statuses = statuses.write();
                let status = statuses.get_mut(&id).unwrap();
                toggle(&mut status.is_reblogged, &mut status.reblogs_count);
            },
            onbookmark: move |id| {
                let mut statuses = statuses.write();
                let status = statuses.get_mut(&id).unwrap();
                status.is_bookmarked = !status.is_bookmarked;
            }
        })
    } else {
        None
    }
}

fn toggle(a: &mut bool, b: &mut u32) {
    if *a {
        *a = false;
        *b -= 1
    } else {
        *a = true;
        *b += 1
    }
}
