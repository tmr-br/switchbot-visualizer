use std::sync::Arc;

use crate::config::Config;
use crate::services::switchbot_api::SwitchbotApi;
use crate::usecases::device::DeviceUsecase;

pub struct AppModule {
    device_usecase: DeviceUsecase,
}

impl AppModule {
    pub fn new() -> Self {
        let config = Config::new();
        let switchbot_api = Arc::new(SwitchbotApi::new(
            config.switchbot_api_token.clone(),
            config.switchbot_api_secret.clone(),
        ));

        let device_usecase = DeviceUsecase::new(switchbot_api.clone());

        Self { device_usecase }
    }

    pub fn device_usecase(&self) -> &DeviceUsecase {
        &self.device_usecase
    }
}
