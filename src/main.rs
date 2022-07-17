mod core;
mod parser;
mod render;

use crate::core::engine::Engine;
use std::{path::Path, fs::{self, OpenOptions}, io::Write};

use log::{info, warn};
use wasm_bindgen::{JsCast, UnwrapThrowExt, JsValue};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;


enum Msg {
    Submit,
    SetScript(String),
}

struct Model {
    engine: Option<Engine>,
    script_text: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub on_change: Callback<String>,
}

fn get_value_from_input_event(e: InputEvent) -> String {
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
        Self { engine: None, script_text: String::new() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetScript(s) => {
                self.script_text = s;
                true
            },
            Msg::Submit => {
                let p = Path::new("autogen_script.script");
                web_sys::console::log_1(&("Doing a thing!".to_string()).into());
                OpenOptions::new().write(true).read(true).create(true).truncate(true).open("autogen_script.script").unwrap();
                //self.engine.replace(Engine::new(p.to_str().unwrap()).unwrap());
                true
            }
        }
    }

    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let on_change = link.callback(Msg::SetScript);
        let onclick = link.callback(|_| Msg::Submit);

        html!(
            <div>
            <img src="http://127.0.0.1:8000/rendered/1/preview.jpg"/>
            <TextInput {on_change} value={self.script_text.clone()}/>
            <button class="submit" {onclick}>{"Compile!"}</button>
            </div>
        )
    }
}

fn main () {
    //wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    env_logger::init();
    yew::start_app::<Model>();
}