use pnet::{transport, datalink};
use panorama::common::*;
use std::net::{UdpSocket, Ipv4Addr};

#[test]
fn interface_test(){
    scan_device().unwrap();
}
#[test]
fn send_packet_test(){
    send_heartbeat_packet();
}


