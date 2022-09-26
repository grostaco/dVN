use super::{Button, FileView, TextInput};
use crate::services::{
    files::{get_file, post_file},
    render::{post_render, RenderResult},
};
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
    let file = use_mut_ref(|| "assets/autogen_script.script".to_string());

    let text = {
        let file = file.clone();
        let client = client.clone();
        use_async(async move {
            let mut content = get_file(client, file.borrow().as_str()).await;
            // > 4096 KiB is probably not going to load well
            if content.len() > 4096 {
                content = "File too large to load properly. If this is an actual script file, consider refactoring it into multiple files with @jump.".to_string();
            }
            Ok::<_, ()>(content)
        })
    };

    let save = {
        let file = file.clone();
        let script = script.clone();
        let client = client.clone();
        use_async(async move {
            log::info!("Doing a thing");
            post_file(client, file.borrow().as_str(), script.borrow().to_string()).await;
            Ok::<_, ()>(())
        })
    };

    let target_cb = {
        let text = text.clone();
        let file = file.clone();
        Callback::from(move |target_file| {
            *file.borrow_mut() = target_file;
            text.run();
        })
    };

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

    let save_cb = { Callback::from(move |_| save.run()) };

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
                <div class="dflex dflex-col dflex-grow-1">
                    <div id="editor-name">{file.borrow().to_string()}</div>
                    <TextInput on_change={script} content={text.data.clone().unwrap_or_default()}/>
                </div>
                <div>
                    <Button label="Compile" onclick={compile}/>
                    <Button label="Save" onclick={save_cb}/>
                </div>
            </div>
        </div>
    }
}
