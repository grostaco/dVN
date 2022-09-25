use image_rpg::core::engine::Engine;
use rocket::{
    serde::json::serde_json::json,
    tokio::fs::{create_dir, remove_dir_all},
};
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};
use walkdir::WalkDir;

#[get("/files")]
pub fn files() -> String {
    WalkDir::new("assets")
        .into_iter()
        .filter_map(|e| {
            e.ok().map(|entry| {
                entry
                    .path()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .replace('\\', "/")
            })
        })
        .reduce(|a, b| a + "," + &b)
        .unwrap_or_default()
}

#[post("/file/assets/<path>", data = "<content>")]
pub fn post_file(path: String, content: &str) {
    OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(format!("assets/{}", path))
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}

#[get("/rendered/<id>/preview.png")]
pub fn image_preview(id: u64) -> std::io::Result<Vec<u8>> {
    let p = Path::new("assets/rendered/.cache")
        .join(id.to_string())
        .with_extension("png");
    fs::read(p)
}

#[post("/render", data = "<content>")]
pub fn render(content: &str) -> String {
    let p = Path::new("assets/autogen_script.script");
    // fs::remove_file("log/requests.log").ok();
    //

    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(p)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
    let mut engine = match Engine::new(p.to_str().unwrap()) {
        Ok(engine) => engine,
        // 300 is most definitely NOT the right code
        Err(e) => return json!({"code": 300, "reason": e.to_string()}).to_string(),
    };
    let mut ids = Vec::new();
    while let Some(result) = engine.next() {
        match result {
            Ok(_) => {
                if let Some(hsh) = engine.render("assets/rendered/.cache") {
                    ids.push(hsh)
                }
            }
            Err(_) => return json!({"code": 300, "data": [], "log": read_consume("log/requests.log").as_bytes()}).to_string(),
        }
    }
    json!({"code": 200, "data": ids, "log": read_consume("log/requests.log").as_bytes()})
        .to_string()
}

#[delete("/render")]
pub async fn clear_rendered() -> io::Result<()> {
    remove_dir_all("assets/rendered/.cache").await.unwrap();
    create_dir("assets/rendered/.cache").await.unwrap();

    Ok(())
}

fn read_consume<P: AsRef<Path>>(path: P) -> String {
    let log = fs::read_to_string(path).unwrap();
    fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .read(true)
        .open("log/requests.log")
        .unwrap();
    log
}
