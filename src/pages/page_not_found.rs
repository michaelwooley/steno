use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Landing));
    html! {
        <div>
            <h1>{ "404: Page not found" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
