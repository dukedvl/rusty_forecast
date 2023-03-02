use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub observations: Vec<Observation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    #[serde(rename = "stationID")]
    pub station_id: String,
    pub obs_time_utc: String,
    pub obs_time_local: String,
    pub neighborhood: String,
    pub software_type: String,
    pub country: String,
    pub solar_radiation: f64,
    pub lon: f64,
    pub realtime_frequency: Value,
    pub epoch: i64,
    pub lat: f64,
    pub uv: f64,
    pub winddir: i64,
    pub humidity: i64,
    pub qc_status: i64,
    pub imperial: Imperial,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Imperial {
    pub temp: i64,
    pub heat_index: i64,
    pub dewpt: i64,
    pub wind_chill: i64,
    pub wind_speed: i64,
    pub wind_gust: i64,
    pub pressure: f64,
    pub precip_rate: f64,
    pub precip_total: f64,
    pub elev: i64,
}
