use chrono::{ DateTime, Local, TimeZone, Utc };
use postgres::{ Client, NoTls };
use std::str::FromStr;

use crate::{ climacell::{ self, models::{ DailyWeather, HourlyWeather } }, wunder::{ self, models::InstModel } };

pub fn poke_db_timestamps() -> Result<(DateTime<Utc>, DateTime<Utc>), String> {
    match Client::connect(&get_conn_str(), NoTls) {
        Ok(mut client) => {
            let new_hourly = match client.query("SELECT created_at FROM hourly_weather ORDER BY created_at DESC", &[]) {
                Ok(t) => t[0].get(0),
                Err(_) => DateTime::<Utc>::MIN_UTC,
            };

            let new_daily = match client.query("SELECT created_at FROM daily_weather ORDER BY created_at DESC", &[]) {
                Ok(t) => t[0].get(0),
                Err(_) => DateTime::<Utc>::MIN_UTC,
            };

            Ok((new_hourly, new_daily))
        }
        Err(e) => Err(format!("Couldn't connect to DB, Reason: {e}")),
    }
}

pub fn get_conn_str() -> String {
    let conn_str = format!(
        "host='{db_hostname}' dbname='{db_name}' port={db_port} user='{db_username}' password='{db_password}'",

        db_hostname = std::env::var("RUSTYFORECAST_DBHOSTNAME").expect("DB Hostname not set"),
        db_name = std::env::var("RUSTYFORECAST_DBNAME").expect("DB Name not set"),
        db_port = std::env::var("RUSTYFORECAST_DBPORT").expect("DB Port not set"),
        db_username = std::env::var("RUSTYFORECAST_DBUSER").expect("DB Username not set"),
        db_password = std::env::var("RUSTYFORECAST_DBPASS").expect("DB Pass not set")
    );

    conn_str
}

pub fn get_daily_db() -> Result<Vec<DailyWeather>, String> {
    match Client::connect(&get_conn_str(), NoTls) {
        Ok(mut client) => {
            match
                client.query(
                    "SELECT * FROM(SELECT id, created_at, weather_time, high, low, weather_code, moon_phase, sunrise_time, sunset_time FROM daily_weather ORDER BY created_at DESC LIMIT 8) as Recent ORDER BY weather_time ASC;",
                    &[]
                )
            {
                Ok(dailies) => {
                    let mut return_vec = vec![];

                    for row in dailies {
                        let weather_convert: chrono::NaiveDateTime = row.get(2);

                        let daily = climacell::models::DailyWeather {
                            id: row.get(0),
                            created_at: row.get(1),
                            weather_time: Local.from_local_datetime(&weather_convert).unwrap().into(),
                            high: row.get(3),
                            low: row.get(4),
                            weather_code: row.get(5),
                            moon_phase: row.get(6),
                            sunrise_time: match row.try_get(7) {
                                Ok(t) => Some(DateTime::<Utc>::from_local(t, Utc)),
                                Err(_) => None,
                            },
                            sunset_time: match row.try_get(8) {
                                Ok(t) => Some(DateTime::<Utc>::from_local(t, Utc)),
                                Err(_) => None,
                            },
                        };

                        return_vec.push(daily);
                    }

                    Ok(return_vec)
                }
                Err(e) => Err(format!("Couldn't retrieve Daily Data from DB, Reason: {e}")),
            }
        }
        Err(e) => Err(format!("Couldn't connect to DB, Reason: {e}")),
    }
}

pub fn get_hourly_db() -> Result<Vec<HourlyWeather>, String> {
    match Client::connect(&get_conn_str(), NoTls) {
        Ok(mut client) => {
            match
                client.query(
                    "SELECT * FROM (SELECT id, created_at, weather_time, temp, feels_like, weather_code, precipitation_type, precipitation_chance, humidity, dew_point from hourly_weather ORDER BY created_at DESC LIMIT 25) as Recent ORDER BY weather_time ASC;",
                    &[]
                )
            {
                Ok(hourlies) => {
                    let mut return_vec = vec![];

                    for row in hourlies.iter() {
                        let weather_convert: chrono::NaiveDateTime = row.get(2);

                        let hourly = climacell::models::HourlyWeather {
                            id: row.get(0),
                            created_at: row.get(1),
                            weather_time: Local.from_local_datetime(&weather_convert).unwrap().into(),
                            temp: row.get(3),
                            feels_like: row.get(4),
                            weather_code: row.get(5),
                            precipitation_type: row.get(6),
                            precipitation_chance: row.get(7),
                            humidity: row.get(8),
                            dew_point: row.get(9),
                        };

                        return_vec.push(hourly);
                    }

                    Ok(return_vec)
                }
                Err(e) => Err(format!("Couldn't deserialize Hourly data from DB, Reason: {e}")),
            }
        }
        Err(e) => Err(format!("Couldn't connect to DB, Reason: {e}")),
    }
}

pub fn get_historical_db(day: &str) -> Result<Vec<InstModel>, String> {
    match Client::connect(&get_conn_str(), NoTls) {
        Ok(mut client) => {
            let query_str = format!("SELECT * FROM historical_weather WHERE date(obs_time_local) = '{day}'");

            match client.query(&query_str, &[]) {
                Ok(rows) => {
                    let mut returns = vec![];

                    for row in rows.iter() {
                        returns.push(InstModel {
                            obs_time_utc: row.get(2),
                            obs_time_local: row.get(3),
                            temp: row.get(4),
                            heat_index: row.get(5),
                            wind_chill: row.get(6),
                            dewpt: row.get(7),
                            humidity: row.get(8),
                            precip_rate: row.get(9),
                            precip_total: row.get(10),
                            wind_speed: row.get(11),
                            winddir: row.get(12),
                            wind_gust: row.get(13),
                            pressure: row.get(14),
                            solar_radiation: row.get(15),
                            uv: row.get(16),
                        });
                    }

                    Ok(returns)
                }
                Err(e) => Err(format!("Couldn't deserialize the historical day data as a vector, Reason: {e}")),
            }
        }
        Err(e) => Err(format!("Client couldn't connect to DB, Reason: {e}")),
    }
}

pub fn get_historical_range_db(day1: &str, day2: &str) -> Result<Vec<InstModel>, String> {
    match Client::connect(&get_conn_str(), NoTls) {
        Ok(mut client) => {
            let query_str = format!("SELECT * FROM historical_weather WHERE date(obs_time_local) BETWEEN '{day1}' AND '{day2}'");

            match client.query(&query_str, &[]) {
                Ok(rows) => {
                    let mut returns = vec![];
                    for row in rows.iter() {
                        returns.push(InstModel {
                            obs_time_utc: row.get(2),
                            obs_time_local: row.get(3),
                            temp: row.get(4),
                            heat_index: row.get(5),
                            wind_chill: row.get(6),
                            dewpt: row.get(7),
                            humidity: row.get(8),
                            precip_rate: row.get(9),
                            precip_total: row.get(10),
                            wind_speed: row.get(11),
                            winddir: row.get(12),
                            wind_gust: row.get(13),
                            pressure: row.get(14),
                            solar_radiation: row.get(15),
                            uv: row.get(16),
                        });
                    }

                    Ok(returns)
                }
                Err(e) => Err(format!("Couldn't deserialize the historical range data as a vector, Reason: {e}")),
            }
        }
        Err(e) => Err(format!("Client couldn't connect to DB, Reason: {e}")),
    }
}

pub enum HistoricalSearchType {
    Daily,
    TimeRange,
}

impl FromStr for HistoricalSearchType {
    type Err = ();

    fn from_str(input: &str) -> Result<HistoricalSearchType, Self::Err> {
        match input {
            "daily" => Ok(HistoricalSearchType::Daily),
            "timerange" => Ok(HistoricalSearchType::TimeRange),
            _ => Err(()),
        }
    }
}

pub fn get_weekly_timestamp() -> String {
    let time = chrono::Local::now() + chrono::Duration::days(5);

    time.to_rfc3339()
}

pub fn get_hourly_timestamp() -> String {
    let time = chrono::Local::now() + chrono::Duration::hours(24);

    time.to_rfc3339()
}

pub fn dump_daily_db(daily_data: &mut [DailyWeather]) -> Result<String, String> {
    match Client::connect(&get_conn_str(), NoTls) {
        Ok(mut client) => {
            for interval in daily_data.iter() {
                match
                    client.execute(
                        "INSERT INTO daily_weather(weather_time,high,low,weather_code,moon_phase,sunrise_time,sunset_time) VALUES ($1,$2,$3,$4,$5,$6,$7)",
                        &[
                            &interval.weather_time.naive_local(),
                            &interval.high,
                            &interval.low,
                            &interval.weather_code,
                            &interval.moon_phase,
                            &interval.sunrise_time.unwrap().naive_local(),
                            &interval.sunset_time.unwrap().naive_local(),
                        ]
                    )
                {
                    Ok(row) => {
                        println!("Successfully dumped interval for {0} to DB, {row} rows affected", interval.weather_time);
                    }
                    Err(e) => {
                        return Err(format!("Failed to dump daily data to DB, Reason: {e}"));
                    }
                }
            }
            Ok("Successfully dumped data to DB".to_string())
        }
        Err(e) => Err(format!("Client couldn't connect to DB, Reason: {e}")),
    }
}

pub fn dump_hourly_db(hourly_data: &mut [HourlyWeather]) -> Result<String, String> {
    match Client::connect(&get_conn_str(), NoTls) {
        Ok(mut client) => {
            for interval in hourly_data.iter() {
                match
                    client.execute(
                        "INSERT INTO hourly_weather(weather_time,temp,feels_like,weather_code,precipitation_type,precipitation_chance,humidity,dew_point) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
                        &[
                            &interval.weather_time.naive_local(),
                            &interval.temp,
                            &interval.feels_like,
                            &interval.weather_code,
                            &interval.precipitation_type,
                            &interval.precipitation_chance,
                            &interval.humidity,
                            &interval.dew_point,
                        ]
                    )
                {
                    Ok(row) => {
                        println!("Successfully dumped interval for {0} to DB, {row} rows affected", interval.weather_time);
                    }
                    Err(e) => {
                        return Err(format!("Failed to dump hourly data to DB, Reason: {e}"));
                    }
                }
            }
            Ok("Successfully dumped data to DB".to_string())
        }
        Err(e) => Err(format!("Client couldn't connect to DB, Reason: {e}")),
    }
}

pub fn dump_inst_db(inst: wunder::models::InstModel, conn_str: &str) -> Result<String, String> {
    match Client::connect(conn_str, NoTls) {
        Ok(mut client) => {
            match
                client.execute(
                    "INSERT INTO historical_weather(obs_time_utc,obs_time_local,temp,heat_index,wind_chill,dew_point,humidity,precip_rate,precip_total,wind_speed,wind_dir,wind_gust,pressure,solar_radiation,uv_index) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)",
                    &[
                        &inst.obs_time_utc,
                        &inst.obs_time_local,
                        &inst.temp,
                        &inst.heat_index,
                        &inst.wind_chill,
                        &inst.dewpt,
                        &inst.humidity,
                        &inst.precip_rate,
                        &inst.precip_total,
                        &inst.wind_speed,
                        &inst.winddir,
                        &inst.wind_gust,
                        &inst.pressure,
                        &inst.solar_radiation,
                        &inst.uv,
                    ]
                )
            {
                Ok(row) => {
                    println!("Successfully dumped interval for {0} to DB, {row} rows affected", &inst.obs_time_utc);
                    Ok("Successfully dumped inst data to historical table".to_string())
                }
                Err(e) => {
                    return Err(format!("Failed to dump inst data to historical table, Reason: {e}"));
                }
            }
        }
        Err(e) => Err(format!("Client couldn't connect to DB, Reason: {e}")),
    }
}
