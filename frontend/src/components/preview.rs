use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Properties};
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};

use super::Button;
use crate::services::render::{delete_cache, get_preview};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Vec<u64>,
}

#[function_component(Preview)]
pub fn preview(props: &Props) -> Html {
    let index = use_state(|| 0);
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
            delete_cache().await;

            Ok::<_, ()>(())
        })
    };

    let onclick_clear = { Callback::from(move |_| delete_cache.run()) };

    let data = &props.data;
    let preview = {
        let data = data.clone();
        let index = index.clone();
        use_async_with_options(
            async move { get_preview(*data.get(*index).unwrap_or(&0)).await },
            UseAsyncOptions::default(),
        )
    };

    {
        let preview = preview.clone();
        let data = data.clone();
        use_effect_with_deps(
            move |_| {
                preview.run();

                || ()
            },
            (index, data),
        );
    }

    {
        let preview = preview;

        use_effect_with_deps(
            move |preview| {
                if let Some(preview) = &preview.data {
                    let preview_element: HtmlImageElement = gloo::utils::document()
                        .get_element_by_id("preview")
                        .unwrap()
                        .dyn_into()
                        .unwrap();
                    let preview_encoded = base64::encode(&preview.bytes);
                    preview_element.set_src(&format!("data:image/png;base64,{preview_encoded}"));
                }

                || ()
            },
            preview,
        );
    }

    html! {
        <div class="dflex dflex-row">
            if data.is_empty()  {
                <p>{"Nothing rendered yet!"}</p>
            } else {
                <img alt="preview" id="preview" src=""/>
                <div class="dflex dflex-gap-sm dflex-col-reverse">
                    <Button label="Clear Cache" onclick={onclick_clear}/>
                    <Button label="Prev" onclick={prev}/>
                    <Button label="Next" onclick={next}/>
                </div>
            }
        </div>
    }
}
