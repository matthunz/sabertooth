use dioxus::prelude::*;
use dioxus_material::{Icon, IconFont, IconKind, NavigationRail, NavigationRailItem};
use dioxus_router::prelude::*;
use dioxus_signals::use_signal;
use std::collections::HashMap;

mod api;
use self::api::get_timeline;

mod ui;
use self::ui::{Login, Server, Timeline};

#[cfg(not(feature = "lookbook"))]
fn main() {
    fn app(cx: Scope) -> Element {
        render! {
            dioxus_material::Theme { Router::<Route> {} }
        }
    }

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

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Wrap)]
    #[route("/")]
    Home,

    #[route("/explore")]
    Explore,

    #[route("/activity")]
    Activity,

    #[route("/server")]
    ServerScreen,

    #[route("/login/:server_uri")]
    Login { server_uri: String },
}

#[component]
fn Activity(cx: Scope) -> Element {
    render!("Activity")
}

#[component]
fn ServerScreen(cx: Scope) -> Element {
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
        div {
            position: "absolute",
            top: 0,
            left: 0,
            width: "100vw",
            height: "100vh",
            display: "flex",
            flex_direction: "row",
            font_family: "sans-serif",
            overflow: "hidden",
            padding: 0,
            NavigationRail {
                NavItem { route: Route::ServerScreen, icon: IconKind::Person, label: "Account" }
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

    render!(NavigationRailItem {
        icon: render!(Icon { kind: *icon }),
        label: render!("{label}"),
        is_selected: is_selected,
        onselect: move |_| {
            if !is_selected {
                navigator.push(route.clone());
            }
        }
    })
}

#[component]
fn Home(cx: Scope) -> Element {
    let statuses_signal = use_signal(cx, || HashMap::new());
    let timeline_signal = use_signal(cx, || None);

    use_effect(cx, (), |()| async move {
        let timeline = get_timeline().await;
        let mut status_ids = Vec::new();
        for status in timeline.into_iter() {
            status_ids.push(status.id.clone());
            statuses_signal.write().insert(status.id.clone(), status);
        }
        timeline_signal.set(Some(status_ids));
    });

    let timeline = use_memo(
        cx,
        (&*statuses_signal.read(), &*timeline_signal.read()),
        move |_| {
            let statuses = statuses_signal.read();
            timeline_signal.read().as_ref().map(|ids| {
                ids.iter()
                    .map(|id| statuses.get(id).unwrap().clone())
                    .collect::<Vec<_>>()
            })
        },
    );

    if let Some(timeline) = timeline {
        render!(Timeline {
            statuses: timeline,
            onfavorite: move |id| {
                let mut statuses = statuses_signal.write();
                let status = statuses.get_mut(&id).unwrap();
                status.is_favorited = !status.is_favorited;
            },
            onreply: move |id| {
                let mut statuses = statuses_signal.write();
                let status = statuses.get_mut(&id).unwrap();
                status.is_replied = !status.is_replied;
            },
            onreblog: move |id| {
                let mut statuses = statuses_signal.write();
                let status = statuses.get_mut(&id).unwrap();
                status.is_reblogged = !status.is_reblogged;
            },
            onbookmark: move |id| {
                let mut statuses = statuses_signal.write();
                let status = statuses.get_mut(&id).unwrap();
                status.is_bookmarked = !status.is_bookmarked;
            }
        })
    } else {
        None
    }
}
