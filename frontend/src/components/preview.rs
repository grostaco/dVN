use reqwest::Client;
use yew::{function_component, html, use_ref, use_state, Callback, Properties};
use yew_hooks::use_async;

use super::Button;
use crate::services::render::delete_cache;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Vec<u64>,
}

#[function_component(Preview)]
pub fn preview(props: &Props) -> Html {
    let index = use_state(|| 0);
    let client = use_ref(Client::new);
    let len = props.data.len();

    let prev = {
        let index = index.clone();
        Callback::from(move |_| index.set((*index as usize).saturating_sub(1)))
    };

    let next = {
        let index = index.clone();
        Callback::from(move |_| index.set((*index + 1).min(len - 1)))
    };

    let delete_cache = {
        use_async(async move {
            delete_cache(client).await;

            Ok::<_, ()>(())
        })
    };

    let onclick_clear = { Callback::from(move |_| delete_cache.run()) };

    let data = &props.data;

    html! {
        <div class="dflex dflex-row">
            if data.is_empty()  {
                <p>{"Nothing rendered yet!"}</p>
            } else {
                <img alt="preview" src={format!("http://127.0.0.1:8000/api/rendered/{}/preview.png", data.get(*index).unwrap_or(&0))}/>
                <div class="dflex dflex-gap-sm dflex-col-reverse">
                    <Button label="Clear Cache" onclick={onclick_clear}/>
                    <Button label="Prev" onclick={prev}/>
                    <Button label="Next" onclick={next}/>
                </div>
            }
        </div>
    }
}
