use std::{path::Path, fs::{self, OpenOptions}, io::{Write, self}};
use image_rpg::core::engine::Engine;
use log4rs::{append::{console::ConsoleAppender, file::FileAppender}, encode::pattern::PatternEncoder, Config, config::{Appender, Root, Logger}};
use rocket::{tokio::fs::{create_dir, remove_dir_all}, fairing::{Fairing, Kind, Info}, Request, Response, http::Header, serde::{Serialize, json::serde_json::json}};

#[macro_use] extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RenderResult {
    hashed_ids: Vec<u64>,
}


#[get("/api/rendered/<id>/preview.png")]
fn image_preview(id: u64) -> std::io::Result<Vec<u8>> {
    let p = Path::new("assets/rendered/.cache").join(id.to_string()).with_extension("png");
    fs::read(p)
}

#[post("/api/render", data = "<content>")]
fn render(content: &str) -> String {
    let p = Path::new("assets/autogen_script.script");
    println!("Content:\n{}", content);

    OpenOptions::new().create(true).write(true).truncate(true).open(p).unwrap().write_all(content.as_bytes()).unwrap();
    let mut engine = match Engine::new(p.to_str().unwrap()) {
        Ok(engine) => engine,
        // 300 is most definitely NOT the right code
        Err(e) => return json!({"code": 300, "reason": e.to_string()}).to_string(), 
    };
    let mut ids = Vec::new();

    while let Some(result) = engine.next() {
        match result {
            Ok(_) => {
                if let Some(hsh) = engine.render("assets/rendered/.cache") { ids.push(hsh) }
            }
            Err(e) => return json!({"code": 300, "reason": e.to_string()}).to_string(),
        }
    }

    json!({"code": 200, "data": ids, "log": fs::read_to_string("log/requests.log").unwrap().as_bytes()}).to_string()
}

#[options("/api/render")]
async fn options() {}

#[delete("/api/render")]
async fn clear_rendered() -> io::Result<()> {
    remove_dir_all("assets/rendered/.cache").await.unwrap();
    create_dir("assets/rendered/.cache").await.unwrap();

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
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, PUT, POST, DELETE, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// This program should be run with cargo -p
#[launch]
fn rocket() -> _ {

    let stdout = ConsoleAppender::builder().build();

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{d} {l}  {M}] {m}\n")))
        .append(false)
        .build("log/requests.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder()
            .appender("requests")
            .additive(false)
            .build("image_rpg", log::LevelFilter::Debug))
        .build(Root::builder().appender("stdout").build(log::LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();

    rocket::build()
    .attach(CORS)
    //.manage(EngineWrapper { engine: Mutex::new(None) })
    .mount("/", routes![image_preview, render, clear_rendered, options])
}