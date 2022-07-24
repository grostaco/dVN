use std::rc::Rc;

use log::info;
use reqwest::Client;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::{Component, Context, html, Html};
use crate::{scene_control::ButtonInput, text_input::TextInput};

pub enum Msg {
    Submit,
    NewIndex(usize),
    SetScript(String),
    UpdateIndex(Vec<u64>),
    UpdateLog(String),
    ClearCache,
}

#[derive(Debug, Default)]
pub struct App {
    client: Rc<Client>,
    script_text: String,
    index: usize,
    rendered: Vec<u64>,
    log: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { script_text: String::new(), client: Rc::new(Client::new()), rendered: Vec::new(), index: 0, log: String::new() }
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
                let link = ctx.link().clone();
                spawn_local(async move  { 
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
                        let ids = content["data"]
                                                    .as_array()
                                                    .unwrap()
                                                    .iter()
                                                    .map(|x| x.as_u64().unwrap())
                                                    .collect::<Vec<_>>();
                        link.send_message(Msg::UpdateIndex(ids));
                        link.send_message(Msg::UpdateLog(String::from_utf8(content["log"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect()).unwrap()));
                    }
                });
                
                false
            },
            Msg::UpdateIndex(ids) => { self.rendered = ids; true },
            Msg::NewIndex(index) => { self.index = index; true }
            Msg::UpdateLog(l) => { self.log = l; true },
            Msg::ClearCache => { 
                let client = self.client.clone();
                let link = ctx.link().clone();
                spawn_local(async move {
                    client.delete("http://127.0.0.1:8000/api/render").send().await.unwrap();
                    link.send_message(Msg::Submit);
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        info!("Current index: {}", self.index);
        
        let index = self.index;

        let link = ctx.link();
        let on_change = link.callback(Msg::SetScript);
        let onclick = link.callback(|_| Msg::Submit);

        info!("rendered len: {}", self.rendered.len());
        let content = self.rendered.get(index).map(ToString::to_string);
        info!("{:#?}", self.script_text);
        html! {
            <div class="main">
                <div class="container">
                    if self.rendered.is_empty()  {
                        <p style="padding: 10; margin: 0; width: 50%">{"Nothing rendered yet!"}</p>
                    } else {
                        
                        <img src={format!("http://127.0.0.1:8000/api/rendered/{}/preview.png", content.unwrap())}/>
                        <ButtonInput maxlen={self.rendered.len()} onclick={link.callback(Msg::NewIndex)}/>
                        <button class="btn" onclick={link.callback(|_| Msg::ClearCache)}>{"Clear Cache"}</button>
                        
                    }
                    <div class="submission">
                        <TextInput {on_change} value={self.script_text.clone()}/>
                        <button class="button" {onclick}>{"Compile"}</button>
                    </div>
                </div>
                <textarea label="log" class="log" readonly=true value={self.log.clone()}/>
            </div>
        }
    }
}
