use axum::{Router, routing, Extension};

use crate::{service::{compilation::SimpleCompilationService, execution::ProcessExecutionService}, control::compile};

pub fn compile_router() -> Router {
    let simple_compile_service = SimpleCompilationService::new(ProcessExecutionService {});
    
    let handler = routing::post(compile::post_compile::<SimpleCompilationService<ProcessExecutionService>>)
        .layer(Extension(simple_compile_service));

    Router::new()
        .route("/", handler)
}
