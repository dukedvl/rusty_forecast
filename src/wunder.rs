use chrono::{DateTime, Local, Utc,NaiveDateTime};
use postgres_types::FromSql;
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
    pub epoch: i32,
    pub lat: f64,
    pub uv: f64,
    pub winddir: i32,
    pub humidity: i32,
    pub qc_status: i32,
    pub imperial: Imperial,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Imperial {
    pub temp: i32,
    pub heat_index: i32,
    pub dewpt: i32,
    pub wind_chill: i32,
    pub wind_speed: i32,
    pub wind_gust: i32,
    pub pressure: f64,
    pub precip_rate: f64,
    pub precip_total: f64,
    pub elev: i32,
}

//DB Model
#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstModel {
    pub obs_time_utc: chrono::DateTime<Utc>,
    pub obs_time_local: chrono::NaiveDateTime,
    pub temp: i32,
    pub heat_index: i32,
    pub wind_chill: i32,
    pub dewpt: i32,
    pub humidity: i32,
    pub precip_rate: f64,
    pub precip_total: f64,
    pub wind_speed: i32,
    pub winddir: i32,
    pub wind_gust: i32,
    pub pressure: f64,
    pub solar_radiation: f64,
    pub uv: f64,
}

impl InstModel {
    pub fn convert(inst: &mut Root) -> InstModel {
        let obs: &Observation = &inst.observations[0];
        let imp = &obs.imperial;
//,
        InstModel {
            obs_time_utc: DateTime::parse_from_rfc3339(&obs.obs_time_utc)
                .unwrap()
                .into(),
            obs_time_local: NaiveDateTime::parse_from_str(&obs.obs_time_local,"%Y-%m-%d %_H:%M:%S")
                .unwrap()
                .into(),
            temp: imp.temp,
            dewpt: imp.dewpt,
            heat_index: imp.heat_index,
            solar_radiation: obs.solar_radiation,
            uv: obs.uv,
            pressure: imp.pressure,
            humidity: obs.humidity,
            precip_rate: imp.precip_rate,
            precip_total: imp.precip_total,
            wind_chill: imp.wind_chill,
            winddir: obs.winddir,
            wind_speed: imp.wind_speed,
            wind_gust: imp.wind_gust,
        }
    }
}
