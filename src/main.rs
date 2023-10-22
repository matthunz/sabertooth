use dioxus::prelude::*;
use dioxus_material::{Icon, IconFont, IconKind, NavigationRail, NavigationRailItem, Theme};
use dioxus_router::prelude::*;
use dioxus_signals::use_signal;
use serde::Deserialize;

mod ui;
use crate::ui::Status;

use self::ui::{Login, Server};

#[cfg(not(feature = "lookbook"))]
fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

#[cfg(feature = "lookbook")]
fn main() {
    fn app(cx: Scope) -> Element {
        render! {
            IconFont {}
            lookbook::LookBook {
                home: Home,
                previews: [ui::LoginPreview, ui::ServerPreview, ui::StatusPreview]
            }
        }
    }

    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        Theme { Router::<Route> {} }
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Wrap)]
    #[route("/")]
    Home,

    #[route("/explore")]
    Explore,

    #[route("/activity")]
    Activity,

    #[route("/login/:server_uri")]
    Login { server_uri: String },
}

#[component]
fn Activity(cx: Scope) -> Element {
    let navigator = use_navigator(cx);
    let _server_uri = use_state(cx, || String::new());

    render! {
        Server {
            onselect: move |server_uri: &str| {
                navigator
                    .push(Route::Login {
                        server_uri: server_uri.to_owned(),
                    });
            }
        }
    }
}

#[component]
fn Explore(cx: Scope) -> Element {
    render! {
        Link { to: Route::Home {}, "Go back home" }
        "Explore"
    }
}

#[component]
fn Wrap(cx: Scope) -> Element {
    cx.render(rsx! {
        IconFont {}
        div { width: "100vw", height: "100vh", display: "flex", flex_direction: "row", font_family: "sans-serif",
            NavigationRail { 
                NavItem { route: Route::Home, icon: IconKind::Home, label: "Home" }
                NavItem { route: Route::Explore, icon: IconKind::Explore, label: "Explore" }
                NavItem { route: Route::Activity, icon: IconKind::Notifications, label: "Activity" }
            }
            div { flex: 1, overflow: "auto", Outlet::<Route> {} }
        }
    })
}

#[component]
fn NavItem<'a>(cx: Scope<'a>, route: Route, icon: IconKind, label: &'a str) -> Element<'a> {
    let navigator = use_navigator(cx);
    let current_route: Option<Route> = use_route(cx);

    let is_selected = current_route.as_ref() == Some(route);
    render!(
        NavigationRailItem {
            icon: render!(Icon { kind : * icon }),
            label: render!("{label}"),
            is_selected: is_selected,
            onselect: move |_| {
                if !is_selected {
                    navigator.push(route.clone());
                }
            }
        }
    )
}

#[component]
fn Home(cx: Scope) -> Element {
    let statuses = use_signal(cx, || None);
    use_effect(cx, (), |()| async move {
        let timeline = get_timeline().await;
        statuses.set(Some(timeline));
    });


    cx.render(rsx! {
        ul { width: "100%", max_width: "400px", margin: "auto", padding: 0, overflow_y: "auto",
            statuses.read().iter().flatten().map(|status| render!(Status {
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

#[derive(Debug, Deserialize)]
struct Account {
    username: String,
    avatar: String
}

#[derive(Debug, Deserialize)]
struct StatusData {
    id: String,
    account: Account,
    content: String,
    #[serde(rename = "favourites_count")]
    favorites_count: u32,
    reblogs_count: u32,
    replies_count: u32,
}

async fn get_timeline() -> Vec<StatusData> {
     reqwest::get("https://mas.to/api/v1/timelines/public")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
