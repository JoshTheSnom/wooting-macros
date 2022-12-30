use hostname;
use hueclient;
use lifx_api_server;
use lifx_rs;

#[derive(Debug, Clone)]
/// Phillips hue lights. Not done yet.
pub struct PhillipsHueStatus{
    pub bridge: Vec<hueclient::Bridge>,
}

pub struct LifxStatus{
    
}

pub trait LightActions {
    fn list_lights() {}
    fn turn_on() {}
    fn turn_off() {}
    fn set_brightness() {}
    fn set_color() {}
    // TODO: This trait should be implemented for each manufacturer of lights.
    fn first_time_setup() {}
}

impl LightActions for PhillipsHueStatus {
    async fn first_time_setup(state: &super::super::MacroBackend) {
        //! TODO: we need to inform the user of this and have them press the button on the bridge.
        // TODO: Cannot test this without a bridge.
        // TODO: Make a manual IP address setup and also an automatic one.
        let bridge = hueclient::Bridge::discover_required()
            .register_user(&state.config.read().await.computer_hostname.as_str())
            .unwrap();
    
        //println!("the username was {}", state.read().await); // handy for later
    
        //println!("the username was {}", bridge.username); // handy for later
    }
}

