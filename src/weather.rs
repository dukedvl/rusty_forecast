use chrono::{DateTime, Utc};

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
