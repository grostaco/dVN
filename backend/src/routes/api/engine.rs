use image_rpg::core::engine::Engine;
use rocket::serde::json::Json;
use std::sync::Mutex;

use backend_types::*;
use rocket::State;

#[post("/engine/init?<script>")]
pub fn engine_init(script: String, state: &State<Mutex<Option<Engine>>>) -> Result<EngineInit> {
    let mut guard = state.lock().unwrap();

    *guard = Some(match Engine::new(&script) {
        Ok(engine) => engine,
        Err(e) => {
            return Json(ErrorOr::Error(Error {
                message: e.to_string(),
            }))
        }
    });

    Json(ErrorOr::Response(()))
}

#[post("/engine/next?<choice>")]
pub fn engine_next(choice: bool, state: &State<Mutex<Option<Engine>>>) -> Result<EngineNext> {
    let mut guard = state.lock().unwrap();

    let engine = guard.as_mut().unwrap();

    engine.set_choice(choice);
    match engine.next() {
        Some(context) => match context {
            Ok(context) => Json(ErrorOr::Response(EngineNext::Content {
                id: engine.render("assets/rendered/.cache").unwrap_or(0),
                context,
            })),
            Err(e) => Json(ErrorOr::Error(Error {
                message: e.cause.to_string(),
            })),
        },
        None => Json(ErrorOr::Response(EngineNext::Ended)),
    }
}

// #[post("/engine/next?<choice>")]
// pub fn engine_next(
//     choice: bool,
//     state: &State<Mutex<Option<Engine>>>,
// ) -> Result<Option<Json<EngineResponse>>, rocket::response::Debug<Error>> {
//     let mut guard = state.lock().unwrap();

//     let engine = guard.as_mut().unwrap();

//     engine.set_choice(choice);
//     let context = engine.next().transpose()?;
//     if context.is_none() {
//         return Ok(None);
//     }

//     let id = engine.render("assets/rendered/.cache").unwrap_or(0);
//     Ok(Some(Json(EngineResponse {
//         id,
//         context: context.unwrap(),
//     })))
// }
