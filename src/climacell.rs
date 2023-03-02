use chrono::{Local, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub struct DailyWeather {
    id: i32,
    created_at: chrono::DateTime<Utc>,
    weather_time: chrono::DateTime<Utc>,
    high: f64,
    low: f64,
    weather_code: String,
    moon_phase: String,
    sunrise_time: Option<chrono::DateTime<Utc>>,
    sunset_time: Option<chrono::DateTime<Utc>>,
}

impl Default for DailyWeather {
    fn default() -> Self {
        Self {
            id: 0,
            created_at: Utc::now(),
            weather_time: Utc::now(),
            high: 0.0,
            low: 0.0,
            weather_code: String::from(""),
            moon_phase: String::from(""),
            sunrise_time: Some(Utc::now()),
            sunset_time: Some(Utc::now()),
        }
    }
}

#[derive(Debug)]
pub struct HourlyWeather {
    id: i32,
    created_at: chrono::DateTime<Utc>,
    weather_time: chrono::DateTime<Utc>,
    temp: f64,
    feels_like: f64,
    weather_code: String,
    precipitation_type: String,
    precipitation_chance: f64,
    humidity: Option<f64>,
    dew_point: Option<f64>,
}

impl Default for HourlyWeather {
    fn default() -> Self {
        Self {
            id: 0,
            created_at: Utc::now(),
            weather_time: Utc::now(),
            temp: 0.0,
            feels_like: 0.0,
            weather_code: String::from(""),
            precipitation_type: String::from(""),
            precipitation_chance: 0.0,
            humidity: Some(0.0),
            dew_point: Some(0.0),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub timelines: Vec<Timeline>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timeline {
    pub timestep: String,
    pub end_time: String,
    pub start_time: String,
    pub intervals: Vec<Interval>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub start_time: String,
    pub values: Values,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Values {
    pub precipitation_probability: i64,
    pub precipitation_type: i64,
    pub temperature: f64,
    pub temperature_apparent: f64,
    pub weather_code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "root")]
pub struct DailyRoot {
    pub data: DailyData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "data")]
pub struct DailyData {
    pub timelines: Vec<DailyTimelines>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "timeline")]
pub struct DailyTimelines {
    pub timestep: String,
    pub end_time: String,
    pub start_time: String,
    pub intervals: Vec<DailyIntervals>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "interval")]
pub struct DailyIntervals {
    pub start_time: String,
    pub values: DailyValues,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "value")]
pub struct DailyValues {
    pub moon_phase: i64,
    pub sunrise_time: String,
    pub sunset_time: String,
    pub temperature_max: f64,
    pub temperature_min: f64,
    pub weather_code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WeatherCode {
    #[default]
    Unknown = 0,
    Clear = 1000,
    Cloudy = 1001,
    Mostly_Clear = 1100,
    Partly_Cloudy = 1101,
    Mostly_Cloudy = 1102,
    Fog = 2000,
    Light_Fog = 2100,
    Light_Wind = 3000,
    Wind = 3001,
    Strong_Wind = 3002,
    Drizzle = 4000,
    Rain = 4001,
    Light_Rain = 4200,
    Heavy_Rain = 4201,
    Snow = 5000,
    Flurries = 5001,
    Light_Snow = 5100,
    Heavy_Snow = 5101,
    Freezing_Drizzle = 6000,
    Freezing_Rain = 6001,
    Light_Freezing_Rain = 6200,
    Heavy_Freezing_Rain = 6201,
    Ice_Pellets = 7000,
    Heavy_Ice_Pellets = 7101,
    Light_Ice_Pellets = 7102,
    Thunderstorm = 8000,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrecipitationType {
    #[default]
    NA = 0,
    Rain = 1,
    Snow = 2,
    Freezing_Rain = 3,
    Ice_Pellets = 4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MoonPhase {
    #[default]
    New = 0,
    Waxing_Crescent = 1,
    First_Quarter = 2,
    Waxing_Gibbous = 3,
    Full = 4,
    Waning_Gibbous = 5,
    Third_Quarter = 6,
    Waning_Crescent = 7,
}
