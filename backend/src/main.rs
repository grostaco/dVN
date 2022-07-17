use std::{path::Path, fs::{self, OpenOptions}, io::Write, sync::Mutex};
use image_rpg::core::engine::Engine;
use rocket::{State, serde::json::Json, http::Status, response::status};
use serde::Serialize;
use serde_json::json;

#[macro_use] extern crate rocket;

struct EngineWrapper {
    engine: Mutex<Option<Engine>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RenderResult {
    hashed_ids: Vec<u64>,
}

#[get("/api/rendered/<id>/preview.png")]
fn image_preview(id: u64) -> std::io::Result<Vec<u8>> {
    let p = Path::new("../assets/rendered/.cache").join(id.to_string()).with_extension("png");
    fs::read(p)
}

#[get("/api/render", data = "<content>")]
fn render(engine_wrapper: &State<EngineWrapper>, content: &str) -> String {
    let p = Path::new("../assets/autogen_script.script");
    let mut engine = engine_wrapper.engine.lock().unwrap();

    OpenOptions::new().write(true).truncate(true).open(p).unwrap().write_all(content.as_bytes()).unwrap();
    engine.replace(Engine::new(p.to_str().unwrap()).unwrap());

    json!({"code": 200, "identifiers": vec![1, 2, 3]}).to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(EngineWrapper { engine: Mutex::new(None) })
    .mount("/", routes![image_preview, render])
}