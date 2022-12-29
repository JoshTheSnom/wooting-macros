use lifx_rs;
use lifx_api_server;
use hueclient;
use hostname;


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Hash, Eq)]
/// Phillips hue lights. Not done yet.
pub enum PhillipsHueStatus {}




pub trait LightActions {
    fn list_lights(){}
    fn turn_on(){}
    fn turn_off(){}
    fn set_brightness(){}
    fn set_color(){}
    fn first_time_setup(){}
}

pub fn phillips_first_time_setup() {
    //

    let bridge = hueclient::Bridge::discover_required()
    .register_user(dbg!(hostname::get().unwrap()).to_str().unwrap()) // Press the bridge before running this
    .unwrap();
    println!("the username was {}", bridge.username); // handy for later
}

