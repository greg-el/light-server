use actix_web::{get, App, HttpServer, Responder};
use reqwest::ClientBuilder;

#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;

pub use self::api_lights_struct::Light;
use crate::put_body_struct::PutBody;
use openweather_struct::OpenWeather;
mod api_lights_struct;
mod openweather_struct;
mod put_body_struct;

const BRIDGE_IP: &str = "192.168.0.17";

lazy_static! {
    static ref HUE_CLIENTKEY: String = std::env::var("HUE_CLIENTKEY").unwrap();
    static ref HUE_USERNAME: String = std::env::var("HUE_USERNAME").unwrap();
    static ref OPEN_WEATHER_MAP_API_KEY: String =
        std::env::var("OPEN_WEATHER_MAP_API_KEY").unwrap();
    static ref LAT: String = std::env::var("LAT").unwrap();
    static ref LON: String = std::env::var("LON").unwrap();
}

async fn put(url: String, p: PutBody) {
    ClientBuilder::new()
        .build()
        .unwrap()
        .put(url)
        .body(p)
        .send()
        .await
        .unwrap();
}

async fn get_device_info() -> reqwest::Response {
    let url: String = format!("http://{BRIDGE_IP}/api/{}/lights", *HUE_USERNAME);

    let resp = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(url)
        .send()
        .await
        .unwrap();

    resp
}

async fn get_weather() {
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        *LAT, *LON, *OPEN_WEATHER_MAP_API_KEY
    );

    let resp = ClientBuilder::new()
        .build()
        .unwrap()
        .get(url)
        .send()
        .await
        .unwrap();

    let body = resp.text().await.unwrap();
    let open_weather: OpenWeather = serde_json::from_str(&body).unwrap();
    let bri = get_brightness_from_weather(open_weather).await;
}

fn get_light_warmth_from_time_of_day(current_weather: OpenWeather) -> u16 {
    let sunrise = current_weather.sys.sunrise as i64;
    let sunset = current_weather.sys.sunset as i64;
    let current_time = current_weather.dt as i64;
    let midday = (sunrise + sunset) / 2;

    if sunrise < current_time && current_time < sunset {
        let s = match current_time < midday {
            true => sunrise,
            false => sunset,
        };
        let a = (current_time - s).abs();
        let b = (midday - s).abs();
        let k = (4000 * a) / b + 2000;
        k as u16
    } else {
        // TODO
        0
    }
}

async fn get_brightness_from_weather(current_weather: OpenWeather) -> f32 {
    let sunrise = current_weather.sys.sunrise;
    let sunset = current_weather.sys.sunset;
    let now = current_weather.dt;

    let total_length = sunset - sunrise;
    let mut current_time = 0;

    if sunset > now {
        current_time = sunset - now;
    }

    let percentage = (current_time as f32 / total_length as f32) * 100.0;
    let distance = (50.0 - percentage).abs();
    (distance * 255.0) / 50.0
}

#[get("/lights/weather")]
async fn weather() -> impl Responder {
    let weather = get_weather().await;
    format!("{weather:?}")
}

#[get("/lights/bright")]
async fn bright() -> impl Responder {
    for id in ["1", "2"] {
        let url: String = format!("http://{BRIDGE_IP}/api/{}/lights/{id}/state", *HUE_USERNAME);
        let body = PutBody {
            on: Some(true),
            bri: Some(254),
            hue: Some(8402),
            sat: Some(140),
            ..Default::default()
        };
        put(url, body).await
    }

    format!("test")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| App::new().service(bright).service(weather))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    fn get_mock_open_data() -> OpenWeather {
        serde_json::from_str(
            r#"{
            "base":"stations",
            "clouds":{
                "all":100
            },
            "cod":200,
            "coord":{
                "lat":51.2766,
                "lon":-0.8422
            },
            "dt":1641045600,
            "id":2649322,
            "main":{
                "feels_like":279.21,
                "humidity":76,
                "pressure":1010,
                "temp":280.86,
                "temp_max":282.38,
                "temp_min":279.75
            },
            "name":"Fleet",
            "sys":{
                "country":"GB",
                "id":2037306,
                "sunrise":1641024000,
                "sunset":1641067200,
                "type":2
            },
            "timezone":0,
            "visibility":10000,
            "weather":
            [
                {
                    "description":"overcast clouds",
                    "icon":"04n",
                    "id":804,
                    "main":"Clouds"
                }
            ],
            "wind":{
                "deg":180,
                "speed":2.57
            }
        }"#,
        )
        .unwrap()
        // sunrise: Sat Jan 01 2022 08:00:00 GMT+0000
        // sunset:  Sat Jan 01 2022 20:00:00 GMT+0000
        // dt:      Sat Jan 01 2022 14:00:00 GMT+0000
    }

    #[actix_rt::test]
    async fn test_get_weather() {
        let mock_open_data: OpenWeather = get_mock_open_data();
        dotenv().ok();
        get_brightness_from_weather(mock_open_data).await;
        assert_eq!(1, 1);
    }

    #[test]
    fn test_get_light_warmth_from_time_of_day() {
        let expected: [(u32, u16, &str); 8] = [
            (1641024000, 0, "08:00"),
            (1641027600, 2666, "09:00"),
            (1641031200, 3333, "10:00"),
            (1641038400, 4666, "12:00"),
            (1641045600, 6000, "14:00"),
            (1641052800, 4666, "16:00"),
            (1641060000, 3333, "18:00"),
            (1641067200, 0, "20:00"),
        ];
        for (epoch, kelvin, time) in expected {
            let mut mock_open_data = get_mock_open_data();
            mock_open_data.dt = epoch;
            let result = get_light_warmth_from_time_of_day(mock_open_data);
            println!("time: {time}, expected_kelvin: {kelvin}, calculated_kelvin: {result}");
        }
    }
}
