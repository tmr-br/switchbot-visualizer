use std::sync::Arc;

use crate::app_module::AppModule;
use axum::{routing::get, Extension, Router};
mod device;

pub fn create_routes(app_module: Arc<AppModule>) -> Router {
    Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        // .route("/hello", axum::routing::get())
        .route("/devices", get(device::get_devices))
        .layer(Extension(app_module))
}
