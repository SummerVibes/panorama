//! define virtual device struct
use serde::{Deserialize, Serialize};

/// device struct
#[derive(Debug,Serialize,Deserialize)]
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
}

/// device type
#[derive(Debug,Serialize,Deserialize)]
pub enum DeviceType{
    Phone,
    Watch,
    PC
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Ability{
    component: Vec<Component>
}

impl Ability {
    pub fn new(component: Vec<Component>) -> Self {
        Ability { component }
    }
}
/// the thing device can do
#[derive(Debug,Serialize,Deserialize)]
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
