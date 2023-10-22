use dioxus::prelude::*;
use dioxus_material::{Button, NavigationRail, NavigationRailItem, TextField, Theme};
use dioxus_router::prelude::*;
use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        Theme { Router::<Route> {} }
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Nav)]
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
    let server_uri = use_state(cx, || String::new());

    render! {
        div {
            flex: 1,
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            justify_content: "center",
            gap: "20px",
            h5 { "Pick your mastodon server" }
            TextField {
                label: "Server address",
                value: "",
                onchange: |event: FormEvent| {
                    server_uri.set(event.value.clone());
                }
            }
            Button {
                onpress: |_| {
                    navigator
                        .push(Route::Login {
                            server_uri: server_uri.to_string(),
                        });
                },
                "Login"
            }
        }
    }
}

#[component]
fn Login(cx: Scope, server_uri: String) -> Element {
    render! {
        div {
            flex: 1,
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            justify_content: "center",
            gap: "20px",
            h5 { "Login to your mastodon account." }
            TextField { label: "Username", value: "", onchange: |_| {} }
            TextField { label: "Password", value: "", onchange: |_| {} }
            Button { onpress: |_| {}, "Login" }
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
fn Nav(cx: Scope) -> Element {
    cx.render(rsx! {
        div { width: "100vw", height: "100vh", display: "flex", flex_direction: "row",
            NavigationRail { 
                NavItem { route: Route::Home, label: "Home" }
                NavItem { route: Route::Explore, label: "Explore" }
                NavItem { route: Route::Activity, label: "Activity" }
            }
            Outlet::<Route> {}
        }
    })
}

#[component]
fn NavItem<'a>(cx: Scope<'a>, route: Route, label: &'a str) -> Element<'a> {
    let navigator = use_navigator(cx);
    let current_route: Option<Route> = use_route(cx);

    let is_selected = current_route.as_ref() == Some(route);
    render!(
        NavigationRailItem {
            icon: render!("A"),
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
    cx.render(rsx! { h1 { "Home!" } })
}
