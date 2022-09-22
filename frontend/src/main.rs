use app::App;

mod app;
mod components;
mod error;
pub mod services;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<App>();
}
