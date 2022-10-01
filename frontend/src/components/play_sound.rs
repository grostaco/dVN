use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlAudioElement;
use yew::prelude::*;
use yew_hooks::use_effect_update_with_deps;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub path: String,
}

#[function_component(PlaySound)]
pub fn play_sound(props: &Props) -> Html {
    let sound = props.path.clone();
    use_effect_update_with_deps(
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

    html! {}
}
