use std::fmt::Debug;

use axum::{Extension, body::StreamBody, response::{IntoResponse, AppendHeaders}};
use http::header::{CONTENT_TYPE, CONTENT_DISPOSITION};
use hyper::StatusCode;
use tokio_util::io::ReaderStream;
use tracing::{info, error};

use crate::service::compilation::CompilationService;

#[tracing::instrument]
pub async fn post_compile<T>(Extension(service): Extension<T>, raw_text: String) -> Result<impl IntoResponse, impl IntoResponse>
where 
    T: CompilationService + Debug,
    <T as CompilationService>::CompileOptions: From<String>,
    <T as CompilationService>::CompilationError: Into<String>
{
    info!("Received compilation attempt");
    let path = match service.compile(raw_text.into()).await {
        Ok(path) => path,
        Err(err) => {
            error!(?err);
            return Err((StatusCode::UNPROCESSABLE_ENTITY, err.into()));
        }
    };

    let file = tokio::fs::File::open(&path).await.unwrap();

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let headers = AppendHeaders([
        (CONTENT_TYPE, "application/pdf"),
        (CONTENT_DISPOSITION, "inline")
    ]);

    info!("Compiled file {:?}", path);

    Ok((headers, body))
}
