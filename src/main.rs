mod core;
mod parser;
mod render;

use std::sync::{Arc, Mutex};

use log::{info, warn};
use reqwest::Client;
use serde_json::Value;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;


enum Msg {
    Prev,
    Next,
    Submit,
    SetScript(String),
    Update,
}

struct Model {
    client: Client,
    script_text: String,
    index: usize,
    rendered: Arc<Mutex<Vec<u64>>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub on_change: Callback<String>,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    e.prevent_default();
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();

    let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props { value, on_change } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    html! {
        <textarea class="text_input" rows="50" {value} {oninput} />
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { script_text: String::new(), client: Client::new(), rendered: Arc::new(Mutex::new(Vec::new())), index: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetScript(s) => {
                self.script_text = s;
                false
            },
            Msg::Submit => {
                info!("Sent a request");
                let client = self.client.clone();
                let script_text = self.script_text.clone();

                let rendered = self.rendered.clone();
                let link = ctx.link().clone();
                spawn_local(async move  { 
                    let mut rendered = rendered.lock().unwrap();
                    let content = client.post("http://127.0.0.1:8000/api/render")
                        .body(script_text)
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    
                    let content: Value = serde_json::from_str(&content).unwrap();
                    info!("Received code {} from render endpoint", content["code"]);
                    if content["code"].as_u64().unwrap() == 200 {
                        *rendered = content["data"]
                                                    .as_array()
                                                    .unwrap()
                                                    .iter()
                                                    .map(|x| x.as_u64().unwrap())
                                                    .collect::<Vec<_>>();
                    }
                    drop(rendered);
                    link.send_message(Msg::Update)
                });
                
                false
            },
            Msg::Update => true,
            Msg::Next => { self.index = (self.index + 1).min(self.rendered.lock().unwrap().len() - 1); true },
            Msg::Prev => { self.index = self.index.max(1) - 1; true  }
            _ => panic!()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        info!("Current index: {}", self.index);
        let rendered = self.rendered.lock().unwrap();
        let index = self.index;

        let link = ctx.link();
        let on_change = link.callback(Msg::SetScript);
        let onclick = link.callback(|_| Msg::Submit);
        // let onclick_prev = link.callback(|_| Msg::Prev);
        // let onclick_next = link.callback(|_| Msg::Next);

        info!("rendered len: {}", rendered.len());
        let content = rendered.get(index).map(ToString::to_string);
        html!(
            <div>
            if rendered.is_empty()  {
                <p style="padding: 10; margin: 0; width: 50%">{"Nothing rendered yet!"}</p>
            } else {
                <img src={format!("http://127.0.0.1:8000/api/rendered/{}/preview.png", content.unwrap())}/>
                <button class="nextprev" onclick={link.callback(|_| Msg::Prev)}>{"Prev"}</button>
                <button class="nextprev" onclick={link.callback(|_| Msg::Next)}>{"Next"}</button>
            }
            <TextInput {on_change} value={self.script_text.clone()}/>
            <button class="button" {onclick}>{"Compile!"}</button>
            </div>
        )
    }
}

fn main () {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    //env_logger::init();
    yew::start_app::<Model>();
}