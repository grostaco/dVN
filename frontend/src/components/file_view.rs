use super::Loading;
use crate::services::files::{file_tree, get_files};
use std::path::Path;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, MouseEvent};
use yew::{function_component, html, Callback, Properties};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onselect: Callback<String>,
}

#[function_component(FileView)]
pub fn file_view(props: &Props) -> Html {
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

    let view = {
        let onselect = props.onselect.clone();
        Callback::from(move |m: MouseEvent| {
            let event_target = m.target().unwrap();
            let target: HtmlDivElement = event_target.dyn_into().unwrap();
            onselect.emit(target.get_attribute("path").unwrap());
        })
    };

    let files = use_async_with_options(
        async move {
            let files = get_files().await.unwrap().files;
            Ok::<_, ()>(file_tree(
                files.iter().map(Path::new).collect(),
                &expand,
                &view,
            ))
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
