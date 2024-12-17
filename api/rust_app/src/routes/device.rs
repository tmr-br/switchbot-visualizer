use std::sync::Arc;

use crate::app_module::AppModule;
use crate::models::switchbot_api::DeviceType;
use axum::{debug_handler, response::Result, Extension, Json};
use reqwest::StatusCode;

#[debug_handler]
pub async fn get_devices(
    Extension(module): Extension<Arc<AppModule>>,
) -> Result<Json<Vec<DeviceType>>, StatusCode> {
    module.device_usecase().get_devices().await
}
