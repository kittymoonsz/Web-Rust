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
    let class = "titles";
    let message: Option<&str> = Some("I am a message");
    html! {
        <>
            <h1 class={class}>{"Hello, world!!"}</h1>
            if class == "titles" {
                <p>{"Hi there!!"}</p>
            } else {
                <p>{"class is not titles"}</p>
            }

            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"no message to see today"}</p>
            }
        </>
    }
}