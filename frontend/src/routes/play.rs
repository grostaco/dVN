use crate::{
    components::{Button, FileView, Nav, TextInput},
    services::{
        engine::{init_engine, next_engine},
        files::get_file,
    },
};
use reqwest::Client;
use yew::{function_component, html, use_mut_ref, use_ref, Callback, Properties};
use yew_hooks::use_async;

#[function_component(Play)]
pub fn play() -> Html {
    let client = use_ref(Client::new);
    let choice = use_mut_ref(bool::default);
    let script_file = use_mut_ref(String::new);
    let engine_response = use_mut_ref(Option::default);

    let init_engine = {
        let client = client.clone();
        let script_file = script_file.clone();
        use_async(async move {
            init_engine(client, script_file.borrow().to_string()).await;
            Ok::<_, ()>(())
        })
    };

    let next_engine = {
        let client = client;
        let engine_response = engine_response.clone();
        use_async(async move {
            *engine_response.borrow_mut() = next_engine(client, *choice.borrow()).await;
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

    let next = {
        Callback::from(move |_| {
            next_engine.run();
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
                <img alt="preview" src={format!("http://127.0.0.1:8000/api/rendered/{}/preview.png", engine_response.borrow().as_ref().map(|x| x.id).unwrap_or(0))}/>
                <Button onclick={next} label="Next"/>
            }
        </div>
        </>
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
