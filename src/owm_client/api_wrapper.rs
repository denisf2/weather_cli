use crate::CliArgs;

use super::json_structs::{Forecast, ServiceRespond};

use exitfailure::ExitFailure;
use reqwest::{Response, Url};

const GEO_API: &str = "https://api.openweathermap.org/geo/1.0/direct";
const WEATHER_API: &str = "https://api.openweathermap.org/data/2.5/weather";
const GEO_REVERSE_API: &str = "https://api.openweathermap.org/geo/1.0/reverse";

#[derive(Debug, Clone)]
pub struct LocationInfo {
    pub city: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
}

pub async fn get_city_info(
    lat: f64,
    lon: f64,
    api_key: &str,
    cli: &CliArgs,
) -> Result<LocationInfo, ExitFailure> {
    if cli.verbose {
        println!("->> {:<12} - get_city_info", "OPENWEATHERMAP");
    }
    // send request
    let limit = 1;
    let url = format!("{GEO_REVERSE_API}?lat={lat}&lon={lon}&limit={limit}&appid={api_key}");

    let resp = send_request(url, cli).await?;

    // parse respond
    let resp = resp.json::<ServiceRespond>().await?;
    // dbg!(&resp);
    match resp {
        ServiceRespond::Main(a) => {
            // check vec size
            let city = a[0].name.clone();
            let country = a[0].country.clone();
            Ok(LocationInfo {
                city,
                country,
                lat,
                lon,
            })
        }
        ServiceRespond::Message(m) => Err(failure::err_msg(format!(
            "Failed to parse json couse: {}",
            m.message
        )))?,
        _ => Err(failure::err_msg("Failed to parse json"))?,
    }
}

pub async fn get_coords(
    city_name: &str,
    country_code: &str,
    api_key: &str,
    cli: &CliArgs,
) -> Result<LocationInfo, ExitFailure> {
    if cli.verbose {
        println!("->> {:<12} - get_coords", "OPENWEATHERMAP");
    }
    // send request
    let state_code = "";
    let limit = 3;
    let url = format!(
        "{GEO_API}?q={city_name},{state_code},{country_code}&limit={limit}&appid={api_key}"
    );

    let resp = send_request(url, cli).await?;

    // parse respond
    let resp = resp.json::<ServiceRespond>().await?;
    // dbg!(&resp);
    match resp {
        ServiceRespond::Main(a) => {
            // check vec size
            let lat = a[0].lat;
            let lon = a[0].lon;
            Ok(LocationInfo {
                city: city_name.to_string(),
                country: country_code.to_string(),
                lat,
                lon,
            })
        }
        ServiceRespond::Message(m) => Err(failure::err_msg(format!(
            "Failed to parse json couse: {}",
            m.message
        )))?,
        _ => Err(failure::err_msg("Failed to parse json"))?,
    }
}

pub async fn get_forcast(
    coord: (f64, f64),
    api_key: &str,
    cli: &CliArgs,
) -> Result<Forecast, ExitFailure> {
    if cli.verbose {
        println!("->> {:<12} - get_forcast", "OPENWEATHERMAP");
    }
    // send request
    let units = "metric";
    let url = format!(
        "{WEATHER_API}?lat={}&lon={}&appid={api_key}&units={units}",
        coord.0, coord.1
    );

    let resp = send_request(url, cli).await?;

    // parse respond
    let resp = resp.json::<ServiceRespond>().await?;
    // dbg!(&resp);
    match resp {
        ServiceRespond::Secondery(a) => Ok(a),
        ServiceRespond::Message(m) => Err(failure::err_msg(format!(
            "Failed to parse json couse: {}",
            m.message
        )))?,
        _ => Err(failure::err_msg("Failed to parse json"))?,
    }
}

async fn send_request(url: String, cli: &CliArgs) -> Result<Response, failure::Error> {
    if cli.verbose {
        println!("->> {:<12} - send_request: {}", "OPENWEATHERMAP", &url);
    }

    let url = Url::parse(&url)?;
    // dbg!(&url);

    let resp = reqwest::get(url).await?;
    Ok(resp)
}
