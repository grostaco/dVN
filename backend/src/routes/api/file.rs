use std::{fs::OpenOptions, io::Write, path::PathBuf};

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

#[post("/file/assets/<path..>", data = "<content>")]
pub fn post_file(path: PathBuf, content: &str) {
    OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(format!("assets/{}", path.display()))
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}
