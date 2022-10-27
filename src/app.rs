use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::FocusEvent;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_ref(NodeRef::default);
    let name = use_state(String::new);
    let greet_msg = use_state(String::new);

    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg =
                        invoke("greet", to_value(&GreetArgs { name: &*name }).unwrap()).await;
                    log(&new_msg.as_string().unwrap());
                    greet_msg.set(new_msg.as_string().unwrap());
                });

                || {}
            },
            name2,
        );
    }

    let greet = {
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <h1>{"Steno"}</h1>
            <div class="row">
                <a href="https://github.com/michaelwooley/steno" target="_blank">
                    <img src="public/icon.png" class="logo" alt="Steno logo"/>
                </a>
            </div>
            <div class="row" style="border-bottom:1px solid #cccccc;margin:2rem;">
            </div>
            <div class="row">
                <form onsubmit={greet}>
                    <input id="greet-input" ref={&*greet_input_ref} placeholder="Enter a name and press enter..." />
                    <button type="submit">{"Greet"}</button>
                </form>
            </div>

            <p><b>{ &*greet_msg }</b></p>
        </main>
    }
}
