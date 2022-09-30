use image_rpg::core::engine::Engine;
use image_rpg::parser::{error::Error, script::ScriptContext};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use std::sync::Mutex;

use rocket::State;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EngineResponse {
    id: u64,
    context: ScriptContext,
}

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
) -> Result<Option<Json<EngineResponse>>, rocket::response::Debug<Error>> {
    let mut guard = state.lock().unwrap();

    let engine = guard.as_mut().unwrap();
    engine.set_choice(choice);
    let context = engine.next().transpose()?;
    if context.is_none() {
        return Ok(None);
    }

    let id = engine.render("assets/rendered/.cache").unwrap();
    Ok(Some(Json(EngineResponse {
        id,
        context: context.unwrap(),
    })))
}
