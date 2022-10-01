use crate::{
    components::{Nav, PlayMusic, PlaySound, Selection},
    services::engine::{init_engine, next_engine},
};
use image_rpg::parser::{
    directives::{Directive, MusicPlay, SoundPlay},
    script::ScriptContext,
};
use reqwest::Client;
use wasm_bindgen::{prelude::Closure, JsCast};
use yew::{function_component, html, use_mut_ref, use_ref, use_state, Callback};
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
    let ended = use_mut_ref(bool::default);

    let music = use_state(MusicPlay::default);
    let sound = use_state(SoundPlay::default);

    let next_engine = {
        let client = client;
        let engine_response = engine_response.clone();
        let music = music.clone();
        let sound = sound.clone();
        let ended = ended.clone();
        use_async(async move {
            loop {
                *engine_response.borrow_mut() = next_engine(client.clone(), *choice.borrow()).await;

                if let Some(response) = engine_response.borrow().as_ref() {
                    if let ScriptContext::Directive(directive) = &response.context {
                        match directive {
                            Directive::MusicPlay(musicplay) => {
                                music.set(musicplay.clone());
                            }
                            Directive::SoundPlay(soundplay) => {
                                sound.set(soundplay.clone());
                            }
                            _ => {}
                        }
                    }
                    if response.id != 0 {
                        break;
                    }
                } else {
                    *ended.borrow_mut() = true;
                    break;
                }
            }
            Ok::<_, ()>(())
        })
    };

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

    let id = engine_response.borrow().as_ref().map(|res| res.id);
    let ended = *ended.borrow();
    html! {
        <div class="dflex dflex-grow-1 dflex-gap-md" style="justify-content: center;">
            if !ended {
                if let Some(id) = id {
                    <img alt="preview" src={format!("http://127.0.0.1:8000/api/rendered/{}/preview.png", id) }/>
                    <PlayMusic path={music.path.to_string()} volume={music.volume.unwrap_or(1.0)} />
                    <PlaySound path={sound.path.to_string()} volume={sound.volume.unwrap_or(1.0)}/>
                } else {
                    <p>{"Press <enter> to start!"}</p>
                }
            } else {
                <p>{"Thank you for playing!"}</p>
            }
        </div>
    }
}
