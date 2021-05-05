use super::*;
use actix_web::{get, Responder};
use actix_web::web::ServiceConfig;

pub struct Treadmill {
    pub abilities: Vec<AllAbility>
}

impl Device for Treadmill{
    fn gen_urls(&self, http_ip: IpAddr) -> Vec<UrlEntry>{
        gen_urls(&self.abilities,http_ip)
    }
    fn get_service(&self, cfg: &mut ServiceConfig){
        cfg.service(collect_running_data);
    }
}

#[tracing::instrument]
#[get("/collect_running_data")]
pub async fn collect_running_data() -> impl Responder {
    format!("Hello from Treadmill")
}