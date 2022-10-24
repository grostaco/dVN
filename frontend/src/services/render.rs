use backend_types::{RenderPreview, RenderResult, RenderResultRequest};

use super::{error::Result, requests::request};

pub async fn get_preview(id: u64) -> Result<RenderPreview> {
    request!(get -> &format!("/api/rendered/{id}/preview.png")).await
}
pub async fn post_render(script: String) -> Result<RenderResult> {
    let render = request!(post -> "/api/render" ; RenderResultRequest {
        data: script.into_bytes(),
    } )
    .await;
    render
}

pub async fn delete_cache() {
    request!(delete -> "/api/render").await.unwrap()
}
