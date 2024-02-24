use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::webmodels::{DailyRoot, HourlyRoot};

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
