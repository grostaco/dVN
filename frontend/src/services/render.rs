use log::info;
use std::rc::Rc;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Debug)]
pub struct RenderResult {
    pub code: u64,
    pub log: Vec<u8>,
    pub data: Vec<u64>,
}

pub async fn post_render(client: Rc<Client>, script: String) -> RenderResult {
    let content = client
        .post("http://127.0.0.1:8000/api/render")
        .body(script)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    info!("content: {content}");

    serde_json::from_str(&content).unwrap()
}

pub async fn delete_cache(client: Rc<Client>) {
    client
        .delete("http://127.0.0.1:8000/api/render")
        .send()
        .await
        .unwrap();
}
