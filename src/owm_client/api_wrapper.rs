use super::json_structs::{CoordsVec, Forecast};

use exitfailure::ExitFailure;
use reqwest::Url;

const GEO_API: &str = "https://api.openweathermap.org/geo/1.0/direct";
const WEATHER_API: &str = "https://api.openweathermap.org/data/2.5/weather";
const GEO_REVERSE_API: &str = "http://api.openweathermap.org/geo/1.0/reverse";

#[derive(Debug, Clone)]
pub struct PlaceInfo {
    pub city: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
}

pub async fn get_city_info(lat: f64, lon: f64, api_key: &str) -> Result<PlaceInfo, ExitFailure> {
    println!("->> {:<12} - get_city_info", "OPENWEATHERMAP");

    let limit = 1;
    let url = format!("{GEO_REVERSE_API}?lat={lat}&lon={lon}&limit={limit}&appid={api_key}");
    // dbg!(&url);

    let url = Url::parse(&url)?;
    // dbg!(&url);

    let resp = reqwest::get(url).await?.json::<CoordsVec>().await?;
    // dbg!(&resp);

    Ok(PlaceInfo {
        city: resp[0].name.clone(),
        country: resp[0].country.clone(),
        lat,
        lon,
    })
}

pub async fn get_coords(
    city_name: &str,
    country_code: &str,
    api_key: &str,
) -> Result<PlaceInfo, ExitFailure> {
    println!("->> {:<12} - get_coords", "OPENWEATHERMAP");

    let state_code = "";
    let limit = 3;
    let url = format!(
        "{GEO_API}?q={city_name},{state_code},{country_code}&limit={limit}&appid={api_key}"
    );
    // dbg!(&url);

    let url = Url::parse(&url)?;
    // dbg!(&url);

    let resp = reqwest::get(url).await?.json::<CoordsVec>().await?;
    // dbg!(&resp);

    Ok(PlaceInfo {
        city: city_name.to_string(),
        country: country_code.to_string(),
        lat: resp[0].lat,
        lon: resp[0].lon,
    })
}

pub async fn get_forcast(coord: (f64, f64), api_key: &str) -> Result<Forecast, ExitFailure> {
    println!("->> {:<12} - get_forcast", "OPENWEATHERMAP");

    let units = "metric";
    let url = format!(
        "{WEATHER_API}?lat={}&lon={}&appid={api_key}&units={units}",
        coord.0, coord.1
    );

    // dbg!(&url);
    let url = Url::parse(&url)?;
    // dbg!(&url);
    let resp = reqwest::get(url).await?.json::<Forecast>().await?;
    // dbg!(&resp);

    Ok(resp)
}
