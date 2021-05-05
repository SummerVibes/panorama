use std::str::FromStr;
use crate::error::Error;
use crate::data::{AllAbility, UrlEntry};
use std::net::IpAddr;
use crate::utils::gen_urls;
use actix_web::web::ServiceConfig;

pub mod phone;
pub mod treadmill;
pub mod sports_bracelet;

pub trait Device {
    fn gen_urls(&self, http_ip: IpAddr) -> Vec<UrlEntry>;
    fn get_service(&self, cfg: &mut ServiceConfig);
}
pub enum DeviceType {
    Phone,
    Treadmill,
    SportsBracelet,
}

impl FromStr for DeviceType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "Phone" => { DeviceType::Phone }
            "Treadmill" => { DeviceType::Treadmill }
            "SportsBracelet" => { DeviceType::Treadmill }
            &_ => {
                return Err(Error::GetDeviceError);
            }
        };
        Ok(res)
    }
}