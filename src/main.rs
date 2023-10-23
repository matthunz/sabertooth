use dioxus::prelude::*;
use dioxus_material::{Icon, IconFont, IconKind, NavigationRail, NavigationRailItem};
use dioxus_router::prelude::*;
use dioxus_signals::use_signal;
use std::collections::HashMap;

mod api;
use crate::ui::Timeline;

use self::api::get_timeline;

mod ui;
use self::ui::{Login, Server, StatusList};

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
                home: |cx| render!("Home"),
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
    HomeScreen,

    #[route("/explore")]
    ExploreScreen,

    #[route("/activity")]
    ActivityScreen,

    #[route("/server")]
    ServerScreen,

    #[route("/login/:server_uri")]
    Login { server_uri: String },
}

#[component]
fn HomeScreen(cx: Scope) -> Element {
    let navigator = use_navigator(cx);
    use_effect(cx, (), move |_| {
        navigator.push(Route::ExploreScreen);
        async {}
    });
    render!("Home")
}

#[component]
fn ActivityScreen(cx: Scope) -> Element {
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
fn ExploreScreen(cx: Scope) -> Element {
    let statuses = use_signal(cx, || HashMap::new());
    render!(Timeline {
        statuses: statuses,
        id: String::from("public")
    })
}

#[component]
fn Wrap(cx: Scope) -> Element {
    let username = use_state(cx, || None::<String>);
    let account_label = if let Some(username) = &**username {
        username
    } else {
        "Sign in"
    };

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
                NavItem { route: Route::ServerScreen, icon: IconKind::Person, label: account_label }
                NavItem { route: Route::HomeScreen, icon: IconKind::Home, label: "Home" }
                NavItem { route: Route::ExploreScreen, icon: IconKind::Explore, label: "Explore" }
                NavItem {
                    route: Route::ActivityScreen,
                    icon: IconKind::Notifications,
                    label: "Activity"
                }
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
