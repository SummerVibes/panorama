use std::net::UdpSocket;
use std::sync::Mutex;
use std::thread;

use gossip::{Update, UpdateHandler};

use crate::bus::{BROADCAST_RECV, BROADCAST_SEND, GOSSIP_ADDRESS};
use crate::bus::peer::{get_closest_peers, response_ping};
use crate::device::DeviceType;
use crate::device::phone::Phone;
use crate::device::sports_bracelet::SportsBracelet;
use crate::device::treadmill::Treadmill;
use crate::Result;
use crate::utils;
use crate::utils::get_self_ip;

use super::*;
use crate::error::Error;
use std::time::Duration;

impl Node {
    pub fn create(device_type: DeviceType) -> Node{

        let self_ip = get_self_ip().unwrap();

        let device:Box<dyn Device + Send + Sync> = match device_type {
            DeviceType::Phone => {
                Box::new(Phone{abilities: vec![AllAbility::TakePhoto]})
            },
            DeviceType::Treadmill => {
                Box::new(Treadmill{ abilities: vec![] })
            },
            DeviceType::SportsBracelet => {
                Box::new(SportsBracelet{ abilities: vec![] })
            },
        };

        let id = utils::generate_node_id(self_ip.to_string());
        let service = GossipService::new_with_defaults(GOSSIP_ADDRESS.parse().unwrap());

        Node{
            id,
            addr: self_ip,
            device: Arc::new(device),
            store: AbilitiesStore::new(id),
            service
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
        self.service.start(Box::new(|| Some(peers)), Box::new(self.store.clone())).unwrap();
        //register service
        self.register()?;
        // start the http service
        Ok(())
    }

    pub fn register(&mut self) ->Result<()> {
        let urls = self.device.gen_urls(self.addr);
        urls.iter().for_each(|u| self.store.insert(u.ability.clone(),u.url.clone()));
        //tell the change to other thread;
        self.service.submit(self.store.ser().into_bytes()).unwrap();
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.device.gen_urls(self.addr).iter().for_each(|u| {
            self.store.remove(u.ability.clone(),u.url.clone());
        });
        self.service.submit(self.store.ser().into_bytes()).unwrap();
        thread::sleep(Duration::from_secs(1));
        self.service.shutdown();
    }
}

impl AbilitiesStore {
    pub fn new(id: NodeId) -> Self{
        AbilitiesStore{ id, map: Arc::new(Mutex::new(Default::default())) }
    }

    //insert or update
    pub fn insert(&mut self, key: Ability, value: URL) {
        let map = self.map.lock().unwrap();
        let read_ctx = map.len();
        let op = map.update(key, read_ctx.derive_add_ctx(self.id), |set, ctx| {
            set.add(value, ctx)
        });
        //drop lock, otherwise program will encounter a dead-lock;
        drop(map);
        let mut mut_map = self.map.lock().unwrap();
        mut_map.apply(
            op
        );
    }

    pub fn remove(&mut self, key: Ability, value: URL) {
        let map = self.map.lock().unwrap();
        let rm_ctx = map.get(&key).derive_rm_ctx();
        let read_ctx = map.get(&key).derive_add_ctx(self.id);
        let op = map.update(key,read_ctx,|set,_|{
            set.rm(value,rm_ctx)
        });
        drop(map);
        let mut mut_map = self.map.lock().unwrap();
        mut_map.apply(
            op
        );
    }

    // merge map directly
    pub fn merge(&self, map: AbilitiesMap) {
        let mut s_map = self.map.lock().unwrap();
        s_map.merge(map);
    }

    pub fn ser(&self) -> String{
        let map = (*self.map.lock().unwrap()).clone();
        serde_json::to_string(&map).unwrap()
    }

    pub fn get_url(&self, a: Ability) -> Result<URL> {
        let res = self.map.lock().unwrap().get(&a);
        let set = &res.val.ok_or(Error::NoSuchService)?;
        let len = set.iter().count();
        //random index, for load balancing
        let idx = rand::thread_rng().gen_range(0..len);
        let url = set.iter().nth(idx).unwrap().val.to_string();
        Ok(url)
    }

    pub fn print(&self) {
        println!("All services:");
        self.map.lock().unwrap().iter().for_each(|k| {
            println!("{}: ",k.val.0);
            k.val.1.iter().enumerate().for_each(|u| {
                println!("{}. {}",u.0+1,u.1.val);
            });
        })
    }
}


impl UpdateHandler for AbilitiesStore {
    fn on_update(&self, update: Update) {
        let res: AbilitiesMap = serde_json::from_slice(update.content().as_slice()).unwrap();
        info!("receive update: {:?}", res);
        self.merge(res);
    }
}


