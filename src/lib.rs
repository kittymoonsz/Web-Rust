use gloo::console::log;
use serde::Serialize;
use yew::prelude::*;

#[derive(Serialize)]
struct myObject {
    username: String,
    favorite_lang: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "kitty";
    let myobject = myObject{
        username: name.to_owned(),
        favorite_lang: "Rust".to_owned(),
    };
    log!(name);
    log!(serde_json::to_string(&myobject).unwrap());
    let class = "title";
    html! {
        <>
            <h1 class={class}>{"Hello, world!!!!!"}</h1>
            if class == "titles" {
                <p>{"Hi there!!"}</p>
            }
        </>
    }
}