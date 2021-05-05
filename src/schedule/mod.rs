//! task schedule module
use crate::data::Node;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::{env, thread};
use std::process::{exit, Command};
use std::time::Duration;
use actix_web::client::Client;

pub async fn ex_command(node: &mut Node) {
    let store = node.store.clone();
    thread::sleep(Duration::from_millis(200));
    loop {
        print!("panorama>");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        let args = parts;
        match command {
            "show" => {
                store.print();
            }
            "run" => {
                let funcs: Vec<&str> = args.collect();
                for str in funcs.iter() {
                    let res = store.get_url(str.to_string());
                    match res {
                        Ok(url) => {
                            let client = Client::default();
                            let res = client
                                .get(url.clone())    // <- Create request builder
                                .send().await;                        // <- Send http request
                            match res {
                                Ok(mut resp) => {
                                    let str = resp.body().await.unwrap();
                                    let str = String::from_utf8(str.to_vec()).unwrap();
                                    println!("From {}, Response: {:?}",url, str);
                                }
                                Err(err) => {
                                    println!("Execute failed: {:?}", err);
                                }
                            }
                        }
                        Err(err) => {
                            println!("Execute failed: {:?}", err);
                        }
                    }
                };
            }
            "quit" | "exit" | "q" => {
                node.shutdown();
                exit(0);
            }
            "cd" => {
                // 如果没有提供路径参数，则默认 '/' 路径
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "" => {
                continue;
            }
            _ => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => { child.wait().unwrap(); }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}

