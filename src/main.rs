use nickel::{HttpRouter, Nickel, QueryString};
use postgres::{Client, Error, NoTls};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

extern crate chrono;
extern crate nickel;

mod weather;

fn main() {
    let a = weather::DailyWeather::default();

    println!("Hello, world!");
}
