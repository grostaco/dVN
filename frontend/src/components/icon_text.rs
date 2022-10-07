use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub icon: VNode,
    pub text: String,
}

#[function_component(IconText)]
pub fn icon_text(props: &Props) -> Html {
    html! {
        <div class="dflex dflex-col dflex-gap-sm">
            <div class="dflex dflex-row dflex-gap-sm">
                {props.icon.clone()}
                <div class="text-bordered">
                    {props.text.clone()}
                </div>
            </div>
        </div>
    }
}

// <svg xmlns="http://www.w3.org/2000/svg" version="1.0"  width="500" height="500" viewBox="0 0 75 75">
//                     <path d="M39.389,13.769 L22.235,28.606 L6,28.606 L6,47.699 L21.989,47.699 L39.389,62.75 L39.389,13.769z"
//                     style="stroke:#111;stroke-width:5;stroke-linejoin:round;fill:#111;"
//                     />
//                     <path d="M48,27.6a19.5,19.5 0 0 1 0,21.4M55.1,20.5a30,30 0 0 1 0,35.6M61.6,14a38.8,38.8 0 0 1 0,48.6" style="fill:none;stroke:#111;stroke-width:5;stroke-linecap:round"/>
//                 </svg>
