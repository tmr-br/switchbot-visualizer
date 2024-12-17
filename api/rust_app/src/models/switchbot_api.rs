use serde::{Deserialize, Serialize};
use std::fmt;

// 共通のデバイスフィールド
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBase {
    device_id: String,
    device_name: String,
    hub_device_id: Option<String>,
}

pub trait Device: fmt::Debug {
    fn base(&self) -> &DeviceBase;

    fn device_id(&self) -> &str {
        &self.base().device_id
    }

    fn device_name(&self) -> &str {
        &self.base().device_name
    }

    fn hub_device_id(&self) -> Option<&str> {
        self.base().hub_device_id.as_deref()
    }
}

// 個別のデバイス型の定義
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
    #[serde(flatten)]
    base: DeviceBase,
    enable_cloud_service: bool,
}

impl Bot {
    fn enable_cloud_service(&self) -> bool {
        self.enable_cloud_service
    }
}

impl Device for Bot {
    fn base(&self) -> &DeviceBase {
        &self.base
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meter {
    #[serde(flatten)]
    base: DeviceBase,
}

impl Device for Meter {
    fn base(&self) -> &DeviceBase {
        &self.base
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ColorBulb {
    #[serde(flatten)]
    base: DeviceBase,
    enable_cloud_service: bool,
}

impl ColorBulb {
    fn enable_cloud_service(&self) -> bool {
        self.enable_cloud_service
    }
}

impl Device for ColorBulb {
    fn base(&self) -> &DeviceBase {
        &self.base
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteWithScreen {
    #[serde(flatten)]
    base: DeviceBase,
    enable_cloud_service: bool,
}

impl Device for RemoteWithScreen {
    fn base(&self) -> &DeviceBase {
        &self.base
    }
}

// MeterProCO2の定義と実装
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MeterProCO2 {
    #[serde(flatten)]
    base: DeviceBase,
    enable_cloud_service: bool,
}

impl MeterProCO2 {
    fn enable_cloud_service(&self) -> bool {
        self.enable_cloud_service
    }
}

impl Device for MeterProCO2 {
    fn base(&self) -> &DeviceBase {
        &self.base
    }
}

// Curtain3の定義と実装
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Curtain3 {
    #[serde(flatten)]
    base: DeviceBase,
    enable_cloud_service: bool,
    curtain_devices_ids: Vec<String>,
    calibrate: bool,
    group: bool,
    master: bool,
    open_direction: String,
}

impl Curtain3 {
    fn enable_cloud_service(&self) -> bool {
        self.enable_cloud_service
    }

    fn curtain_devices_ids(&self) -> &Vec<String> {
        &self.curtain_devices_ids
    }

    fn calibrate(&self) -> bool {
        self.calibrate
    }

    fn group(&self) -> bool {
        self.group
    }

    fn master(&self) -> bool {
        self.master
    }

    fn open_direction(&self) -> &str {
        &self.open_direction
    }
}

impl Device for Curtain3 {
    fn base(&self) -> &DeviceBase {
        &self.base
    }
}

// Smart Lock Pro
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmartLockPro {
    #[serde(flatten)]
    base: DeviceBase,
    group: bool,
    master: bool,
    group_name: Option<String>,
    lock_devices_ids: Option<Vec<String>>,
}

impl SmartLockPro {
    fn group(&self) -> bool {
        self.group
    }

    fn master(&self) -> bool {
        self.master
    }

    fn group_name(&self) -> Option<&str> {
        self.group_name.as_deref()
    }

    fn lock_devices_ids(&self) -> Option<Vec<String>> {
        self.lock_devices_ids.clone()
    }
}

impl Device for SmartLockPro {
    fn base(&self) -> &DeviceBase {
        &self.base
    }
}

// Smart Lock
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmartLock {
    #[serde(flatten)]
    base: DeviceBase,
}

impl Device for SmartLock {
    fn base(&self) -> &DeviceBase {
        &self.base
    }
}

// Motion Sensor
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MotionSensor {
    #[serde(flatten)]
    base: DeviceBase,
}

impl Device for MotionSensor {
    fn base(&self) -> &DeviceBase {
        &self.base
    }
}

// Keypad Touch
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeypadTouch {
    #[serde(flatten)]
    base: DeviceBase,
    lock_device_id: Option<String>,
    key_list: Option<Vec<KeyEntry>>,
}

impl KeypadTouch {
    fn lock_device_id(&self) -> Option<&str> {
        self.lock_device_id.as_deref()
    }

    fn key_list(&self) -> Option<&Vec<KeyEntry>> {
        self.key_list.as_ref()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyEntry {
    id: u32,
    name: String,
    r#type: String,
    status: String,
    create_time: u64,
    password: String,
    iv: String,
}

impl KeyEntry {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn r#type(&self) -> &str {
        &self.r#type
    }

    fn status(&self) -> &str {
        &self.status
    }

    fn create_time(&self) -> u64 {
        self.create_time
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn iv(&self) -> &str {
        &self.iv
    }
}

impl Device for KeypadTouch {
    fn base(&self) -> &DeviceBase {
        &self.base
    }
}

// Hub 2
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hub2 {
    #[serde(flatten)]
    base: DeviceBase,
}

impl Device for Hub2 {
    fn base(&self) -> &DeviceBase {
        &self.base
    }
}

// Hub Mini
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HubMini {
    #[serde(flatten)]
    base: DeviceBase,
}

impl Device for HubMini {
    fn base(&self) -> &DeviceBase {
        &self.base
    }
}

// 赤外線リモコンデバイス (例: DIY Fan, DIY Light, TV)
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfraredRemote {
    device_id: String,
    device_name: String,
    remote_type: String,
    hub_device_id: String,
}

impl InfraredRemote {
    fn device_id(&self) -> &str {
        &self.device_id
    }

    fn device_name(&self) -> &str {
        &self.device_name
    }

    fn remote_type(&self) -> &str {
        &self.remote_type
    }

    fn hub_device_id(&self) -> &str {
        &self.hub_device_id
    }
}

// デバイスタイプの列挙型
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "deviceType")]
pub enum DeviceType {
    Bot(Bot),
    Meter(Meter),
    #[serde(rename = "Color Bulb")] // 明示的にJSONフィールド値を指定
    ColorBulb(ColorBulb),
    Curtain3(Curtain3),
    #[serde(rename = "MeterPro(CO2)")]
    MeterProCO2(MeterProCO2),
    #[serde(rename = "remote with screen")]
    RemoteWithScreen(RemoteWithScreen),
    #[serde(rename = "Smart Lock Pro")]
    SmartLockPro(SmartLockPro),
    #[serde(rename = "Smart Lock")]
    SmartLock(SmartLock),
    #[serde(rename = "Motion Sensor")]
    MotionSensor(MotionSensor),
    #[serde(rename = "Keypad Touch")]
    KeypadTouch(KeypadTouch),
    #[serde(rename = "Hub 2")]
    Hub2(Hub2),
    #[serde(rename = "Hub Mini")]
    HubMini(HubMini),
    #[serde(other)]
    Unknown,
}

// APIレスポンス全体
#[derive(Debug, Deserialize, Clone)]
pub struct ApiResponse {
    status_code: i32,
    message: String,
    body: DeviceResponseBody,
}

impl ApiResponse {
    pub fn status_code(&self) -> i32 {
        self.status_code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn body(&self) -> &DeviceResponseBody {
        &self.body
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeviceResponseBody {
    device_list: Vec<DeviceType>,
    infrared_remote_list: Vec<InfraredRemote>,
}

impl DeviceResponseBody {
    pub fn device_list(&self) -> &Vec<DeviceType> {
        &self.device_list
    }

    pub fn infrared_remote_list(&self) -> &Vec<InfraredRemote> {
        &self.infrared_remote_list
    }
}
