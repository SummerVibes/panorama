#[macro_use]
extern crate log;

use std::{env, fs};
use std::collections::HashMap;
use std::io::stdin;
use std::net::{IpAddr, SocketAddrV4, TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};

use clap::{App, Arg};

use panorama::bus::{Bus};
use panorama::bus::device::Device;

fn main() {
    env::set_var("RUST_LOG", "error");
    env_logger::init();
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("DEVICE_CONFIG_FILE").required(true)).get_matches();
    let file_path = matches.value_of("DEVICE_CONFIG_FILE").unwrap();
    info!("{}", file_path);


    /// scan all device in the local network
    let map = Arc::new(Mutex::new(HashMap::new()));
    let bus = Bus::new(map.clone());
    let mut device = bus.get_device_from_file(file_path).unwrap();
    bus.scan_device(device);

    ///TODO make scan_device and input separated
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        match command {
            "show" => {
                let map = map.lock().unwrap();
                println!("Total of {} device", map.len());
                for i in map.iter() {
                    println!("{:?}",i);
                }
            },
            "run" => {
                let mut peekable = args.peekable();
                let new_dir = peekable.peek().unwrap();
                info!("{}", new_dir);
            }
            //TODO cd and ls command should be complemented
            "cd" => {
                let new_dir = args.peekable().peek()
                    .map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    error!("{}", e);
                }
            }
            "quit" | "exit" | "q" => {
                return;
            }
            _ => { println!("No such command") }
        }
    }
}