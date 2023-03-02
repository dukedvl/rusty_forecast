use nickel::{HttpRouter, Nickel, QueryString};
//use postgres::{Client, Error, NoTls};
use serde::{Deserialize, Serialize};
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
    //Wakeup
    let _hourlies: Vec<climacell::HourlyWeather> = vec![];
    let _dailies: Vec<climacell::DailyWeather> = vec![];

    //Start Cache refresh Loop

    //WeatherCacheLoop
    //if Now - LastInMemInstantPull > 30min,
    //PullInstWeb
    //RefreshTimestamps
    //if Now - LastDBHourlyPull > 6hr,
    //PullHourlyWeb
    //RefreshTimestamp
    //PersistHourlyData
    //if Now - LastDailyPull > 24h,
    //PullDailyWeb
    //RefreshTimestamp
    //PersistDailyData

    let mut server = Nickel::new();

    //Weather routes
    server.get(
        "/forecast/hourly",
        middleware! {|request|get_hourly(request)},
    );
    server.get(
        "/forecast/instant",
        middleware! {|request|get_instant(request)},
    );
    server.get(
        "/forecast/historical",
        middleware! {|request|get_hist(request)},
    );
    server.get("/forecast/daily", middleware! {|request|get_daily(request)});
    server.get("/forecast/echo", middleware! {|request| echo(request)});

    server.listen("127.0.0.1:3031").unwrap();
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

pub fn get_daily(request: &mut nickel::Request) -> String {
    println!("hitting API");
    let client = reqwest::blocking::Client::new();
    let params = [
        ("location", "30.4664,-97.7713"),
        ("units", "imperial"),
        ("timesteps", "1d"),
        //TODO ("endTime", now + 7d ),
        (
            "fields",
            "temperatureMin,temperatureMax,moonPhase,weatherCode,sunsetTime,sunriseTime",
        ),
        ("apikey", ""),
    ];
    let resp = client
        .get("https://api.tomorrow.io/v4/timelines")
        .query(&params)
        .send()
        .unwrap();

    match &resp.json::<climacell::DailyRoot>() {
        Ok(decoded_value) => {
            println!(
                "Decoded {0:?} intervals",
                decoded_value.data.timelines[0].intervals.len()
            );
            return serde_json::to_string(&decoded_value).unwrap();
        }
        Err(err) => {
            println!("error decoding into json:{err}");
        }
    }
    "daily func".to_string()
}

pub fn get_hourly(request: &mut nickel::Request) -> String {
    println!("hitting API");
    let client = reqwest::blocking::Client::new();
    let params = [
        ("location", "30.466,-97.771"),
        ("fields", "temperature,temperatureApparent,weatherCode,precipitationType,precipitationProbability"),
        ("timesteps", "1h"),
        //TODO (endTime, now+24h)
        ("units", "imperial"),
        ("apikey", ""),
    ];

    let resp = client
        .get("https://api.tomorrow.io/v4/timelines")
        .query(&params)
        .send()
        .unwrap();

    match &resp.json::<climacell::Root>() {
        Ok(decoded_value) => {
            println!(
                "Decoded {0:?} intervals",
                decoded_value.data.timelines[0].intervals.len()
            );
            return serde_json::to_string(&decoded_value).unwrap();
        }
        Err(err) => {
            println!("error decoding into json:{err}");
        }
    }

    "hourly func".to_string()
}

//broken at Wunderground.
pub fn get_instant(_request: &mut nickel::Request) -> String {
    println!("hitting API");
    let client = reqwest::blocking::Client::new();
    let mut params = HashMap::new();
    params.insert("stationId", "KTXAUSTI2731");
    params.insert("format", "json");
    params.insert("units", "e");
    params.insert("apiKey", "");

    let resp = client
        .get("https://api.weather.com/v2/pws/observations/current")
        .query(&params)
        .send()
        .unwrap();

    match resp.json::<wunder::Root>() {
        Ok(decoded_value) => {
            println!("Success");
            return serde_json::to_string(&decoded_value).unwrap();
        }
        Err(err) => {
            println!("error decoding into json:{err}");
        }
    }

    "instant func".to_string()
}

pub fn get_hist(request: &mut nickel::Request) -> String {
    "historical func".to_string()
}
