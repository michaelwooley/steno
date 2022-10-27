use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::{about::About, landing::Landing, page_not_found::PageNotFound};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Landing,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    PageNotFound,
}

#[allow(clippy::let_unit_value)]
fn switch_routes(routes: &Route) -> Html {
    match routes {
        Route::Landing => html! { <Landing />},
        Route::About => html! {
            <About />
        },
        Route::PageNotFound => html! { <PageNotFound />},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch_routes)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
