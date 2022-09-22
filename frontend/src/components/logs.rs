use lazy_static::lazy_static;
use regex::Regex;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub logs: String,
}

#[function_component(Logs)]
pub fn logs(props: &Props) -> Html {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(\S*)\s(\S*)\s+(\S*)\]\s+([^\n\[]*)").unwrap();
    }

    let logs = RE.captures_iter(&props.logs).map(|capture| {
        let level = capture.get(2).unwrap().as_str();
        let message = capture.get(4).unwrap().as_str();

        let color = match level {
            "DEBUG" => "blue",
            "INFO" => "green",
            _ => "yellow",
        };
        html! {
            <>
            <div style={format!("background-color: {color}; color: white; padding: 5px 10px; border-radius: 16px; text-align: center; vertical-align: middle;")}>
                    {level}
            </div>
            <div class="dflex dflex-justify-center">{message}</div>
            </>
            
        }
    });
    
    html! {
        <div class="dflex dflex-col dflex-gap-sm">
            <div class="bold" style="color: white; margin-bottom: 1rem;">
                {"Logs"}
            </div>
            <div style="display: grid; grid-template-columns: auto 1fr; gap: 0.5rem;">
                {for logs}
            </div>
        </div>
    }
}
