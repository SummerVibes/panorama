//! data access module

pub mod node;
pub mod store;

use crdts::{Map, CmRDT, CvRDT, Orswot};
use rand::Rng;
use crate::device::Device;
use std::net::IpAddr;
use gossip::GossipService;
use std::sync::{Arc, RwLock};

pub type Ability = String;
pub type URL = String;
pub type NodeId = u64;
pub type AbilitiesMap = Map<Ability, Orswot<URL, NodeId>, NodeId>;
pub type ArcAbilitiesMap = Arc<RwLock<AbilitiesMap>>;

pub enum AllAbility {
    TakePhoto,
    CollectRunningData,
    CollectBodyData,
}
impl ToString for AllAbility{
    fn to_string(&self) -> String {
        match self {
            AllAbility::TakePhoto => String::from("take_photo"),
            AllAbility::CollectRunningData => String::from("collect_running_data"),
            AllAbility::CollectBodyData => String::from("collect_body_data")
        }
    }
}


#[derive(Default,Debug,Clone)]
pub struct AbilitiesStore {
    id: NodeId,
    urls: Vec<UrlEntry>,
    map: ArcAbilitiesMap
}

/// App Node

pub struct Node{
    pub id: NodeId,
    pub addr: IpAddr,
    pub device: Arc<Box<dyn Device + Send + Sync>>,
    //store and service share AbilitiesMap
    pub store: AbilitiesStore,
    pub service: GossipService<AbilitiesStore>
}

#[derive(Debug,Clone)]
pub struct UrlEntry {
    pub(crate) url: String,
    pub(crate) ability: String
}



