use std::path::Path;

use super::Loading;
use crate::services::files::{file_tree, get_files};
use reqwest::Client;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, MouseEvent};
use yew::{function_component, html, use_ref, Callback};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[function_component(FileView)]
pub fn file_view() -> Html {
    let client = use_ref(Client::new);

    let expand = Callback::from(|m: MouseEvent| {
        let event_target = m.target().unwrap();
        let target: HtmlDivElement = event_target.dyn_into().unwrap();
        let class_list = target.class_list();

        if class_list.length() == 1 {
            class_list.add_1("expand").unwrap();
        } else {
            class_list.remove_1("expand").unwrap();
        }
    });

    let files = use_async_with_options(
        async move {
            let files = get_files(client).await;
            Ok::<_, ()>(file_tree(files.iter().map(Path::new).collect(), &expand))
        },
        UseAsyncOptions::enable_auto(),
    );

    html! {
        <div id="file-tree" style="background-color: #181b2b; flex: 1; padding-left: 10px;">
            if let Some(files) = files.data.clone() {
                { files }
            } else {
                <Loading />
            }
        </div>
    }
}

//  <div class="name" onclick={expand}>{"Folder"}</div>
//             <div class="children">
//                 <div class="folders">
//                     <div class="name">{"Folder"}</div>
//                 </div>
//             </div>
