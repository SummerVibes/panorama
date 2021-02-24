use clap::{App, Arg};
use std::io::stdin;
use std::path::Path;
use std::{env, fs};
use std::net::{TcpListener, SocketAddrV4, IpAddr, TcpStream};
use anyhow::{Result, Error};
use std::time::Duration;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("DEVICE_CONFIG_FILE").required(true)).get_matches();
    let file_path = matches.value_of("DEVICE_CONFIG_FILE");
    info!("{}", file_path.unwrap());
    //TODO read device description file
    fs::read_to_string(file_path);


    //TODO scan all device in the local network
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        match command {
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
                    eprintln!("{}", e);
                }
            }
            "quit" | "exit" | "q" => {
                return;
            }
            _ => { error!("No such command") }
        }
    }
}