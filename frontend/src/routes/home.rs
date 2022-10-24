use backend_types::RenderResult;
use yew::{function_component, html, use_state, Callback};

use crate::components::{Editor, Logs, Nav, Preview};
#[function_component(Home)]
pub fn home() -> Html {
    let render_result = use_state(|| RenderResult {
        data: Vec::new(),
        log: Vec::new(),
    });
    let editor_onchange = {
        let render_result = render_result.clone();
        Callback::from(move |result| render_result.set(result))
    };

    html! {
        <>
        <Nav/>
        <div class="main dflex-gap-md">
            <div class="dflex dflex-row dflex-justify-between dflex-gap-sm">
                <Preview data={render_result.data.clone()}/>
                <Editor data_cb={editor_onchange}/>
            </div>
            <Logs logs={render_result.log.clone()}/>
        </div>
        </>
    }
}
