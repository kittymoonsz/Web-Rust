use gloo::console::log;
use serde::Serialize;
use yew::prelude::*;

#[derive(Serialize)]
struct MyObject {
    username: String,
    favorite_lang: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "kitty";
    let myobject = MyObject{
        username: name.to_owned(),
        favorite_lang: "Rust".to_owned(),
    };
    log!(name);
    log!(serde_json::to_string(&myobject).unwrap());
    let class = "titles";
    let message: Option<&str> = Some("I am a message");

    let tasks = vec!["task 1", "task 2", "task 3"];
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

            <ul>
                {list_to_html(tasks)}
            </ul>
        </>
    }
}

fn list_to_html(list: Vec<&str>) -> Html {
    html! {
        <>
            { for list.iter().map(|item| html!{<li>{*item}</li>}) }
        </>
    }
}
