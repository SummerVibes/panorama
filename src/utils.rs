use std::net::{IpAddr, UdpSocket};
use pnet::datalink;
use pnet::datalink::NetworkInterface;
use crate::Result;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use crate::data::{AllAbility, UrlEntry};
use crate::bus::HTTP_SERVICE_PORT;

/// get lan interface, namely en0
pub fn get_lan_interface() -> Result<NetworkInterface> {
    for i in datalink::interfaces() {
        if i.name.eq("en0") {
            return Ok(i);
        }
    }
    return Err(anyhow!("en0 interface no found"));
}

/// get local ip address in current LAN, this method need to connect to the network
pub fn get_self_ip() -> Result<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    Ok(socket.local_addr()?.ip())
}

// pub fn get_self_ip() -> Result<IpAddr> {
//     for ip in get_lan_interface()?.ips {
//         if ip.is_ipv4() {
//             return Ok(ip.ip());
//         }
//     }
//     return Err(anyhow!("get self ip failed"));
// }

pub fn get_broadcast_addr(addr: String) -> Result<String> {
    let mut vec:String = addr.split(".").take(3).map(|s| s.to_string()+".").collect();
    vec = vec+"255";
    Ok(vec)
}

pub fn generate_node_id(node_address: String) -> u64 {
    let mut hasher = Sha256::new();
    hasher.input_str(node_address.as_str());
    let hash_prefix = &hasher.result_str()[..8];
    let mut buf: [u8; 8] = [0; 8];
    buf.copy_from_slice(hash_prefix.as_bytes());
    let id: u64 = u64::from_be_bytes(buf);
    id
}

pub fn gen_urls(abilities: &Vec<AllAbility>, http_ip: IpAddr) -> Vec<UrlEntry>{
    abilities.iter().map(|a| UrlEntry{
        url: format!("http://{}:{}/{}", http_ip.to_string(), HTTP_SERVICE_PORT, a.to_string()),
        ability: a.to_string() })
        .collect()
}