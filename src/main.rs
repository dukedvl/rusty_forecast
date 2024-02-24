use chrono::{DateTime, Utc};
use climacell::models::{HourlyWeather, DailyWeather};
use nickel::{HttpRouter, Nickel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

extern crate chrono;
#[macro_use]
extern crate nickel;

mod climacell;
mod forecast_db;
mod web;
mod wunder;

fn main() {
    //Wakeup
    let hourlies: Arc<Mutex<Vec<HourlyWeather>>> = Arc::new(Mutex::new(vec![]));
    let dailies: Arc<Mutex<Vec<DailyWeather>>> = Arc::new(Mutex::new(vec![]));
    let inst: Arc<Mutex<wunder::models::Root>> =
        Arc::new(Mutex::new(wunder::models::Root::default()));

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

        let conn_str = forecast_db::get_conn_str();

        //Poke DB timestamps
        (_hourly_timestamp, _daily_timestamp) = forecast_db::poke_db_timestamps();

        loop {
            if (Utc::now() - _last_instantpull) > chrono::Duration::minutes(30) {
                //PullInst Web
                println!("Pulling Instant Web");
                let mut instant_t = thandle_inst.lock().unwrap();
                wunder::get_inst_web(&mut instant_t);
                _last_instantpull = Utc::now();

                forecast_db::dump_inst_db(
                    wunder::models::InstModel::convert(&mut instant_t),
                    &conn_str,
                );
                drop(instant_t);
            } else {
                println!("Instant data still fresh");
            }

            if (Utc::now() - _hourly_timestamp) > chrono::Duration::hours(8) {
                //Pull Hourly Web
                println!("Hourly data expired (8h), pulling from web");

                let mut hourly_t = thandle_hourly.lock().unwrap();

                //Hit Web API
                web::get_hourly_web(&mut hourly_t);

                //Refresh Timestamp
                _hourly_timestamp = Utc::now();

                //Persist Hourly Data in DB
                println!("Persisting new hourly data to DB");
                forecast_db::dump_hourly_db(&mut hourly_t);

                //Unlock reference to hourly data
                drop(hourly_t);
            } else {
                let mut hourly_t = thandle_hourly.lock().unwrap();

                if hourly_t.len() == 0 {
                    println!("Hourly cache empty, pulling from DB");

                    *hourly_t = forecast_db::get_hourly_db();
                } else {
                    println!("Hourly data still fresh");
                }
            }

            if (Utc::now() - _daily_timestamp) > chrono::Duration::hours(24) {
                //Pull Daily Web
                println!("Daily data expired(24h), pulling from web");
                let mut daily_t = thandle_daily.lock().unwrap();

                web::get_daily_web(&mut daily_t);

                _daily_timestamp = Utc::now();

                //Persist Daily Data in DB
                println!("Persisting daily data to DB");
                forecast_db::dump_daily_db(&mut daily_t);

                drop(daily_t);
            } else {
                let mut daily_t = thandle_daily.lock().unwrap();

                if daily_t.len() == 0 {
                    println!("Daily cache empty, pulling from DB");

                    *daily_t = forecast_db::get_daily_db();
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

    let mut router = Nickel::router();

    //Weather routes
    router.get(
        "/forecast/hourly",
        middleware! {|request|
            let mut hourly = mhandle_hour.lock().unwrap();
            let resp = web::get_cached_hourly(request,&mut hourly);
            drop(hourly);
            resp
        },
    );

    router.get(
        "/forecast/instant",
        middleware! {|request|
            let mut isnt = mhandle_inst.lock().unwrap();
            let resp = web::get_cached_inst(request,&mut isnt);
            drop(isnt);
            resp
        },
    );

    router.get(
        "/forecast/daily",
        middleware! {|request|
            let mut daily = mhandle_daily.lock().unwrap();
            let resp = web::get_cached_daily(request,&mut daily);
            drop(daily);
            resp
        },
    );

    router.get(
        "/forecast/historical",
        middleware! {|request, mut response| web::get_hist(request, &mut response) },
    );

    router.get("/forecast/echo", middleware! {|request| web::echo(request)});
    router.get("/forecast/poke", middleware! {web::poke()});

    router.get(
        "/forecast/healthcheck",
        middleware! {println!("Healthcheck(Ok)"); "ok"},
    );

    server.utilize(web::set_default_headers);

    server.utilize(router);

    server
        .listen(format!(
            "{hostname}:{port}",
            hostname = std::env::var("RUSTYFORECAST_HostURL").unwrap(),
            port = std::env::var("RUSTYFORECAST_HostPort").unwrap()
        ))
        .unwrap();
}