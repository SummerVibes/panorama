//!this module provides some shared functions

use anyhow::{Result};
use std::time::Duration;
use std::net::{IpAddr, SocketAddrV4, TcpStream, SocketAddr, UdpSocket, Ipv4Addr};
use std::str::FromStr;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use std::thread;


const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1";
const DEFAULT_LISTENING_PORT: &str = "4000";

///TODO create another thread to send heart-beat packet repeatedly;
pub fn scan_device() -> Result<()> {
    ///To display devices dynamically at terminal
    let self_ip = get_self_ip()?;
    let arr_ip = self_ip.to_string();
    info!("self ip address is: {}", arr_ip);
    thread::spawn();
    // println!("Find {} device in local area network{:?}", res.len(), res);
    Ok(())
}

///join multicast group, send heartbeat packet
pub fn send_heartbeat_packet() -> Result<()>{
    let mut socket = UdpSocket::bind("0.0.0.0:8888").unwrap();
    let mut buf = [0u8; 65535];
    let multi_addr = Ipv4Addr::new(224, 0, 0, 123);
    let inter = Ipv4Addr::new(0,0,0,0);
    socket.join_multicast_v4(&multi_addr,&inter);

    loop {
        //send heartbeat packet
        socket.send_to(&buf[0..10],"224.0.0.123:8888");
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("received {} bytes from {:?}", amt, src);
        thread::sleep(Duration::from_secs(3));
    }
}


/// get lan interface, namely en0
pub fn get_lan_interface() -> Result<NetworkInterface> {
    for i in datalink::interfaces() {
        // info!("{}",iface.name);
        // info!("{:?}",iface.ips);
        if i.name.eq("en0") {
            return Ok(i);
        }
    }
    return Err(anyhow!("en0 interface no found"));
}

/// get local ip address in current LAN
pub fn get_self_ip() -> Result<IpAddr> {
    for ip in get_lan_interface()?.ips {
        if ip.is_ipv4() {
            return Ok(ip.ip());
        }
    }
    return Err(anyhow!("self ip no found"));
}
