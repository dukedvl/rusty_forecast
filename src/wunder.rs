pub mod models;

fn get_station_id() -> String {
    std::env::var("RUSTYFORECAST_StationID").expect("Station ID not set")
}

pub fn get_inst_web(inst_model: &mut models::Root) {
    let client = reqwest::blocking::Client::new();

    let station_id = get_station_id();

    let params = [
        ("stationId", station_id.as_str()),
        ("format", "json"),
        ("units", "e"),
        ("apiKey", &std::env::var("RUSTYFORECAST_wunderApi").unwrap()),
    ];

    let resp = client
        .get("https://api.weather.com/v2/pws/observations/current")
        .query(&params)
        .send()
        .unwrap();

    *inst_model = resp.json::<models::Root>().unwrap();
}
