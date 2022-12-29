#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Hash, Eq)]
/// Phillips hue lights. Not done yet.
pub enum PhillipsHueStatus {}




pub trait LightActions {
    pub fn list_lights(){},
    pub fn turn_on(){},
    pub fn turn_off(){},
    pub fn set_brightness(){},
    pub fn set_color(){},
    pub fn first_time_setup(){},
}

pub fn phillips_first_time_setup() {
    let bridge = hueclient::Bridge::discover_required()
    .register_user("mycomputer") // Press the bridge before running this
    .unwrap();
    println!("the username was {}", bridge.username); // handy for later
}

