use pnet::{transport, datalink};
use panorama::bus::Bus;
use std::net::{UdpSocket, Ipv4Addr};
use std::collections::HashMap;

#[test]
fn scan_device_test(){
    let bus = Bus::new(HashMap::new());
    bus.scan_device();
}


