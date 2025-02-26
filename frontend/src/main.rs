use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct HelloResponse {
    message: String,
}

#[function_component(App)]
fn app() -> Html {
    let result = use_state(|| String::new());
    let name_input_ref = use_node_ref();

    let fetch_hello = {
        let result = result.clone();
        Callback::from(move |_| {
            let result = result.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("/api/hello").send().await {
                    Ok(response) => {
                        if let Ok(data) = response.json::<HelloResponse>().await {
                            result.set(data.message);
                        }
                    }
                    Err(err) => {
                        result.set(format!("Error fetching data: {:?}", err));
                    }
                }
            });
        })
    };

    let fetch_hello_name = {
        let result = result.clone();
        let name_input_ref = name_input_ref.clone();
        Callback::from(move |_| {
            let result = result.clone();
            let input = name_input_ref.cast::<HtmlInputElement>();
            let name = match input {
                Some(input) => {
                    let value = input.value();
                    if value.is_empty() { "world".to_string() } else { value }
                }
                None => "world".to_string(),
            };

            wasm_bindgen_futures::spawn_local(async move {
                match Request::get(&format!("/api/hello/{}", name)).send().await {
                    Ok(response) => {
                        if let Ok(data) = response.json::<HelloResponse>().await {
                            result.set(data.message);
                        }
                    }
                    Err(err) => {
                        result.set(format!("Error fetching data: {:?}", err));
                    }
                }
            });
        })
    };

    html! {
        <div class="container">
            <h1>{"Hello World from Rust withaasas Ye !!!!!w!"}</h1>
            
            <div class="api-section">
                <h2>{"Test API Endpoints"}</h2>
                <button onclick={fetch_hello}>{"Fetch Hello"}</button>
                <input type="text" ref={name_input_ref} placeholder="Enter a name" />
                <button onclick={fetch_hello_name}>{"Fetch Hello with Name"}</button>
                
                <div class="result">
                    {&*result}
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
} 