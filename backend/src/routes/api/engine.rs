use image_rpg::core::engine::Engine;
use image_rpg::parser::{error::Error, script::ScriptContext};
use rocket::serde::json::Json;
use std::sync::Mutex;

use rocket::State;

#[post("/engine/init?<script>")]
pub fn engine_init(
    script: String,
    state: &State<Mutex<Option<Engine>>>,
) -> Result<(), std::io::Error> {
    let mut guard = state.lock().unwrap();

    *guard = Some(Engine::new(&script)?);

    Ok(())
}

#[post("/engine/next?<choice>")]
pub fn engine_next(
    choice: bool,
    state: &State<Mutex<Option<Engine>>>,
) -> Result<Option<Json<ScriptContext>>, rocket::response::Debug<Error>> {
    let mut guard = state.lock().unwrap();

    let engine = guard.as_mut().unwrap();
    engine.set_choice(choice);
    Ok(engine.next().transpose()?.map(Json))
}
