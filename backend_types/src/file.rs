use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileFiles {
    pub files: Vec<String>,
}

pub type FilePost = ();

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilePostRequest {
    pub content: String,
}
