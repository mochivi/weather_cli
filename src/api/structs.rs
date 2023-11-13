use serde::Deserialize;
use serde;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct WeatherResponse {
    pub location: Location,
    pub current: Current,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Location {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    tz_id: String,
    localtime_epoch: u32,
    localtime: String,
}

impl Location {
    pub fn simple_display(&self) {
        println!("City: {}", self.name);
        println!("Country: {}", self.country);
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Current {
    last_updated_epoch: u32,
    last_updated: String,
    temp_c: f64,
    temp_f: f64,
    is_day: u8,
    condition: Condition,
    wind_mph: f64,
    wind_kph: f64,
    wind_degree: u32,
    wind_dir: String,
    pressure_mb: f64,
    pressure_in: f64,
    precip_mm: f64,
    precip_in: f64,
    humidity: u32,
    cloud: u16,
    feelslike_c: f64,
    feelslike_f: f64,
    vis_km: f64,
    vis_miles: f64,
    uv: f64,
    gust_mph: f64,
    gust_kph: f64,
}

impl Current {
    pub fn simple_display(&self) {
        println!("Temperature (C): {}", self.temp_c);
        println!("Feels like (C): {}", self.feelslike_c);
        println!("Humidity: {}", self.humidity);
        println!("Precipitation (mm): {}", self.precip_mm);
        println!("Cloud coverage (%): {}", self.cloud);
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Condition {
    text: String,
    icon: String,
    code: u16
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AirQuality {
    co: f32,
    no2: f32,
    o3: f32,
    so2: f32,
    pm2_5: f32,
    pm10: f32,

    #[serde(rename(deserialize = "us-epa-index"))]
    us_epa_index: u8,

    #[serde(rename(deserialize = "gb-defra-index"))]
    gb_defra_index: u8,
}