use std::{path::Path, fs::{self, OpenOptions}, io::{Write, self}};
use image_rpg::core::engine::Engine;
use rocket::{tokio::fs::{remove_dir, create_dir}, fairing::{Fairing, Kind, Info}, Request, Response, http::Header, serde::{Serialize, json::serde_json::json}};

#[macro_use] extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RenderResult {
    hashed_ids: Vec<u64>,
}

#[derive(Responder)]
#[response(status = 200)]
struct RenderResponder {
    data: String,
}

#[get("/api/rendered/<id>/preview.png")]
fn image_preview(id: u64) -> std::io::Result<Vec<u8>> {
    let p = Path::new("assets/rendered/.cache").join(id.to_string()).with_extension("png");
    fs::read(p)
}

#[post("/api/render", data = "<content>")]
fn render(content: &str) -> RenderResponder {
    let p = Path::new("assets/autogen_script.script");
    println!("Content:\n{}", content);

    OpenOptions::new().create(true).write(true).truncate(true).open(p).unwrap().write_all(content.as_bytes()).unwrap();
    let mut engine = match Engine::new(p.to_str().unwrap()) {
        Ok(engine) => engine,
        // 300 is most definitely NOT the right code
        Err(e) => return RenderResponder { data: json!({"code": 300, "reason": e.to_string()}).to_string() }, 
    };
    let mut ids = Vec::new();

    while let Some(result) = engine.next() {
        match result {
            Ok(_) => {
                if let Some(hsh) = engine.render("assets/rendered/.cache") { ids.push(hsh) }
            }
            Err(e) => return RenderResponder { data: json!({"code": 300, "reason": e.to_string()}).to_string() },
        }
    }

    RenderResponder { data: json!({"code": 200, "data": ids}).to_string() }
}

#[delete("/api/render")]
async fn clear_rendered() -> io::Result<()> {
    remove_dir("assets/rendered/.cache").await?;
    create_dir("assets/rendered/.cache").await?;

    Ok(())
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// This program should be run with cargo -p
#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(CORS)
    //.manage(EngineWrapper { engine: Mutex::new(None) })
    .mount("/", routes![image_preview, render, clear_rendered])
}