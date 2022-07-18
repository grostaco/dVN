use std::{path::Path, fs::{self, OpenOptions}, io::{Write, self}, sync::Mutex};
use image_rpg::core::engine::Engine;
use rocket::tokio::fs::{remove_dir, create_dir};
use serde::Serialize;
use serde_json::json;

#[macro_use] extern crate rocket;

// struct EngineWrapper {
//     engine: Mutex<Option<Engine>>,
// }

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RenderResult {
    hashed_ids: Vec<u64>,
}

#[get("/api/rendered/<id>/preview.png")]
fn image_preview(id: u64) -> std::io::Result<Vec<u8>> {
    let p = Path::new("../assets/rendered/").join(id.to_string()).with_extension("png");
    fs::read(p)
}

#[post("/api/render", data = "<content>")]
fn render(content: &str) -> String {
    let p = Path::new("../assets/autogen_script.script");
    println!("Content:\n{}", content);

    OpenOptions::new().create(true).write(true).truncate(true).open(p).unwrap().write_all(content.as_bytes()).unwrap();
    let mut engine = match Engine::new(p.to_str().unwrap()) {
        Ok(engine) => engine,
        Err(e) => return json!({"code": 400, "reason": e.to_string()}).to_string(), 
    };
    let mut ids = Vec::new();

    while let Some(result) = engine.next() {
        match result {
            Ok(_) => {
                if let Some(hsh) = engine.render("../assets/rendered/.cache") { ids.push(hsh) }
            }
            Err(e) => return json!({"code": 400, "reason": e.to_string()}).to_string(),
        }
    }

    json!({"code": 200, "identifiers": ids}).to_string()
}

#[delete("/api/render")]
async fn clear_rendered() -> io::Result<()> {
    remove_dir("../assets/rendered/.cache").await?;
    create_dir("../assets/rendered/.cache").await?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    //.manage(EngineWrapper { engine: Mutex::new(None) })
    .mount("/", routes![image_preview, render, clear_rendered])
}