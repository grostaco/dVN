use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlTextAreaElement, InputEvent};
use yew::{function_component, html, use_effect, Callback, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_change: Rc<RefCell<String>>,
    pub content: String,
    #[prop_or(true)]
    pub readonly: bool,
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
    let Props {
        on_change,
        content,
        readonly,
    } = props.clone();
    let oninput = {
        Callback::from(move |input_event: InputEvent| {
            *on_change.borrow_mut() = get_value_from_input_event(input_event);
        })
    };

    use_effect(move || {
        let element = gloo::utils::document()
            .get_element_by_id("text-input")
            .unwrap();
        let text_area: HtmlTextAreaElement = element.dyn_into().unwrap();
        text_area.set_value(&content);
        || ()
    });

    html! {
        <textarea id="text-input" style="flex: 1" data-gramm="false" {readonly} {oninput} />
    }
}

/*


*/
