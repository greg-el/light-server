use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenWeather {
    pub base: String,
    pub clouds: Clouds,
    pub cod: u32,
    pub coord: Coord,
    pub dt: u32,
    pub id: u32,
    pub main: Main,
    pub name: String,
    pub sys: Sys,
    pub timezone: i32,
    pub weather: Vec<Weather>,
    pub wind: Wind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub feels_like: f32,
    pub humidity: u32,
    pub pressure: u32,
    pub temp: f32,
    pub temp_max: f32,
    pub temp_min: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    pub country: String,
    pub id: u32,
    pub sunrise: u32,
    pub sunset: u32,
    pub r#type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub description: String,
    pub icon: String,
    pub id: u32,
    pub main: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub deg: u32,
    pub speed: f32,
}
