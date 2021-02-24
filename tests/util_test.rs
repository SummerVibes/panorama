use std::fs;
use std::path::Path;
use panorama::bus::device::*;
use std::borrow::Borrow;

#[test]
fn struct_to_json(){
    let ability = Ability::new(vec![Component::Camera,Component::Loudspeaker,Component::Screen]);
    let device = Device::new(
        String::from("host1"),
        100000,
        DeviceType::PC,
        ability,
        String::from("192.168.0.1")
    );
    println!("{}", serde_json::to_string(&device).unwrap());
}
#[test]
fn json_to_struct(){
    let str = fs::read_to_string(Path::new("src/ddf_template/host1.json")).unwrap();
    let d:Device = serde_json::from_str(str.as_str()).unwrap();
    println!("{:?}",d);
}