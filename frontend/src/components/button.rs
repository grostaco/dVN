use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
    pub label: &'static str,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let onclick = &props.onclick;
    html! {
        <button {onclick} class="btn">{props.label}</button>
    }
}
