use clap::{App as ClapApp, Arg};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use panorama::data::Node;
use panorama::device::DeviceType;
use actix_web::{HttpServer, App};
use panorama::bus::HTTP_SERVICE_PORT;
use std::thread;
use panorama::schedule::ex_command;
use futures::executor::block_on;
use std::str::FromStr;

#[tracing::instrument]
#[actix_web::main]
async fn main(){

    let arg_name = "type";
    let matches = ClapApp::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new(arg_name).required(true)).get_matches();
    let arg_type = DeviceType::from_str(matches.value_of(arg_name).expect("The input is not a string"))
        .expect("invalid device type");

    // init tracing
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting tracing default failed");

    // create and start the node
    let mut node = Node::create(arg_type);
    node.start().is_ok().then(|| info!("node started"));
    //create a server
    let device = node.device.clone();
    let srv = HttpServer::new(move ||{
        App::new().configure(|cfg| {device.get_service(cfg)})
    }).bind(format!("{}:{}",node.addr.to_string(),HTTP_SERVICE_PORT)).unwrap().run();
    // create another thread to handle request
    thread::spawn(|| block_on(async{
        info!("http server started");
        srv.await.unwrap();
    }));
    //register service
    ex_command(&mut node).await;
}