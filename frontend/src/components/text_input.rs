use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlTextAreaElement, InputEvent};
use yew::{function_component, html, Callback, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_change: Rc<RefCell<String>>,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    e.prevent_default();
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();

    let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
    //web_sys::console::log_1(&target.value().into());
    target.value()
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props { on_change } = props.clone();

    let oninput = {
        Callback::from(move |input_event: InputEvent| {
            *on_change.borrow_mut() = get_value_from_input_event(input_event);
        })
    };

    html! {
        <div class="dflex dflex-col">
            <div id="editor-name">{"tmp.script*"}</div>
            <textarea id="text-input" {oninput} />
        </div>
    }
}
