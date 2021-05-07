use super::*;
use crate::Result;
use crate::error::Error;
use gossip::{UpdateHandler, Update};
use std::sync::{RwLockWriteGuard, RwLockReadGuard};
use std::ops::Deref;

impl AbilitiesStore {
    pub fn new(id: NodeId, urls: Vec<UrlEntry>, service: Arc<RwLock<GossipService<AbilitiesStore>>>) -> Self{
        AbilitiesStore{ id, urls, map: Arc::new(RwLock::new(Default::default())),service }
    }

    fn get_mut_map(&self) -> RwLockWriteGuard<'_, AbilitiesMap> {
        self.map.write().unwrap()
    }
    fn get_map(&self) -> RwLockReadGuard<'_, AbilitiesMap> {
        self.map.read().unwrap()
    }

    //insert or update
    pub fn insert(&self, key: Ability, value: URL) {
        let mut map = self.get_mut_map();
        let read_ctx = map.read_ctx();
        let op = map.update(key, read_ctx.derive_add_ctx(self.id), |set, ctx| {
            set.add(value, ctx)
        });
        map.apply(
            op
        );
    }

    pub fn remove(&self, key: Ability, value: URL) {
        let mut map = self.get_mut_map();
        // let rm_ctx = map.get(&key).derive_rm_ctx();
        let read_ctx = map.get(&key).derive_add_ctx(self.id);
        let op = map.update(key,read_ctx,|set,_|{
            set.rm(value,set.read_ctx().derive_rm_ctx())
        });
        map.apply(
            op
        );
    }

    pub fn register(&self){
        let urls = self.urls.clone();
        for u in urls {
            self.insert(u.ability,u.url);
        }
    }

    // merge map directly
    pub fn merge(&self, map: AbilitiesMap) {
        let mut s_map = self.get_mut_map();
        s_map.merge(map);
    }

    pub fn ser(&self) -> String{
        let map = self.get_map();
        serde_json::to_string(map.deref()).unwrap()
    }

    pub fn exist(&self, map: &AbilitiesMap) -> bool {
        for u in self.urls.iter() {
            let set = map.get(&u.ability).val;
            if set.is_none() {
                return false;
            }
            if !set.unwrap().contains(&u.url).val {
                return false;
            }
        }
        true
    }

    pub fn get_url(&self, a: Ability) -> Result<URL> {
        let res = self.get_map().get(&a);
        let set = &res.val.ok_or(Error::NoSuchService)?;
        let len = set.iter().count();
        //random index, for load balancing
        let idx = rand::thread_rng().gen_range(0..len);
        let url = set.iter().nth(idx).unwrap().val.to_string();
        Ok(url)
    }

    pub fn print(&self) {
        println!("All services:");
        self.get_map().iter().for_each(|k| {
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
        info!("receive update from: {:?}", res);
        self.merge(res.clone());
        //register self abilities again, to provided covered by map that received;
        self.register();
        //if peers doesn't have self abilities, then push map to them
        if !self.exist(&res) {
            self.service.write().unwrap().submit(self.ser().into_bytes()).unwrap()
        }
    }
}

mod test{
    use super::*;
    #[test]
    fn test_map(){
        // let mut store1 = AbilitiesStore::new(1);
        // let mut store2 = AbilitiesStore::new(2);
        // store1.insert(String::from("sfd"), String::from("sdfsdf"));
        // store2.merge((*store1.map.lock().unwrap()).clone());
        //
        // store1.remove(String::from("sfd"), String::from("sdfsdf"));
        // store2.merge((*store1.map.lock().unwrap()).clone());
        //
        //
        // let mut store1 = AbilitiesStore::new(1);
        // store1.merge((*store2.map.lock().unwrap()).clone());
        // store1.insert(String::from("sfd"), String::from("sdfsdf"));
        // println!("{:?}", store1);

    }
}


