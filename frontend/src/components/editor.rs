use super::{Button, FileView, TextInput};
use crate::services::{
    files::{get_file, post_file},
    render::post_render,
};
use backend_types::RenderResult;

use yew::{
    function_component, html, use_effect_with_deps, use_mut_ref, use_state, Callback, Properties,
};
use yew_hooks::use_async;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data_cb: Callback<RenderResult>,
}

#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
    let script = use_mut_ref(String::new);
    let render_result = use_state(|| RenderResult {
        data: Vec::new(),
        log: Vec::new(),
    });
    let file = use_mut_ref(|| "assets/tmp.script".to_string());

    let text = {
        let file = file.clone();
        let script = script.clone();
        use_async(async move {
            let content = get_file(file.borrow().as_str()).await;
            *script.borrow_mut() = content.clone();
            Ok::<_, ()>(content)
        })
    };

    let save = {
        let file = file.clone();
        let script = script.clone();
        use_async(async move {
            log::info!("Saving {}'s content:\n{}", *file.borrow(), *script.borrow());
            post_file(file.borrow().as_str(), script.borrow().to_string())
                .await
                .unwrap();
            Ok::<_, ()>(())
        })
    };

    let target_cb = {
        let text = text.clone();
        let file = file.clone();
        Callback::from(move |target_file| {
            log::info!("File switched to {target_file}");
            *file.borrow_mut() = target_file;
            text.run();
        })
    };

    let to_compile = use_state(String::new);

    let render = {
        let to_compile = to_compile.clone();
        let render_result = render_result.clone();
        use_async(async move {
            render_result.set(match post_render((*to_compile).to_string()).await {
                Ok(r) => r,
                Err(e) => {
                    log::error!("Error while post render: {e:#?}");
                    panic!()
                }
            });
            Ok::<_, ()>(())
        })
    };

    let compile = {
        let to_compile = to_compile.clone();
        let script = script.clone();
        Callback::from(move |_| {
            log::info!("Compilation requested!");
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

    let text = text.data.clone().unwrap_or_default();
    let (text, readonly) = if text.len() > 8192 {
        ("File too large to load properly. If this is an actual script file, consider refactoring it into multiple files with @jump.".to_string(), true)
    } else {
        (text, false)
    };
    html! {
        <div class="text-edit dflex-gap-sm">
            <FileView onselect={target_cb}/>
            <div class="dflex dflex-col dflex-gap-sm" style="flex: 1">
                <div class="dflex dflex-col dflex-grow-1">
                    <div id="editor-name">{file.borrow().to_string()}</div>
                    <TextInput on_change={script} content={text} {readonly} />
                </div>
                <div>
                    <Button label="Compile" onclick={compile}/>
                    <Button label="Save" onclick={save_cb}/>
                </div>
            </div>
        </div>
    }
}
