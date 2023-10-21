use dioxus::prelude::*;
use dioxus_material::{NavigationRail, NavigationRailItem, use_theme, use_theme_provider, Theme};
use dioxus_router::prelude::*;
use log::LevelFilter;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    use_theme_provider(cx, Theme::default());

    render! { Router::<Route> {} }
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
}

#[component]
fn Activity(cx: Scope,) -> Element {
    render! {
        Link { to: Route::Home {}, "Go back home" }
        "Activity"
    }
}

#[component]
fn Explore(cx: Scope,) -> Element {
    render! {
        Link { to: Route::Home {}, "Go back home" }
        "Explore"
    }
}


#[component]
fn Nav(cx: Scope) -> Element {
    cx.render(rsx! {
        NavigationRail { 
            NavItem { route: Route::Home, label: "Home" }
            NavItem { route: Route::Explore, label: "Explore" }
            NavItem { route: Route::Activity, label: "Activity" }
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
