use std::net::{IpAddr, UdpSocket};
use std::time::{Duration, SystemTime};

use gossip::Peer;
use serde::{Deserialize, Serialize};

use crate::bus::{BROADCAST_RECV, GOSSIP_ADDRESS_PORT};
use crate::Result;
use crate::utils::get_broadcast_addr;

/// response ping request
#[tracing::instrument]
pub fn response_ping(rc_socket: UdpSocket) {
    loop {
        let mut buf = [0; 10];
        let (amt, src) = rc_socket.recv_from(&mut buf).unwrap();
        let str = String::from_utf8_lossy(&buf[..amt]);
        let res: Ping = serde_json::from_str(&str).unwrap_or(Ping::NONE);
        match res {
            Ping::PING => {
                //send Pong back to the src
                rc_socket.send_to(serde_json::to_string(&Ping::PONG).unwrap().as_bytes(), src)
                    .is_err().then(|| info!("send Pong failed"));
            }
            _ => { error!("invalid msg: {} from {}", str, src) }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Ping {
    PING,
    PONG,
    NONE,
}

///send broadcast and wait for some time, then return all response
#[tracing::instrument]
pub fn get_closest_peers(self_ip: IpAddr, sd_socket: UdpSocket) -> Result<Vec<Peer>> {
    let broadcast_addr = format!("{}:{}", get_broadcast_addr(self_ip.to_string())?, BROADCAST_RECV);

    let data = serde_json::to_string(&Ping::PING)?;
    let mut result: Vec<Peer> = vec![];

    // send data
    sd_socket.set_broadcast(true).expect("set_broadcast call failed");
    sd_socket.send_to(data.as_bytes(), broadcast_addr.as_str())?;

    //time to receive data
    let mut left_time = Duration::from_secs(2);

    //buffer to read PING or PONG;
    let buf = &mut [0; 10];

    // deal msg that received in 3 secs
    //set a timer
    let timer = SystemTime::now();

    while left_time > Duration::from_secs(0) {

        sd_socket.set_read_timeout(Some(left_time))?;
        let timer = SystemTime::now();
        let (amt, mut src) = match sd_socket.recv_from(buf) {
            Ok(data) => data,
            Err(_) => {
                info!("time used: {:?} ", SystemTime::now().duration_since(timer).unwrap());
                return Ok(result);
            },
        };
        let str = String::from_utf8_lossy(&buf[..amt]);
        let res: Ping = serde_json::from_str(&str).unwrap_or(Ping::NONE);
        match res {
            Ping::PONG => {
                //not self
                if src.ip() != self_ip {
                    src.set_port(GOSSIP_ADDRESS_PORT);
                    result.push(Peer::new(src.ip().to_string()))
                }
            }
            _ => { error!("invalid msg: {} from {}", str, src) }
        }
        left_time -= SystemTime::now().duration_since(timer)?;
    }
    //print actual time used to receive msg
    info!("time used: {:?} ", SystemTime::now().duration_since(timer).unwrap());
    Ok(result)
}
