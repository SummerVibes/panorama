//! bus module
use std::{fs, thread};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{stdout, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream, UdpSocket};
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use std::sync::{Arc, mpsc, Mutex};
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{MutablePacket, Packet};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};

use crate::bus::device::Device;

pub mod device;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1";
const DEFAULT_LISTENING_PORT: &str = "4000";

///provide some functions about networking
pub struct Bus {
    device_map: Arc<Mutex<HashMap<String, Device>>>
}

impl Bus {

    /// get lan interface, namely en0
    fn get_lan_interface(&self) -> Result<NetworkInterface> {
        for i in datalink::interfaces() {
            info!("{}",i.name);
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
    pub fn scan_device(&self, mut device: Device) -> Result<()> {
        ///get one Arc pointer
        let mut map = self.device_map.clone();
        let mut rc_socket = UdpSocket::bind("0.0.0.0:8888")?;
        let mut sd_socket = rc_socket.try_clone()?;
        ///join multicast group, send heartbeat packet, create another thread to do this thing
        let receiver = thread::spawn(move || {
            //map, store device data at the LAN
            let mut buf = [0u8; 2048];
            let multi_addr = Ipv4Addr::new(224, 0, 0, 123);
            let inter = Ipv4Addr::new(0, 0, 0, 0);
            //join multicast group
            rc_socket.join_multicast_v4(&multi_addr, &inter);
            //send and receive data repeatedly
            loop {
                //receive heartbeat packet
                let (amt, src) = rc_socket.recv_from(&mut buf).unwrap();
                //serde don't have a function to convert from [u8], so convert bytes to str by ourself
                let str = String::from_utf8_lossy(&buf[..amt]);
                //
                let rc_device: Device = serde_json::from_str(&*str).unwrap();
                //insert to map, and send map to main thread
                let mut m = map.lock().unwrap();
                m.insert(String::from(rc_device.name()), rc_device);
                //fmt print map
                info!("{:?}", m);
                stdout().flush().unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        /// send packet
        let sender = thread::spawn(move || {
            loop {
                let data = serde_json::to_string(&device).unwrap();
                sd_socket.send_to(data.as_bytes(), "224.0.0.123:8888").unwrap();
                thread::sleep(Duration::from_secs(3));
            }
        });
        // sender.join();
        // receiver.join();
        Ok(())
    }
    /// read device description file
    pub fn get_device_from_file(&self, path: &str) -> anyhow::Result<Device> {
        let str = fs::read_to_string(Path::new(path))?;
        let mut device: Device = serde_json::from_str(str.as_str())?;
        let self_ip = self.get_self_ip()?;
        let self_ip = self_ip.to_string();
        info!("self ip address is: {}", self_ip);
        ///set ip
        device.set_ip(self_ip);
        Ok(device)
    }

    pub fn new(device_map: Arc<Mutex<HashMap<String, Device>>>) -> Self {
        Bus { device_map }
    }
}


