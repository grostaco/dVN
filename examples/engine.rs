use image_rpg::core::engine::Engine;

fn main() {
    env_logger::init();
    let mut engine = Engine::new("test_script.script").unwrap();

    while engine.next().is_some() {  
        engine.render("assets/rendered");
    }
}