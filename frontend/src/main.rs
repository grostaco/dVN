use app::App;

mod app;
mod nav;
mod scene_control;
mod text_input;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<App>();
}
