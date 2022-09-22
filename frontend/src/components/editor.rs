use crate::services::render::{post_render, RenderResult};

use super::{Button, TextInput};
use reqwest::Client;
use yew::{
    function_component, html, use_effect_with_deps, use_mut_ref, use_ref, use_state, Callback,
    Properties,
};
use yew_hooks::use_async;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data_cb: Callback<RenderResult>,
}

#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
    let client = use_ref(Client::new);
    let script = use_mut_ref(String::new);
    let render_result = use_mut_ref(RenderResult::default);

    let to_compile = use_state(String::new);

    let render = {
        let to_compile = to_compile.clone();
        let render_result = render_result.clone();
        use_async(async move {
            *render_result.borrow_mut() = post_render(client, (*to_compile).to_string()).await;
            Ok::<_, ()>(())
        })
    };

    let compile = {
        let to_compile = to_compile.clone();
        let script = script.clone();
        Callback::from(move |_| {
            to_compile.set(script.borrow().to_string());
        })
    };

    {
        let to_compile = to_compile;
        let callback = props.data_cb.clone();
        use_effect_with_deps(
            move |_| {
                render.run();
                callback.emit(render_result.borrow().clone());
                || ()
            },
            to_compile,
        );
    }
    html! {
        <div class="text-edit dflex-gap-sm">
            <TextInput on_change={script}/>
            <div>
                <Button label="Compile" onclick={compile}/>
            </div>
        </div>
    }
}
