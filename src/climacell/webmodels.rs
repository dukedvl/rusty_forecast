use serde::{Deserialize, Serialize};
use super::{moon_phase::MoonPhase, weather_code::WeatherCode};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "root")]
pub struct HourlyRoot {
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
    pub precipitation_type: crate::climacell::precipitation_type::PrecipitationType,
    pub temperature: f64,
    pub temperature_apparent: f64,
    pub weather_code: crate::climacell::weather_code::WeatherCode,
    pub humidity: Option<f64>,
    pub dew_point: Option<f64>,
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
    pub moon_phase: MoonPhase,
    pub sunrise_time: String,
    pub sunset_time: String,
    pub temperature_max: f64,
    pub temperature_min: f64,
    pub weather_code: WeatherCode,
}