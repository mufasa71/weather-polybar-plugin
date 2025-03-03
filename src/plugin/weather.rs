use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    name: String,
    region: String,
    country: String,
    lat: f32,
    lon: f32,
    tz_id: String,
    localtime_epoch: i32,
    localtime: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    last_updated_epoch: i32,
    last_updated: String,
    pub temp_c: f32,
    temp_f: f32,
    pub is_day: i32,
    pub condition: Condition,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: i32,
    wind_dir: String,
    pressure_mb: f32,
    pressure_in: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: i32,
    cloud: i32,
    feelslike_c: f32,
    feelslike_f: f32,
    vis_km: f32,
    vis_miles: f32,
    uv: f32,
    gust_mph: f32,
    gust_kph: f32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub location: Location,
    pub current: Current,
}
