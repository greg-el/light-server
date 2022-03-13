use reqwest::Body;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PutBody {
    pub on: Option<bool>,
    pub bri: Option<u8>,
    pub hue: Option<u16>,
    pub sat: Option<u8>,
    pub xy: Option<[f32; 2]>,
    pub ct: Option<u16>,
    pub alert: Option<String>,
    pub effect: Option<String>,
    pub transitiontime: Option<u16>,
    pub bri_inc: Option<i8>,
    pub sat_inc: Option<i8>,
    pub hue_inc: Option<i32>,
    pub ct_inc: Option<i32>,
    pub xy_inc: Option<[f32; 2]>,
}

impl Into<Body> for PutBody {
    fn into(self) -> Body {
        match serde_json::to_string(&self) {
            Ok(t) => Body::from(t),
            _ => Body::from("{}"),
        }
    }
}
