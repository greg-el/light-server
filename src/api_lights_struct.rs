use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Light {
    #[serde(rename = "1")]
    pub id1: LightStates,
    #[serde(rename = "2")]
    pub id2: LightStates,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LightStates {
    pub state: LightState,
    pub swupdate: SwUpdate,
    pub r#type: String,
    pub name: String,
    pub modelid: String,
    pub manufacturername: String,
    pub productname: String,
    pub capabilities: Capabilities,
    pub config: Config,
    pub uniqueid: String,
    pub swversion: String,
    pub swconfigid: String,
    pub productid: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LightState {
    pub on: bool,
    pub bri: u16,
    pub hue: u16,
    pub sat: u16,
    pub effect: String,
    pub xy: Vec<f32>,
    pub ct: u16,
    pub alert: String,
    pub colormode: String,
    pub mode: String,
    pub reachable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwUpdate {
    pub state: String,
    pub lastinstall: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Capabilities {
    pub certified: bool,
    pub control: Control,
    pub streaming: Streaming,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Control {
    pub mindimlevel: u32,
    pub maxlumen: u32,
    pub colorgamuttype: String,
    pub colorgamut: Vec<Vec<f32>>,
    pub ct: Ct,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ct {
    pub min: u32,
    pub max: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Streaming {
    pub renderer: bool,
    pub proxy: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub archetype: String,
    pub function: String,
    pub direction: String,
    pub startup: Startup,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Startup {
    pub mode: String,
    pub configured: bool,
}
