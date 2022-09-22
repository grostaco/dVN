use reqwest::Client;

use yew::{
    function_component, html, use_effect_with_deps, use_mut_ref, use_ref, use_state, Callback,
};
use yew_hooks::use_async;

use crate::services::render::{post_render, RenderResult};

use super::components::*;

#[function_component(App)]
pub fn app() -> Html {
    let render_result = use_state(RenderResult::default);
    let editor_onchange = {
        let render_result = render_result.clone();
        Callback::from(move |result| render_result.set(result))
    };
    // let client = use_ref(Client::new);
    // let script = use_mut_ref(String::new);
    // let log = use_state(String::new);
    // let ids = use_mut_ref(Vec::new);

    // let to_compile = use_state(String::new);

    // let render = {
    //     let to_compile = to_compile.clone();
    //     let log = log.clone();
    //     let ids = ids.clone();
    //     use_async(async move {
    //         let content = post_render(client, (*to_compile).to_string()).await;
    //         *ids.borrow_mut() = content.data;
    //         log.set(String::from_utf8(content.log).unwrap());
    //         Ok::<_, ()>(())
    //     })
    // };

    // let compile = {
    //     let to_compile = to_compile.clone();
    //     let script = script.clone();
    //     Callback::from(move |_| {
    //         to_compile.set(script.borrow().to_string());
    //     })
    // };

    // {
    //     let to_compile = to_compile;
    //     use_effect_with_deps(
    //         move |_| {
    //             render.run();
    //             || ()
    //         },
    //         to_compile,
    //     );
    // }

    html! {
        <>
        <Nav/>
        <div class="main dflex-gap-md">
            <div class="dflex dflex-row dflex-justify-between">
                <Preview data={render_result.data.clone()}/>
                <Editor data_cb={editor_onchange}/>
            </div>
            <Logs logs={render_result.log.clone()}/>
        </div>
        </>
    }
}

// <div class="text-edit dflex-gap-sm">
//     <TextInput on_change={script}/>
//     <div>
//         <Button label="Compile" onclick={compile}/>
//     </div>
// </div>
// <div class="dflex dflex-row">
//     if to_compile.is_empty()  {
//         <p>{"Nothing rendered yet!"}</p>
//     } else {
//         <img alt="preview" src={format!("http://127.0.0.1:8000/api/rendered/{}/preview.png", ids.borrow().get(*index).unwrap_or(&0))}/>
//         <div class="dflex dflex-gap-sm" style="flex-direction: column-reverse;">
//             <Button label="Clear Cache" onclick={compile.clone()}/>
//             <Button label="Prev" onclick={prev}/>
//             <Button label="Next" onclick={next}/>
//         </div>
//     }
// </div>
//         html! {
//             <>
//             <Nav/>
//             <div class="main dflex-gap-md">
//                 <div class="dflex dflex-row dflex-justify-between" id="editor">
//                     if self.rendered.is_empty()  {
//                         <p>{"Nothing rendered yet!"}</p>
//                     } else {
//                         <div class="dflex dflex-row">
//                             <img alt="preview" src={format!("http://127.0.0.1:8000/api/rendered/{}/preview.png", content.unwrap())}/>
//                             <div class="dflex dflex-gap-sm" style="flex-direction: column-reverse;">
//                                 <ButtonInput maxlen={self.rendered.len()} onclick={link.callback(Msg::NewIndex)}/>
//                                 <button class="btn" onclick={link.callback(|_| Msg::ClearCache)}>{"Clear Cache"}</button>
//                             </div>
//                         </div>
//                     }
//                     <div class="text-edit dflex-gap-sm">
//                         <TextInput {on_change} value={self.script_text.clone()}/>
//                         <div>
//                             <button class="btn" {onclick}>{"Compile"}</button>
//                         </div>
//                     </div>
//                 </div>
//                 <Logs logs={self.log.clone()}/>
//             </div>
//             </>
//         }
//     }
// }

// <div class="bold" style="color: white; margin-bottom: 1rem;">
//     {"Logs"}
// </div>
// <div class="dflex dflex-row dflex-justify-center dflex-gap-sm">
//     <div style="background-color: green; color: white; padding: 5px 10px; border-radius: 16px;">
//         {"INFO"}
//     </div>
//     <div>{"Parsed dialogue \"A: A\""}</div>
// </div>

// pub enum Msg {
//     Submit,
//     NewIndex(usize),
//     SetScript(String),
//     UpdateIndex(Vec<u64>),
//     UpdateLog(String),
//     ClearCache,
// }

// #[derive(Debug, Default)]
// pub struct App {
//     client: Rc<Client>,
//     script_text: String,
//     index: usize,
//     rendered: Vec<u64>,
//     log: String,
// }

// impl Component for App {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {
//             script_text: String::new(),
//             client: Rc::new(Client::new()),
//             rendered: Vec::new(),
//             index: 0,
//             log: String::new(),
//         }
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::SetScript(s) => {
//                 self.script_text = s;
//                 false
//             }
//             Msg::Submit => {
//                 info!("Sent a request");
//                 let client = self.client.clone();
//                 let script_text = self.script_text.clone();
//                 let link = ctx.link().clone();
//                 spawn_local(async move {
//                     let content = client
//                         .post("http://127.0.0.1:8000/api/render")
//                         .body(script_text)
//                         .send()
//                         .await
//                         .unwrap()
//                         .text()
//                         .await
//                         .unwrap();

//                     let content: Value = serde_json::from_str(&content).unwrap();
//                     info!("Received code {} from render endpoint", content["code"]);
//                     if content["code"].as_u64().unwrap() == 200 {
//                         let ids = content["data"]
//                             .as_array()
//                             .unwrap()
//                             .iter()
//                             .map(|x| x.as_u64().unwrap())
//                             .collect::<Vec<_>>();
//                         link.send_message(Msg::UpdateIndex(ids));
//                         link.send_message(Msg::UpdateLog(
//                             String::from_utf8(
//                                 content["log"]
//                                     .as_array()
//                                     .unwrap()
//                                     .iter()
//                                     .map(|v| v.as_u64().unwrap() as u8)
//                                     .collect(),
//                             )
//                             .unwrap(),
//                         ));
//                     }
//                 });

//                 false
//             }
//             Msg::UpdateIndex(ids) => {
//                 self.rendered = ids;
//                 true
//             }
//             Msg::NewIndex(index) => {
//                 self.index = index;
//                 true
//             }
//             Msg::UpdateLog(l) => {
//                 self.log = l;
//                 true
//             }
//             Msg::ClearCache => {
//                 let client = self.client.clone();
//                 let link = ctx.link().clone();
//                 spawn_local(async move {
//                     client
//                         .delete("http://127.0.0.1:8000/api/render")
//                         .send()
//                         .await
//                         .unwrap();
//                     link.send_message(Msg::Submit);
//                 });
//                 false
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let index = self.index;

//         let link = ctx.link();
//         let on_change = link.callback(Msg::SetScript);
//         let onclick = link.callback(|_| Msg::Submit);

//         let content = self.rendered.get(index).map(ToString::to_string);

//         html! {
//             <>
//             <Nav/>
//             <div class="main dflex-gap-md">
//                 <div class="dflex dflex-row dflex-justify-between" id="editor">
//                     if self.rendered.is_empty()  {
//                         <p>{"Nothing rendered yet!"}</p>
//                     } else {
//                         <div class="dflex dflex-row">
//                             <img alt="preview" src={format!("http://127.0.0.1:8000/api/rendered/{}/preview.png", content.unwrap())}/>
//                             <div class="dflex dflex-gap-sm" style="flex-direction: column-reverse;">
//                                 <ButtonInput maxlen={self.rendered.len()} onclick={link.callback(Msg::NewIndex)}/>
//                                 <button class="btn" onclick={link.callback(|_| Msg::ClearCache)}>{"Clear Cache"}</button>
//                             </div>
//                         </div>
//                     }
//                     <div class="text-edit dflex-gap-sm">
//                         <TextInput {on_change} value={self.script_text.clone()}/>
//                         <div>
//                             <button class="btn" {onclick}>{"Compile"}</button>
//                         </div>
//                     </div>
//                 </div>
//                 <Logs logs={self.log.clone()}/>
//             </div>
//             </>
//         }
//     }
// }

// <div class="bold" style="color: white; margin-bottom: 1rem;">
//     {"Logs"}
// </div>
// <div class="dflex dflex-row dflex-justify-center dflex-gap-sm">
//     <div style="background-color: green; color: white; padding: 5px 10px; border-radius: 16px;">
//         {"INFO"}
//     </div>
//     <div>{"Parsed dialogue \"A: A\""}</div>
// </div>
