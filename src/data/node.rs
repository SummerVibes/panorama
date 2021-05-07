use std::net::{UdpSocket, SocketAddr};
use std::thread;

use crate::bus::{BROADCAST_RECV, BROADCAST_SEND, GOSSIP_ADDRESS_PORT};
use crate::bus::peer::{get_closest_peers, response_ping};
use crate::device::DeviceType;
use crate::device::phone::Phone;
use crate::device::sports_bracelet::SportsBracelet;
use crate::device::treadmill::Treadmill;
use crate::Result;
use crate::utils;
use crate::utils::get_self_ip;

use super::*;
use std::time::Duration;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

impl Node {
    pub fn create(device_type: DeviceType) -> Node{

        let self_ip = get_self_ip().unwrap();
        let device:Box<dyn Device + Send + Sync> = match device_type {
            DeviceType::Phone => {
                Box::new(Phone{abilities: vec![AllAbility::TakePhoto]})
            },
            DeviceType::Treadmill => {
                Box::new(Treadmill{ abilities: vec![AllAbility::CollectRunningData] })
            },
            DeviceType::SportsBracelet => {
                Box::new(SportsBracelet{ abilities: vec![AllAbility::CollectBodyData] })
            },
        };

        let id = utils::generate_node_id(self_ip.to_string());
        // let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str(LOCAL_ADDRESS).unwrap()), GOSSIP_ADDRESS_PORT);
        let ip = SocketAddr::new(self_ip, GOSSIP_ADDRESS_PORT);
        let service = Arc::new(RwLock::new(GossipService::new_with_defaults(ip)));
        let urls = device.gen_urls(self_ip);
        Node{
            id,
            addr: self_ip,
            device: Arc::new(device),
            store: AbilitiesStore::new(id,urls,service.clone()),
            service: service.clone()
        }
    }
    pub fn start(&mut self) -> Result<()>{
        let rc_socket = UdpSocket::bind(format!("0.0.0.0:{}", BROADCAST_RECV)).expect("bind failed");
        let sd_socket = UdpSocket::bind(format!("0.0.0.0:{}", BROADCAST_SEND)).expect("bind failed");
        //create a thread to response ping request
        thread::spawn(|| response_ping(rc_socket));
        let peers = get_closest_peers(self.addr.clone(), sd_socket).unwrap();
        info!("find closest peers: {:?}", peers);
        // start the gossip service
        self.get_mut_service().start(Box::new(|| Some(peers)), Box::new(self.store.clone())).unwrap();
        // self.register().unwrap();
        Ok(())
    }

    pub fn register(&mut self) ->Result<()> {
        //tell the change to other thread;
        self.store.register();
        self.get_mut_service().submit(self.store.ser().into_bytes()).unwrap();
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.device.gen_urls(self.addr).iter().for_each(|u| {
            self.store.remove(u.ability.clone(),u.url.clone());
        });
        self.get_service().submit(self.store.ser().into_bytes()).unwrap();
        //TODO how to make sure peers receive this.
        thread::sleep(Duration::from_secs(3));
        self.get_mut_service().shutdown();
    }

    pub fn get_service(&self) -> RwLockReadGuard<'_, GossipService<AbilitiesStore>> {
        self.service.read().unwrap()
    }
    pub fn get_mut_service(&self) -> RwLockWriteGuard<'_, GossipService<AbilitiesStore>> {
        self.service.write().unwrap()
    }
}
