use crate::device::Device;
use crate::data::AllAbility;
use std::net::IpAddr;
use super::*;
use actix_web::{get, Responder};

pub struct Phone {
    pub abilities: Vec<AllAbility>
}

impl Device for Phone {
    fn gen_urls(&self, http_ip: IpAddr) -> Vec<UrlEntry>{
        gen_urls(&self.abilities,http_ip)
    }

    fn get_service(&self, cfg: &mut ServiceConfig){
        cfg.service(take_photo);
    }
}

#[tracing::instrument]
#[get("/take_photo")]
pub async fn take_photo() -> impl Responder {
    format!("Hello from Phone")
}
