use nickel::{HttpRouter, Nickel, QueryString};
use postgres::{Client, Error, NoTls};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

extern crate chrono;
#[macro_use]
extern crate nickel;

mod weather;

fn main() {
    //Wakeup

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
    server.get("/forecast/daily", middleware! {"daily"});
    server.get("/forecast/hourly", middleware! {"hourly"});
    server.get("/forecast/instant", middleware! { "instant"});
    server.get("/historical", middleware! {"historical"});

    server.listen("127.0.0.1:3031").unwrap();
}
