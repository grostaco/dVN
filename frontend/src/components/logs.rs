use lazy_static::lazy_static;
use regex::Regex;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub logs: Vec<u8>,
}

#[function_component(Logs)]
pub fn logs(props: &Props) -> Html {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(\S*)\s(\S*)\s+(\S*)\]\s+([^\n\[]*)").unwrap();
    }
    let logs_string = String::from_utf8(props.logs.clone()).unwrap();

    let logs = RE.captures_iter(&logs_string).map(|capture| {
        let level = capture.get(2).unwrap().as_str();
        let message = capture.get(4).unwrap().as_str();

        let color = match level {
            "DEBUG" => "blue",
            "INFO" => "green",
            "WARNING" => "yellow",
            "ERROR" => "red",
            _ => "purple",
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
