use std::{alloc::System, borrow::Borrow, time::SystemTime};

use crate::models::switchbot_api::{ApiResponse, DeviceResponseBody};
use base64::Engine;
use hmac::{Hmac, Mac};
use reqwest;
use sha2::Sha256;
use uuid::Uuid;

const SWITCHBOT_API_URL: &str = "https://api.switch-bot.com";

pub struct SwitchbotApi {
    client: reqwest::Client,
    token: String,
    secret: String,
}

impl SwitchbotApi {
    pub fn new(token: String, secret: String) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            token,
            secret,
        }
    }

    pub async fn get_devices(&self) -> Result<DeviceResponseBody, Box<dyn std::error::Error>> {
        let url = format!("{}/v1.1/devices", SWITCHBOT_API_URL);

        let response = self
            .client
            .get(&url)
            .headers(self.generate_headers())
            .send()
            .await?;

        let text = response.text().await?;
        println!("{:?}", text);
        let result = serde_json::from_str::<ApiResponse>(&text)?;

        Ok(result.body().clone())
    }

    fn generate_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();

        let t = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let nonce = Uuid::new_v4().to_string();

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes()).unwrap();
        mac.update(format!("{}{}{}", self.token, t, nonce).as_bytes());
        let sign = base64::prelude::BASE64_STANDARD
            .encode(mac.finalize().into_bytes())
            .to_string();
        // .to_uppercase();

        headers.insert(reqwest::header::AUTHORIZATION, self.token.parse().unwrap());
        headers.insert("sign", sign.parse().unwrap());
        headers.insert("nonce", nonce.parse().unwrap());
        headers.insert("t", t.parse().unwrap());

        println!("headers: {:?}", headers);

        headers
    }
}
