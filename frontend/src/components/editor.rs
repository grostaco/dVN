use crate::services::render::{post_render, RenderResult};

use super::{Button, FileView, TextInput};
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
    let render_result = use_state(RenderResult::default);
    let file = use_mut_ref(String::new);

    let target_cb = {
        Callback::from(move |target_file| {
            *file.borrow_mut() = target_file;
        })
    };

    // let text = use_async(async move {
    //     get_files
    // });

    let to_compile = use_state(String::new);

    let render = {
        let to_compile = to_compile.clone();
        let render_result = render_result.clone();
        use_async(async move {
            render_result.set(post_render(client, (*to_compile).to_string()).await);
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

        use_effect_with_deps(
            move |_| {
                render.run();
                || ()
            },
            to_compile,
        );
    }

    {
        let callback = props.data_cb.clone();
        use_effect_with_deps(
            move |render_result| {
                callback.emit((**render_result).to_owned());
                || ()
            },
            render_result,
        );
    }

    html! {
        <div class="text-edit dflex-gap-sm">
            <FileView onselect={target_cb}/>
            <div class="dflex dflex-col dflex-gap-sm" style="flex: 1">
                <TextInput on_change={script}/>
                <div>
                    <Button label="Compile" onclick={compile}/>
                </div>
            </div>
        </div>
    }
}
