use app::App;

mod scene_control;
mod text_input;
mod app;

fn main () {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<App>();
}
