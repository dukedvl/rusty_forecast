use chrono::{DateTime, Local, TimeZone, Utc};
use climacell::{DailyWeather, HourlyWeather};
use config::Config;
use nickel::{HttpRouter, Nickel, QueryString};
use postgres::{Client, NoTls};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

extern crate chrono;
#[macro_use]
extern crate nickel;

mod climacell;
mod wunder;

fn main() {
    let configdata = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    let handle_cfg: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(configdata));
    let thandle_cfg: Arc<Mutex<HashMap<String, String>>> = Arc::clone(&handle_cfg);
    let mhandle_cfg: Arc<Mutex<HashMap<String, String>>> = Arc::clone(&handle_cfg);

    //Wakeup
    let hourlies: Arc<Mutex<Vec<climacell::HourlyWeather>>> = Arc::new(Mutex::new(vec![]));
    let dailies: Arc<Mutex<Vec<climacell::DailyWeather>>> = Arc::new(Mutex::new(vec![]));
    let inst: Arc<Mutex<wunder::Root>> = Arc::new(Mutex::new(wunder::Root::default()));

    //Start Cache refresh Loop
    let thandle_hourly = Arc::clone(&hourlies);
    let thandle_daily = Arc::clone(&dailies);
    let thandle_inst = Arc::clone(&inst);

    //WeatherCacheLoop
    let _handle = thread::spawn(move || {
        //Timestamps
        let mut _hourly_timestamp = DateTime::<Utc>::MIN_UTC;
        let mut _daily_timestamp = DateTime::<Utc>::MIN_UTC;
        let mut _last_instantpull = DateTime::<Utc>::MIN_UTC;

        //Poke DB timestamps
        (_hourly_timestamp, _daily_timestamp) =
            poke_db_timestamps(thandle_cfg.lock().unwrap()["connectionString"].clone());

        loop {
            if (Utc::now() - _last_instantpull) > chrono::Duration::minutes(30) {
                //PullInst Web
                println!("Pulling Instant Web");
                let mut instant_t = thandle_inst.lock().unwrap();
                get_inst_web(
                    &mut instant_t,
                    thandle_cfg.lock().unwrap()["wunderApi"].clone(),
                );
                _last_instantpull = Utc::now();
                drop(instant_t);
            } else {
                println!("Instant data still fresh");
            }

            if (Utc::now() - _hourly_timestamp) > chrono::Duration::hours(8) {
                //Pull Hourly Web
                println!("Hourly data expired (8h), pulling from web");

                let mut hourly_t = thandle_hourly.lock().unwrap();

                //Hit Web API
                get_hourly_web(
                    &mut hourly_t,
                    thandle_cfg.lock().unwrap()["climacellApi"].clone(),
                );

                //Refresh Timestamp
                _hourly_timestamp = Utc::now();

                //Persist Hourly Data in DB
                println!("Persisting new hourly data to DB");
                dump_hourly_db(
                    &mut hourly_t,
                    thandle_cfg.lock().unwrap()["connectionString"].clone(),
                );

                //Unlock reference to hourly data
                drop(hourly_t);
            } else {
                let mut hourly_t = thandle_hourly.lock().unwrap();

                if hourly_t.len() == 0 {
                    println!("Hourly cache empty, pulling from DB");

                    *hourly_t =
                        get_hourly_db(thandle_cfg.lock().unwrap()["connectionString"].clone());
                } else {
                    println!("Hourly data still fresh");
                }
            }

            if (Utc::now() - _daily_timestamp) > chrono::Duration::hours(24) {
                //Pull Daily Web
                println!("Daily data expired(24h), pulling from web");
                let mut daily_t = thandle_daily.lock().unwrap();

                get_daily_web(
                    &mut daily_t,
                    thandle_cfg.lock().unwrap()["climacellApi"].clone(),
                );

                _daily_timestamp = Utc::now();

                //Persist Daily Data in DB
                println!("Persisting daily data to DB");
                dump_daily_db(
                    &mut daily_t,
                    thandle_cfg.lock().unwrap()["connectionString"].clone(),
                );

                drop(daily_t);
            } else {
                let mut daily_t = thandle_daily.lock().unwrap();

                if daily_t.len() == 0 {
                    println!("Daily cache empty, pulling from DB");

                    *daily_t =
                        get_daily_db(thandle_cfg.lock().unwrap()["connectionString"].clone());
                } else {
                    println!("Daily data still fresh");
                }
            }

            thread::sleep(Duration::from_secs(15 * 60));
        }
    });

    let mut server = Nickel::new();

    let mhandle_inst = Arc::clone(&inst);
    let mhandle_hour = Arc::clone(&hourlies);
    let mhandle_daily = Arc::clone(&dailies);

    //Weather routes
    server.get(
        "/forecast/hourly",
        middleware! {|request|
            let mut hourly = mhandle_hour.lock().unwrap();
            let resp =get_cached_hourly(request,&mut hourly);
            drop(hourly);
            resp
        },
    );
    server.get(
        "/forecast/instant",
        middleware! {|request|
            let mut isnt = mhandle_inst.lock().unwrap();
            let resp =get_cached_inst(request,&mut isnt);
            drop(isnt);
            resp
        },
    );
    server.get(
        "/forecast/daily",
        middleware! {|request|
            let mut daily = mhandle_daily.lock().unwrap();
            let resp =get_cached_daily(request,&mut daily);
            drop(daily);
            resp
        },
    );

    server.get(
        "/forecast/historical",
        middleware! {|request|get_hist(request)},
    );

    server.get("/forecast/echo", middleware! {|request| echo(request)});
    server.get("/forecast/poke", middleware! {|request| poke(request, mhandle_cfg.lock().unwrap()["connectionString"].clone())});

    server.listen("0.0.0.0:3031").unwrap();
}

pub fn poke(_request: &mut nickel::Request, conn_str: String) -> String {
    let dailies = get_daily_db(conn_str.clone());

    for daily in dailies.iter() {
        println!("{daily:?}");
    }

    let hourlies = get_hourly_db(conn_str);

    for hourly in hourlies.iter() {
        println!("{hourly:?}");
    }

    serde_json::to_string(&hourlies).unwrap() + &serde_json::to_string(&dailies).unwrap()
}

pub fn echo(request: &mut nickel::Request) -> String {
    const ERROR_STR: &str = "error";

    match request.query().all("echo") {
        Some(value) => match value.first() {
            Some(value) => value.to_owned(),
            None => ERROR_STR.to_string(),
        },
        None => ERROR_STR.to_string(),
    }
}

pub fn get_daily_web(daily_model: &mut Vec<DailyWeather>, api_key: String) {
    println!("hitting daily web API");
    let client = reqwest::blocking::Client::new();
    let params = [
        ("location", "30.4664,-97.7713"),
        ("units", "imperial"),
        ("timesteps", "1d"),
        ("endTime", &get_weekly_timestamp()),
        (
            "fields",
            "temperatureMin,temperatureMax,moonPhase,weatherCode,sunsetTime,sunriseTime",
        ),
        ("apikey", &api_key),
    ];
    let resp = client
        .get("https://api.tomorrow.io/v4/timelines")
        .query(&params)
        .send()
        .unwrap();

    //Fix error handling, don't overwrite data

    let root = resp.json::<climacell::DailyRoot>().unwrap();
    let daily = DailyWeather::convert(root);
    *daily_model = daily;
}

pub fn get_cached_daily(_request: &mut nickel::Request, daily: &mut Vec<DailyWeather>) -> String {
    println!("hitting daily cache");

    serde_json::to_string(&daily).unwrap()
}

pub fn get_hourly_web(hourly_model: &mut Vec<HourlyWeather>, api_key: String) {
    println!("hitting hourly web API");
    let client = reqwest::blocking::Client::new();
    let params = [
        ("location", "30.466,-97.771"),
        ("fields", "temperature,temperatureApparent,weatherCode,precipitationType,precipitationProbability,humidity,dewPoint"),
        ("timesteps", "1h"),
        ("endTime", &get_hourly_timestamp()),
        ("units", "imperial"),
        ("apikey", &api_key),
    ];

    let resp = client
        .get("https://api.tomorrow.io/v4/timelines")
        .query(&params)
        .send()
        .unwrap();
    //Fix error handling, don't overwrite data
    let root = resp.json::<climacell::HourlyRoot>().unwrap();
    let hourly = HourlyWeather::convert(root);
    *hourly_model = hourly;
}

pub fn get_cached_hourly(
    _request: &mut nickel::Request,
    hourly: &mut Vec<climacell::HourlyWeather>,
) -> String {
    println!("hitting hourly cache");

    serde_json::to_string(&hourly).unwrap()
}

pub fn get_cached_inst(_request: &mut nickel::Request, inst: &mut wunder::Root) -> String {
    println!("hitting Inst API");

    serde_json::to_string(&inst).unwrap()
}

pub fn get_inst_web(inst_model: &mut wunder::Root, api_key: String) {
    let client = reqwest::blocking::Client::new();
    let mut params = HashMap::new();
    params.insert("stationId", "KTXAUSTI2731");
    params.insert("format", "json");
    params.insert("units", "e");
    params.insert("apiKey", &api_key);

    let resp = client
        .get("https://api.weather.com/v2/pws/observations/current")
        .query(&params)
        .send()
        .unwrap();

    *inst_model = resp.json::<wunder::Root>().unwrap();
}

pub fn get_hist(_request: &mut nickel::Request) -> String {
    "historical func".to_string()
}

pub fn get_weekly_timestamp() -> String {
    let time = chrono::Local::now() + chrono::Duration::days(7);

    time.to_rfc3339()
}

pub fn get_hourly_timestamp() -> String {
    let time = chrono::Local::now() + chrono::Duration::hours(24);

    time.to_rfc3339()
}

pub fn dump_daily_db(daily_data: &mut [DailyWeather], conn_str: String) {
    let mut client = Client::connect(&conn_str, NoTls).unwrap();

    for interval in daily_data.iter() {
        client.execute("INSERT INTO daily_weather(weather_time,high,low,weather_code,moon_phase,sunrise_time,sunset_time) VALUES ($1,$2,$3,$4,$5,$6,$7)",
    &[&interval.weather_time.naive_local(), &interval.high, &interval.low, &interval.weather_code, &interval.moon_phase, &interval.sunrise_time.unwrap().naive_local(), &interval.sunset_time.unwrap().naive_local()]).unwrap();
    }
}

pub fn dump_hourly_db(hourly_data: &mut [HourlyWeather], conn_str: String) {
    let mut client = Client::connect(&conn_str, NoTls).unwrap();

    for interval in hourly_data.iter() {
        client.execute("INSERT INTO hourly_weather(weather_time,temp,feels_like,weather_code,precipitation_type,precipitation_chance,humidity,dew_point) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
    &[&interval.weather_time.naive_local(), &interval.temp, &interval.feels_like, &interval.weather_code, &interval.precipitation_type, &interval.precipitation_chance, &interval.humidity, &interval.dew_point]).unwrap();
    }
}

pub fn poke_db_timestamps(conn_str: String) -> (DateTime<Utc>, DateTime<Utc>) {
    let mut client = Client::connect(&conn_str, NoTls).unwrap();

    let new_hourly = match client.query(
        "SELECT created_at FROM hourly_weather ORDER BY created_at DESC",
        &[],
    ) {
        Ok(t) => t[0].get(0),
        Err(_) => DateTime::<Utc>::MIN_UTC,
    };

    let new_daily = match client.query(
        "SELECT created_at FROM daily_weather ORDER BY created_at DESC",
        &[],
    ) {
        Ok(t) => t[0].get(0),
        Err(_) => DateTime::<Utc>::MIN_UTC,
    };

    (new_hourly, new_daily)
}

pub fn get_daily_db(conn_str: String) -> Vec<DailyWeather> {
    let mut client = Client::connect(&conn_str, NoTls).unwrap();

    let dailies: Vec<postgres::Row> = client
        .query(
            "SELECT * FROM(SELECT created_at, weather_time, high, low, weather_code, moon_phase, sunrise_time, sunset_time FROM daily_weather ORDER BY created_at DESC LIMIT 8) as Recent ORDER BY weather_time ASC;",
            &[],
        )
        .unwrap();
    let mut return_vec = vec![];

    for row in dailies {
        let weather_convert: chrono::NaiveDateTime = row.get(2);

        let daily = climacell::DailyWeather {
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

    return_vec
}

pub fn get_hourly_db(conn_str: String) -> Vec<HourlyWeather> {
    let mut return_vec = vec![];

    let mut client = Client::connect(&conn_str, NoTls).unwrap();

    let hourlies: Vec<postgres::Row> = client
        .query(
            "SELECT * FROM (SELECT created_at, weather_time, temp, feels_like, weather_code, precipitation_type, precipitation_chance, humidity, dew_point from hourly_weather ORDER BY created_at DESC LIMIT 25) as Recent ORDER BY weather_time ASC;",
            &[],
        )
        .unwrap();

    for row in hourlies.iter() {
        let weather_convert: chrono::NaiveDateTime = row.get(2);

        let hourly = climacell::HourlyWeather {
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

    return_vec
}
