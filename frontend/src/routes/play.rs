use crate::{
    components::{Button, FileView, Nav, TextInput},
    services::{
        engine::{init_engine, next_engine},
        files::get_file,
    },
};
use image_rpg::parser::{directives::Directive, script::ScriptContext};
use reqwest::Client;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlAudioElement;
use yew::{
    function_component, html, use_effect_with_deps, use_mut_ref, use_ref, use_state, Callback,
    Properties,
};
use yew_hooks::{use_async, use_effect_once};

#[function_component(Play)]
pub fn play() -> Html {
    let client = use_ref(Client::new);
    let script_file = use_mut_ref(String::new);
    let init_engine = {
        let client = client;
        let script_file = script_file.clone();
        use_async(async move {
            init_engine(client, script_file.borrow().to_string()).await;
            Ok::<_, ()>(())
        })
    };

    let onselect = {
        let init_engine = init_engine.clone();
        Callback::from(move |file: String| {
            *script_file.borrow_mut() = file;
            init_engine.run();
        })
    };

    html! {
        <>
        <Nav />
        <div class="main dflex dflex-row dflex-gap-sm">
            if init_engine.data.is_none() {
                <Selection {onselect}/>
            }
            else {
                <PlayView/>
            }
        </div>
        </>
    }
}

#[function_component(PlayView)]
pub fn play_view() -> Html {
    let client = use_ref(Client::new);
    let engine_response = use_mut_ref(Option::default);
    let choice = use_mut_ref(bool::default);

    let music = use_state(String::default);
    let sound = use_state(String::default);

    let next_engine = {
        let client = client;
        let engine_response = engine_response.clone();
        let music = music.clone();
        let sound = sound.clone();
        use_async(async move {
            loop {
                *engine_response.borrow_mut() = next_engine(client.clone(), *choice.borrow()).await;

                if let Some(response) = engine_response.borrow().as_ref() {
                    if let ScriptContext::Directive(directive) = &response.context {
                        match directive {
                            Directive::MusicPlay(musicplay) => {
                                music.set(musicplay.path.clone());
                            }
                            Directive::SoundPlay(soundplay) => {
                                sound.set(soundplay.path.clone());
                            }
                            _ => {}
                        }
                    }
                    if response.id != 0 {
                        break;
                    }
                } else {
                    break;
                }
            }
            Ok::<_, ()>(())
        })
    };

    {
        use_effect_with_deps(
            move |music| {
                let element = HtmlAudioElement::new_with_src(music.as_str()).unwrap();
                let promise = element.play().unwrap();
                let future = wasm_bindgen_futures::JsFuture::from(promise);
                spawn_local(async move {
                    future.await.unwrap();
                });
                move || {
                    element.pause().unwrap();
                }
            },
            music,
        );
    }

    {
        use_effect_with_deps(
            move |sound| {
                let element = HtmlAudioElement::new_with_src(sound.as_str()).unwrap();
                let promise = element.play().unwrap();
                let future = wasm_bindgen_futures::JsFuture::from(promise);
                spawn_local(async move {
                    future.await.unwrap();
                });
                // Unlike music, sound does not pause
                move || {}
            },
            sound,
        );
    }

    {
        let next_engine = next_engine;
        use_effect_once(|| {
            let onpress = Closure::wrap(Box::new(move || {
                next_engine.run();
            }) as Box<dyn Fn()>);
            gloo::utils::document().set_onkeypress(Some(onpress.as_ref().unchecked_ref()));
            || {
                onpress.into_js_value();
                gloo::utils::document().set_onkeypress(None);
            }
        });
    };

    html! {
        <div class="dflex dflex-grow-1" style="justify-content: center;">
            if engine_response.borrow().is_none() {
                <p>{"Press <enter> to start!"}</p>
            } else {
                <img alt="preview" src={format!("http://127.0.0.1:8000/api/rendered/{}/preview.png", engine_response.borrow().as_ref().unwrap().id) }/>
            }

        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectionProps {
    pub onselect: Callback<String>,
}

#[function_component(Selection)]
pub fn selection(props: &SelectionProps) -> Html {
    let client = use_ref(Client::new);
    let file = use_mut_ref(String::new);

    let content = {
        let file = file.clone();
        use_async(async move {
            let content = get_file(client, file.borrow().as_str()).await;
            Ok::<_, ()>(content)
        })
    };

    let onselect = {
        let file = file.clone();
        let content = content.clone();

        Callback::from(move |selected_file: String| {
            *file.borrow_mut() = selected_file;
            content.run();
        })
    };

    let onchange = use_mut_ref(String::new);
    let props_onselect = props.onselect.clone();
    let onclick = Callback::from(move |_| {
        props_onselect.emit(file.borrow().to_string());
    });

    let content = content
        .data
        .clone()
        .take()
        .map(|content| if content.len() >= 4096 { "File is too large to load properly. This shouldn't matter if it is a valid script file".to_string()  } else { content })
        .unwrap_or_default();

    html! {
        <>
            <FileView {onselect}/>
            <TextInput {content} on_change={onchange} readonly=true />
            <div class="dflex dflex-col-reverse">
                <Button {onclick} label="Run"/>
            </div>
        </>
    }
}
