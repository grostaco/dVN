use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlAudioElement;
use yew::prelude::*;
use yew_hooks::use_effect_update_with_deps;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub path: String,
    pub volume: f64,
}

#[function_component(PlayMusic)]
pub fn play_music(props: &Props) -> Html {
    let music = props.path.clone();

    let volume = props.volume;
    {
        let music = music;
        use_effect_update_with_deps(
            move |music| {
                let element = HtmlAudioElement::new_with_src(music.as_str()).unwrap();
                element.set_loop(true);
                element.set_volume(volume);
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

    html! {}
}
