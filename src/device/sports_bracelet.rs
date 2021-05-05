use super::*;
use actix_web::{get, Responder};

pub struct SportsBracelet {
    pub abilities: Vec<AllAbility>
}

impl Device for SportsBracelet {
    fn gen_urls(&self, http_ip: IpAddr) -> Vec<UrlEntry>{
        gen_urls(&self.abilities,http_ip)
    }

    fn get_service(&self, cfg: &mut ServiceConfig){
        cfg.service(collect_body_data);
    }
}

#[tracing::instrument]
#[get("/collect_body_data")]
pub async fn collect_body_data() -> impl Responder {
    format!("Hello from SportsBracelet")
}
