use std::path::Path;

use backend_types::{FileFiles, FilePost, FilePostRequest};
use reqwest::Client;
use web_sys::MouseEvent;
use yew::{html, Callback, Html};

use super::{error::Result, requests::request};

pub async fn get_files() -> Result<FileFiles> {
    let res = request!(get -> "/api/files").await;
    res
}

pub async fn get_file(file: &str) -> String {
    Client::new()
        .get(format!("http://127.0.0.1:8000/{file}"))
        .header("Content-Length", 8192)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

pub async fn post_file(file: &str, content: String) -> Result<FilePost> {
    request!(post -> &format!("/api/file/{file}") ; FilePostRequest { content } ).await
}

pub fn file_tree(
    files: Vec<&'_ Path>,
    expand_callback: &Callback<MouseEvent>,
    file_callback: &Callback<MouseEvent>,
) -> Html {
    let mut folders = Vec::new();
    let mut folder_files = Vec::new();

    let mut i = 1;
    while i < files.len() {
        let file = files.get(i).unwrap();
        if file.extension().is_none() {
            let sub_files = files
                .iter()
                .skip(i)
                .take_while(|p| p.starts_with(file))
                .copied()
                .collect::<Vec<_>>();
            i += sub_files.len() - 1;
            folders.push(file_tree(sub_files, expand_callback, file_callback));
        } else {
            folder_files.push(
                html! { <div path={file.to_str().unwrap().to_string()} onclick={file_callback}>{file.file_name().unwrap().to_str().unwrap()}</div> },
            );
        }
        i += 1;
    }

    html! {
        <>
        if let Some(file) = files.first() {
            <div class="name" onclick={expand_callback}>{ file.file_name().unwrap().to_str().unwrap() }</div>
            <div class="children">
                if !folders.is_empty() {
                    <div class="folder">
                        {for folders}
                    </div>
                }

                if !folder_files.is_empty() {
                    <div class="files">
                        {for folder_files}
                    </div>
                }
            </div>
        }
        </>
    }
}
