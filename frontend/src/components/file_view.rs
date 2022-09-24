use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, MouseEvent};
use yew::{function_component, html, Callback};

#[function_component(FileView)]
pub fn file_view() -> Html {
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

    html! {
        <div id="file-tree" style="background-color: #181b2b; flex: 1; padding-left: 10px;">
            <div class="name" onclick={expand}>{"Folder"}</div>
            <div class="children">
                <div class="folders">
                    <div class="name">{"Folder"}</div>
                </div>
            </div>
        </div>
    }
}
