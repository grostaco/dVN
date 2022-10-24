use std::collections::HashMap;

use crate::{
    components::{IconText, Nav, PlayMusic, PlaySound, Selection},
    services::{
        engine::{init_engine, next_engine},
        render::get_preview,
    },
};
use backend_types::EngineNext;
use image_rpg::parser::{
    directives::{Directive, Metadata, MusicPlay, SoundPlay},
    script::ScriptContext,
};
use regex::Regex;
use reqwest::Client;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlImageElement;
use yew::{
    function_component, html, use_effect_with_deps, use_mut_ref, use_ref, use_state,
    virtual_dom::VNode, Callback,
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
        if init_engine.data.is_none() {
                <div class="main dflex dflex-row dflex-gap-sm">
                    <Selection {onselect}/>
                </div>
            }
            else {
                <div class="main dflex dflex-row dflex-gap-sm dflex-justify-center">
                    <PlayView/>
                </div>
            }

        </>
    }
}

#[function_component(PlayView)]
pub fn play_view() -> Html {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"([\w -]+)\.").unwrap();
    };

    let client = use_ref(Client::new);
    let engine_response = use_mut_ref(Option::default);
    let choice = use_mut_ref(bool::default);
    let ended = use_mut_ref(bool::default);

    let music = use_state(MusicPlay::default);
    let sound = use_state(SoundPlay::default);
    let metadata = use_mut_ref(HashMap::<String, String>::default);

    let next_engine = {
        let client = client;
        let engine_response = engine_response.clone();
        let music = music.clone();
        let sound = sound.clone();
        let ended = ended.clone();
        let metadata = metadata.clone();
        use_async(async move {
            loop {
                *engine_response.borrow_mut() = next_engine(client.clone(), *choice.borrow()).await;

                if let Some(EngineNext::Content { id, context }) = engine_response.borrow().as_ref()
                {
                    if let ScriptContext::Directive(directive) = context {
                        match directive {
                            Directive::MusicPlay(musicplay) => {
                                music.set(musicplay.clone());
                            }
                            Directive::SoundPlay(soundplay) => {
                                sound.set(soundplay.clone());
                            }
                            Directive::Metadata(Metadata { key, value }) => {
                                log::info!("Received metadata of {key} and {value}");
                                metadata.borrow_mut().insert(key.clone(), value.clone());
                            }
                            _ => {}
                        }
                    }
                    if *id != 0 {
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

    let id = engine_response.borrow().as_ref().map(|res| {
        if let EngineNext::Content { id, .. } = res {
            *id
        } else {
            0
        }
    });

    let ended = *ended.borrow();
    let music_name = RE
        .captures_iter(&music.path)
        .last()
        .map(|captures| captures.get(1).unwrap().as_str())
        .unwrap_or("Nothing Currently Playing")
        .to_string();
    let name = metadata
        .borrow()
        .get("name")
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| "Not Provided".to_string());
    let description = metadata
        .borrow()
        .get("description")
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| "Not Provided".to_string());

    // should probably be include_bytes!
    let volume_icon = html! {
        <svg xmlns="http://www.w3.org/2000/svg" version="1.0"  width="24" height="24" viewBox="0 0 75 75" class="text-bordered" style="padding: 10px;">
            <path d="M39.389,13.769 L22.235,28.606 L6,28.606 L6,47.699 L21.989,47.699 L39.389,62.75 L39.389,13.769z"
            style="stroke:#646B82;stroke-width:5;stroke-linejoin:round;fill:#646B82;"
            />
            <path d="M48,27.6a19.5,19.5 0 0 1 0,21.4M55.1,20.5a30,30 0 0 1 0,35.6M61.6,14a38.8,38.8 0 0 1 0,48.6" style="fill:none;stroke:#646B82;stroke-width:5;stroke-linecap:round"/>
        </svg>
    };

    let game_icon = html! {
        <svg enable-background="new 0 0 512.549 512.549" version="1.1" viewBox="0 0 512.55 512.55" height="24px" width="24px" xmlns="http://www.w3.org/2000/svg" class="text-bordered" style="stroke:#646B82;stroke-width:5;stroke-linejoin:round;fill:#646B82;padding: 10px;">
            <g transform="translate(-1)">
                <path d="m214.61 213.6h-21.333v-21.333c0-11.782-9.551-21.333-21.333-21.333s-21.333 9.551-21.333 21.333v21.333h-21.333c-11.782 0-21.333 9.551-21.333 21.333s9.551 21.333 21.333 21.333h21.333v21.333c0 11.782 9.551 21.333 21.333 21.333s21.333-9.551 21.333-21.333v-21.333h21.333c11.782 0 21.333-9.551 21.333-21.333 1e-3 -11.782-9.551-21.333-21.333-21.333z"/>
                <path d="m500.92 269.31c-12.915-49.866-33.027-100.13-53.035-141.86-0.273-1.775-1.598-7.188-2.052-8.96-1.151-4.496-2.174-7.968-3.373-11.107-3.242-8.489-6.404-13.576-15.335-16.973l-62.11-23.598c-12.051-4.584-25.525-2.901-36.12 4.506l-2.105 1.472-1.696 1.93c-1.745 1.985-4.607 5.115-7.619 8.168-0.822 0.829-0.822 0.829-1.629 1.624-0.394 0.386-0.773 0.751-1.132 1.091h-114.38c-1.075-1.142-1.075-1.142-1.908-2.056-2.723-3.006-5.302-6.099-6.856-8.047l-1.925-2.413-2.53-1.768c-10.596-7.408-24.07-9.09-36.128-4.503l-62.092 23.591c-8.969 3.412-12.121 8.51-15.348 17.028-1.184 3.126-2.188 6.564-3.337 11.078-0.4 1.572-1.694 6.865-1.941 7.831-19.259 38.796-41.47 92.992-54.698 143.47-21.455 81.869-17.351 143.08 27.181 172.55 13.705 9.031 31.564 7.364 43.661-3.43l73.894-66.253c6.541-5.872 14.782-9.057 23.284-9.057h152.83c8.502 0 16.743 3.185 23.274 9.048l73.866 66.228c12.24 10.921 30.427 12.721 44.123 3.176 42.858-30.084 46.404-91.037 25.238-172.77zm-45.167 133.98-69.566-62.373c-14.316-12.852-32.697-19.956-51.767-19.956h-152.83c-19.07 0-37.45 7.104-51.777 19.965l-69.758 62.544c-19.658-18.052-21.296-61.46-5.213-122.83 12.346-47.111 33.478-98.672 51.546-134.31 1.399-2.752 2.112-5.086 3.217-9.411 0.319-1.249 1.62-6.572 1.952-7.876 0.16-0.629 0.314-1.22 0.461-1.772l50.992-19.374c1.186 1.381 2.463 2.836 3.793 4.303 1.113 1.222 1.113 1.222 2.273 2.457 9.927 10.492 13.691 13.62 24.204 13.62h128c10.147 0 13.785-2.861 24.417-13.27 1.094-1.079 1.094-1.079 2.155-2.148 1.719-1.742 3.365-3.468 4.858-5.073l51.321 19.499c0.151 0.56 0.308 1.159 0.471 1.797 0.382 1.493 1.69 6.836 1.931 7.787 1.066 4.206 1.73 6.435 2.992 9.058 18.908 39.428 38.104 87.404 50.193 134.08 15.8 61.011 14.632 104.61-3.863 123.28z"/>
                <path d="m342.61 192.27c11.776 0 21.333-9.557 21.333-21.333s-9.557-21.333-21.333-21.333-21.333 9.557-21.333 21.333 9.557 21.333 21.333 21.333z"/>
                <path d="m342.61 234.94c-11.776 0-21.333 9.557-21.333 21.333s9.557 21.333 21.333 21.333 21.333-9.557 21.333-21.333-9.557-21.333-21.333-21.333z"/>
                <path d="m299.94 192.27c-11.776 0-21.333 9.557-21.333 21.333s9.557 21.333 21.333 21.333 21.333-9.557 21.333-21.333-9.557-21.333-21.333-21.333z"/>
                <path d="m385.28 192.27c-11.776 0-21.333 9.557-21.333 21.333s9.557 21.333 21.333 21.333 21.333-9.557 21.333-21.333-9.557-21.333-21.333-21.333z"/>
            </g>
        </svg>
    };

    let preview = {
        let id = id;

        use_async(async move { get_preview(id.unwrap_or(0)).await })
    };

    {
        let preview = preview.clone();
        let id = id;
        use_effect_with_deps(
            move |_| {
                preview.run();

                || ()
            },
            id,
        );
    }

    {
        let preview = preview;

        use_effect_with_deps(
            move |preview| {
                if let Some(preview) = &preview.data {
                    let preview_element: HtmlImageElement = gloo::utils::document()
                        .get_element_by_id("play-preview")
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
        <div class="dflex dflex-grow-1 dflex-gap-md" style="justify-content: center;">
            <PlayMusic path={music.path.to_string()} volume={music.volume.unwrap_or(1.0)} />
            <PlaySound path={sound.path.to_string()} volume={sound.volume.unwrap_or(1.0)} />

            if !ended {
                if let Some(_id) = id {
                    <div>
                        <img alt="preview" id="play-preview" src="" style="border-width: 2px; border-style: solid; border-color: #181B2B"/>
                    </div>
                    <div class="dflex dflex-col dflex-justify-between">
                        <div class="dflex dflex-col dflex-gap-sm">
                            <IconText icon={game_icon} text={name}/>
                            <IconText icon={VNode::default()} text={"Description".to_string()}/>
                            <div class="text-bordered" style="min-height: 10rem; max-width: 400px;">{description}</div>
                            <IconText icon={volume_icon} text={music_name}/>
                        </div>
                    </div>
                } else {
                    <p>{"Press <enter> to start!"}</p>
                }

            } else {
                <p>{"Thank you for playing!"}</p>
            }
        </div>
    }
}
