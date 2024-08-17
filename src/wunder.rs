pub mod models;

fn get_station_id() -> String {
    std::env::var("RUSTYFORECAST_StationID").expect("Station ID not set")
}

pub fn get_inst_web(inst_model: &mut models::Root) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();

    let station_id = get_station_id();

    let params = [
        ("stationId", station_id.as_str()),
        ("format", "json"),
        ("units", "e"),
        ("apiKey", &std::env::var("RUSTYFORECAST_wunderApi").unwrap()),
    ];

    match client.get("https://api.weather.com/v2/pws/observations/current").query(&params).send() {
        Ok(resp) => {
            match resp.json::<models::Root>() {
                Ok(model) => {
                    *inst_model = model;
                    Ok("Successfully pulled and deserialized Inst data from Wunderground".to_string())
                }
                Err(e) => Err(format!("Couldn't deserialize Inst data from wunderground, Reason: {e}")),
            }
        }
        Err(e) => Err(format!("Couldn't pull Inst data from wunderground, Reason: {e}")),
    }
}
