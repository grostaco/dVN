use std::rc::Rc;

use reqwest::Client;
use web_sys::console::info;

pub async fn post_render(client: Rc<Client>, script: String) -> String {
    let content = client
        .post("http://127.0.0.1:8000/api/render")
        .body(script)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    content
}

//     // let content: Value = serde_json::from_str(&content).unwrap();
//     // info!("Received code {} from render endpoint", content["code"]);
//     // if content["code"].as_u64().unwrap() == 200 {
//     //     let ids = content["data"]
//     //         .as_array()
//     //         .unwrap()
//     //         .iter()
//     //         .map(|x| x.as_u64().unwrap())
//     //         .collect::<Vec<_>>();
//     //     link.send_message(Msg::UpdateIndex(ids));
//     //     link.send_message(Msg::UpdateLog(
//     //         String::from_utf8(
//     //             content["log"]
//     //                 .as_array()
//     //                 .unwrap()
//     //                 .iter()
//     //                 .map(|v| v.as_u64().unwrap() as u8)
//     //                 .collect(),
//     //         )
//     //         .unwrap(),
//     //     ));
// }
