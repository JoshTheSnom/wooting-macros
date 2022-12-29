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

pub async fn phillips_first_time_setup(state: &super::super::MacroBackend) {
    //! TODO: we need to inform the user of this and have them press the button on the bridge.

    let bridge = hueclient::Bridge::discover_required()
    .register_user("test") // Press the bridge before running this
    .unwrap();
    
    //println!("the username was {}", state.read().await); // handy for later
    
    println!("the username was {}", bridge.username); // handy for later
}

