//! define virtual device struct
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// device struct
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Device {
    name: String,
    computing_power: u32,
    device_type: DeviceType,
    /// one device has multiple ability
    ability: Ability,
    /// 0 to 100
    battery: u8,
    /// ip address
    ip: String,
}

impl Device {
    pub fn new(name: String, computing_power: u32, device_type: DeviceType, ability:Ability, ip: String) -> Self {
        Device { name, computing_power, device_type, ability, ip, battery: 100 }
    }
    fn charge(&mut self){
        self.battery = 100;
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn computing_power(&self) -> u32 {
        self.computing_power
    }
    pub fn device_type(&self) -> &DeviceType {
        &self.device_type
    }
    pub fn ability(&self) -> &Ability {
        &self.ability
    }
    pub fn battery(&self) -> u8 {
        self.battery
    }
    pub fn ip(&self) -> &str {
        &self.ip
    }
    pub fn set_ip(&mut self, ip: String) {
        self.ip = ip;
    }
}

/// device type
#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum DeviceType{
    Phone,
    Watch,
    PC
}
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Ability{
    component: Vec<Component>
}

impl Ability {
    pub fn new(component: Vec<Component>) -> Self {
        Ability { component }
    }
}
/// the thing device can do
#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum Component {
    Camera,
    Recorder,
    Loudspeaker,
    Screen
}
impl Component {
    fn get_battery_cost(self) -> u8{
        match self {
            /// one min cost 2
            Component::Recorder => {2}
            /// one min cost 3
            Component::Loudspeaker => {3}
            /// one min cost 4
            Component::Screen => {4}
            ///one min cost 5
            Component::Camera => {5}
        }
    }
}
