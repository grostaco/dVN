use image_rpg::parser::script::ScriptContext;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::rc::Rc;

pub async fn init_engine(client: Rc<Client>, script_file: String) {
    client
        .post("http://127.0.0.1:8000/api/engine/init")
        .query(&[("script", &script_file)])
        .send()
        .await
        .unwrap();
}

#[derive(Deserialize)]
pub struct EngineResponse {
    pub id: u64,
    pub context: ScriptContext,
}

pub async fn next_engine(client: Rc<Client>, choice: bool) -> Option<EngineResponse> {
    let response = client
        .post("http://127.0.0.1:8000/api/engine/next")
        .query(&[("choice", choice)])
        .send()
        .await
        .unwrap();
    if response.status() == StatusCode::NOT_FOUND {
        return None;
    }

    let result: EngineResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    Some(result)
}
