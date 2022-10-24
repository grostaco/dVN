use std::{fs::OpenOptions, io::Write, path::PathBuf};

use backend_types::{FileFiles, FilePost, FilePostRequest};
use rocket::serde::json::Json;
use walkdir::WalkDir;

#[get("/files")]
pub fn files() -> Json<FileFiles> {
    let files = WalkDir::new("assets")
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
        .collect();

    Json(FileFiles { files })
}

#[post("/file/assets/<path..>", format = "json", data = "<req>")]
pub fn post_file(path: PathBuf, req: Json<FilePostRequest>) -> Json<FilePost> {
    OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(format!("assets/{}", path.display()))
        .unwrap()
        .write_all(req.0.content.as_bytes())
        .unwrap();

    Json(())
}
