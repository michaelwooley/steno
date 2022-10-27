use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
#[function_component(About)]
pub fn about() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Landing));
    html! {
        <div>
            <h1>{ "About" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
