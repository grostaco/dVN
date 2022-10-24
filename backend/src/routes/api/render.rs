use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use backend_types::*;
use image_rpg::core::engine::Engine;
use rocket::{
    serde::json::Json,
    tokio::fs::{create_dir, remove_dir_all},
};

#[get("/rendered/<id>/preview.png")]
pub fn image_preview(id: u64) -> Result<RenderPreview> {
    let p = Path::new("assets/rendered/.cache")
        .join(id.to_string())
        .with_extension("png");
    let bytes = fs::read(p);

    match bytes {
        Ok(bytes) => ErrorOr::json_response_from(RenderPreview { bytes }),
        Err(e) => ErrorOr::json_error_from(e),
    }
}

#[post("/render", format = "json", data = "<content>")]
pub fn render(content: Json<RenderResultRequest>) -> Result<RenderResult> {
    let p = Path::new("assets/autogen_script.script");
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(p)
        .unwrap()
        .write_all(&content.0.data)
        .unwrap();
    let mut engine = match Engine::new(p.to_str().unwrap()) {
        Ok(engine) => engine,
        Err(e) => return ErrorOr::json_error_from(e),
    };
    let mut ids = Vec::new();
    while let Some(result) = engine.next() {
        match result {
            Ok(_) => {
                if let Some(hsh) = engine.render("assets/rendered/.cache") {
                    ids.push(hsh)
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    ErrorOr::json_response_from(RenderResult {
        data: ids,
        log: read_consume("log/requests.log").as_bytes().to_vec(),
    })
}

#[delete("/render")]
pub async fn clear_rendered() -> Result<RenderClear> {
    if let Err(e) = remove_dir_all("assets/rendered/.cache").await {
        return ErrorOr::json_error_from(e);
    }
    if let Err(e) = create_dir("assets/rendered/.cache").await {
        return ErrorOr::json_error_from(e);
    }

    ErrorOr::json_response_from(())
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
