use std::sync::Arc;

use crate::{models::switchbot_api::DeviceType, services::switchbot_api};
use axum::{response::Result, Json};
use reqwest::StatusCode;

pub struct DeviceUsecase {
    switchbot_api: Arc<switchbot_api::SwitchbotApi>,
}

impl DeviceUsecase {
    pub fn new(switchbot_api: Arc<switchbot_api::SwitchbotApi>) -> Self {
        Self { switchbot_api }
    }

    pub async fn get_devices(&self) -> Result<Json<Vec<DeviceType>>, StatusCode> {
        let response_body = self.switchbot_api.get_devices().await;
        println!("{:?}", response_body);
        match response_body {
            Ok(body) => Ok(Json(body.device_list().clone())),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    // pub async fn get_device_by_id(&self, Path(id): Path<i32>) -> Json<Device> {
    //     Json(Device {
    //         id,
    //         name: "Test Device".into(),
    //         mac_address: "aaa".into(),
    //     })
    // }
}
