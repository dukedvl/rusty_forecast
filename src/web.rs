use crate::climacell::{ models::{ DailyWeather, HourlyWeather }, webmodels::{ DailyRoot, HourlyRoot } };
use crate::{ forecast_db::{ self, HistoricalSearchType }, wunder };
use nickel::{ hyper::header::AccessControlAllowOrigin, status::StatusCode, MediaType, QueryString };
use std::str::FromStr;

pub fn get_hist(_request: &mut nickel::Request, _response: &mut nickel::Response) -> String {
    match _request.query().get("search_type") {
        Some(search_type) => {
            match HistoricalSearchType::from_str(search_type) {
                Ok(decoded_search_type) => {
                    match decoded_search_type {
                        HistoricalSearchType::Daily => {
                            let day = match _request.query().get("day") {
                                Some(d) => d,
                                None => {
                                    _response.set(StatusCode::BadRequest);
                                    return "missing day".to_string();
                                }
                            };
                            //Get historical data points, using single day
                            match forecast_db::get_historical_db(day) {
                                Ok(historical_data) => { serde_json::to_string(&historical_data).unwrap() }
                                Err(e) => { serde_json::to_string(&e).unwrap() }
                            }
                        }

                        HistoricalSearchType::TimeRange => {
                            let query = _request.query();

                            let day1 = match query.get("day1") {
                                Some(d1) => d1,
                                None => {
                                    _response.set(StatusCode::BadRequest);
                                    return "missing day1".to_string();
                                }
                            };

                            let day2 = match query.get("day2") {
                                Some(d2) => d2,
                                None => {
                                    _response.set(StatusCode::BadRequest);
                                    return "missing day2".to_string();
                                }
                            };
                            //Do a time-range call
                            match forecast_db::get_historical_range_db(day1, day2) {
                                Ok(historical_data) => { serde_json::to_string(&historical_data).unwrap() }
                                Err(e) => { serde_json::to_string(&e).unwrap() }
                            }
                        }
                    }
                }
                Err(_) => {
                    _response.set(StatusCode::BadRequest);
                    format!("unknown search_type provided")
                }
            }
        }
        None => "missing search_type".to_string(),
    }
}

pub fn get_lat_long() -> String {
    std::env::var("RUSTYFORECAST_LATLONG").expect("Lat Long not set")
}

pub fn get_hourly_web(hourly_model: &mut Vec<HourlyWeather>) -> Result<String, String> {
    println!("hitting hourly web API");
    let client = reqwest::blocking::Client::new();
    let lat_long = get_lat_long();

    let params = [
        ("location", lat_long.as_str()),
        ("fields", "temperature,temperatureApparent,weatherCode,precipitationType,precipitationProbability,humidity,dewPoint"),
        ("timesteps", "1h"),
        ("endTime", &forecast_db::get_hourly_timestamp()),
        ("units", "imperial"),
        ("apikey", &std::env::var("RUSTYFORECAST_climacellApi").unwrap()),
    ];

    match client.get("https://api.tomorrow.io/v4/timelines").query(&params).send() {
        Ok(resp) => {
            //Fix error handling, don't overwrite data
            match resp.json::<HourlyRoot>() {
                Ok(root) => {
                    let hourly = HourlyWeather::convert(root);
                    *hourly_model = hourly;

                    Ok("Successfully pulled hourly data from Web".to_string())
                }
                Err(e) => Err(format!("Couldn't deserialize hourly data from web, Reason: {e}")),
            }
        }
        Err(e) => Err(format!("Couldn't retrieve hourly data from Web, Reason: {e}")),
    }
}

pub fn get_daily_web(daily_model: &mut Vec<DailyWeather>) -> Result<String, String> {
    println!("hitting daily web API");
    let client = reqwest::blocking::Client::new();
    let lat_long = get_lat_long();

    let params = [
        ("location", lat_long.as_str()),
        ("units", "imperial"),
        ("timesteps", "1d"),
        ("endTime", &forecast_db::get_weekly_timestamp()),
        ("fields", "temperatureMin,temperatureMax,moonPhase,weatherCode,sunsetTime,sunriseTime"),
        ("apikey", &std::env::var("RUSTYFORECAST_climacellApi").unwrap()),
    ];

    match client.get("https://api.tomorrow.io/v4/timelines").query(&params).send() {
        Ok(resp) => {
            match resp.json::<DailyRoot>() {
                Ok(root) => {
                    let daily = DailyWeather::convert(root);
                    *daily_model = daily;

                    Ok("Successfully pulled daily data from Web".to_string())
                }
                Err(e) => Err(format!("Couldn't deserialize daily data from web, Reason: {e}")),
            }
        }
        Err(e) => Err(format!("Couldn't retrieve daily data from Web, Reason: {e}")),
    }
}

pub fn poke() -> String {
    let dailies = forecast_db::get_daily_db();

    for daily in dailies.iter() {
        println!("{daily:?}");
    }

    let hourlies = forecast_db::get_hourly_db();

    for hourly in hourlies.iter() {
        println!("{hourly:?}");
    }

    serde_json::to_string(&hourlies).unwrap() + &serde_json::to_string(&dailies).unwrap()
}

pub fn echo(request: &mut nickel::Request) -> String {
    const ERROR_STR: &str = "error";

    match request.query().all("echo") {
        Some(value) =>
            match value.first() {
                Some(value) => value.to_owned(),
                None => ERROR_STR.to_string(),
            }
        None => ERROR_STR.to_string(),
    }
}

pub fn set_default_headers<'mw>(_req: &mut nickel::Request, mut res: nickel::Response<'mw>) -> nickel::MiddlewareResult<'mw> {
    res.set(AccessControlAllowOrigin::Any);
    res.set(MediaType::Json);

    res.next_middleware()
}

pub fn get_cached_daily(_request: &mut nickel::Request, daily: &mut Vec<DailyWeather>) -> String {
    println!("hitting daily cache");

    serde_json::to_string(&daily).unwrap()
}

pub fn get_cached_hourly(_request: &mut nickel::Request, hourly: &mut Vec<HourlyWeather>) -> String {
    println!("hitting hourly cache");

    serde_json::to_string(&hourly).unwrap()
}

pub fn get_cached_inst(_request: &mut nickel::Request, inst: &mut wunder::models::Root) -> String {
    println!("hitting Inst cache");

    serde_json::to_string(&inst).unwrap()
}
