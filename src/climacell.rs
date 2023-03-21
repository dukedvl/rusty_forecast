use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod moon_phase;
mod precipitation_type;
mod weather_code;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DailyWeather {
    pub id: i32,
    pub created_at: chrono::DateTime<Utc>,
    pub weather_time: chrono::DateTime<Utc>,
    pub high: f64,
    pub low: f64,
    pub weather_code: crate::climacell::weather_code::WeatherCode,
    pub moon_phase: crate::climacell::moon_phase::MoonPhase,
    pub sunrise_time: Option<chrono::DateTime<Utc>>,
    pub sunset_time: Option<chrono::DateTime<Utc>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct HourlyWeather {
    pub id: i32,
    pub created_at: chrono::DateTime<Utc>,
    pub weather_time: chrono::DateTime<Utc>,
    pub temp: f64,
    pub feels_like: f64,
    pub weather_code: crate::climacell::weather_code::WeatherCode,
    pub precipitation_type: crate::climacell::precipitation_type::PrecipitationType,
    pub precipitation_chance: f64,
    pub humidity: Option<f64>,
    pub dew_point: Option<f64>,
}

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
    pub moon_phase: crate::climacell::moon_phase::MoonPhase,
    pub sunrise_time: String,
    pub sunset_time: String,
    pub temperature_max: f64,
    pub temperature_min: f64,
    pub weather_code: crate::climacell::weather_code::WeatherCode,
}

impl HourlyWeather {
    pub fn convert(hourly: HourlyRoot) -> Vec<HourlyWeather> {
        let mut hourlyvec = vec![];

        for interval in hourly.data.timelines[0].intervals.iter() {
            hourlyvec.push(HourlyWeather {
                id: 0,
                created_at: Utc::now(),
                weather_time: DateTime::parse_from_rfc3339(&interval.start_time)
                    .unwrap()
                    .into(),
                temp: interval.values.temperature,
                feels_like: interval.values.temperature_apparent,
                weather_code: interval.values.weather_code,
                precipitation_chance: interval.values.precipitation_probability as f64,
                precipitation_type: interval.values.precipitation_type,
                humidity: interval.values.humidity,
                dew_point: interval.values.dew_point,
            });
        }

        hourlyvec
    }
}
impl DailyWeather {
    pub fn convert(daily: DailyRoot) -> Vec<DailyWeather> {
        let mut dailyvec = vec![];

        for interval in daily.data.timelines[0].intervals.iter() {
            dailyvec.push(DailyWeather {
                id: 0,
                created_at: Utc::now(),
                weather_time: DateTime::parse_from_rfc3339(&interval.start_time)
                    .unwrap()
                    .into(),
                high: interval.values.temperature_max,
                low: interval.values.temperature_min,
                weather_code: interval.values.weather_code,
                sunrise_time: Some(
                    DateTime::parse_from_rfc3339(&interval.values.sunrise_time)
                        .unwrap()
                        .into(),
                ),
                sunset_time: Some(
                    DateTime::parse_from_rfc3339(&interval.values.sunset_time)
                        .unwrap()
                        .into(),
                ),
                moon_phase: interval.values.moon_phase,
            });
        }

        dailyvec
    }
}
