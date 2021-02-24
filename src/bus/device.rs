pub struct Device {
    name: String,
    computing_power: u32,
    device_type: DeviceType,

}
pub enum DeviceType{
    Phone,
    Watch,
    PC
}