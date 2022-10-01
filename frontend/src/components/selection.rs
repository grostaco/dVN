use reqwest::Client;
use yew::prelude::*;
use yew_hooks::use_async;

use super::{Button, FileView, TextInput};
use crate::services::files::get_file;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub onselect: Callback<String>,
}

#[function_component(Selection)]
pub fn selection(props: &Props) -> Html {
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
