//! bus module
use std::{fs, thread};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream, UdpSocket};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{MutablePacket, Packet};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};

use crate::bus::device::Device;
use std::convert::TryInto;
use std::borrow::Borrow;

pub mod device;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1";
const DEFAULT_LISTENING_PORT: &str = "4000";

///provide some functions about networking
pub struct Bus{
    device_map: HashMap<String,Device>
}


impl Bus {
    /// read device description file
    fn read_ddf(&self, path: &str) -> Result<Device>{
        let str = fs::read_to_string(Path::new(path))?;
        let device:Device = serde_json::from_str(str.as_str())?;
        Ok(device)
    }




    /// get lan interface, namely en0
    fn get_lan_interface(&self) -> Result<NetworkInterface> {
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
    fn get_self_ip(&self) -> Result<IpAddr> {
        for ip in self.get_lan_interface()?.ips {
            if ip.is_ipv4() {
                return Ok(ip.ip());
            }
        }
        return Err(anyhow!("self ip no found"));
    }

    ///TODO create another thread to send heart-beat packet repeatedly;
    pub fn scan_device(&self) -> Result<()> {
        ///To display devices dynamically at terminal
        let self_ip = self.get_self_ip()?;
        let arr_ip = self_ip.to_string();
        info!("self ip address is: {}", arr_ip);
        let device = self.read_ddf("src/ddf_template/host1.json")?;
        ///join multicast group, send heartbeat packet
        let handle = thread::spawn(move ||{
            let mut socket = UdpSocket::bind("0.0.0.0:8888").unwrap();
            let mut buf = [0u8; 2048];
            let multi_addr = Ipv4Addr::new(224, 0, 0, 123);
            let inter = Ipv4Addr::new(0, 0, 0, 0);
            socket.join_multicast_v4(&multi_addr, &inter);
            loop {
                let data = serde_json::to_string(&device).unwrap();
                //send heartbeat packet
                socket.send_to(data.as_bytes(), "224.0.0.123:8888").unwrap();
                let (amt, src) = socket.recv_from(&mut buf).unwrap();
                let msg = String::from_utf8_lossy(&buf[0..amt]);
                println!("received {} bytes from {:?}: {:?}", amt, src, msg);
                thread::sleep(Duration::from_secs(3));
            }
        });
        handle.join();
        // println!("Find {} device in local area network{:?}", res.len(), res);
        Ok(())
    }
    pub fn new(device_map: HashMap<String, Device>) -> Self {
        Bus { device_map }
    }
}
