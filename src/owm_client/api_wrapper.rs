use super::json_structs::{CoordsVec, Forecast};

use exitfailure::ExitFailure;
use reqwest::Url;

pub async fn get_coords(
    city_name: &str,
    country_code: &str,
    api_key: &str,
) -> Result<(f64, f64), ExitFailure> {
    println!("->> {:<12} - get_coords", "OPENWEATHERMAP");

    let state_code = String::new();
    let limit = 3;
    let url = format!("https://api.openweathermap.org/geo/1.0/direct?q={city_name},{state_code},{country_code}&limit={limit}&appid={api_key}");
    // dbg!(&url);

    let url = Url::parse(&url)?;
    // dbg!(&url);

    let resp = reqwest::get(url).await?.json::<CoordsVec>().await?;
    // dbg!(&resp);

    Ok((resp[0].lat, resp[0].lon))
}

pub async fn get_forcast(coord: (f64, f64), api_key: &str) -> Result<Forecast, ExitFailure> {
    println!("->> {:<12} - get_forcast", "OPENWEATHERMAP");

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={api_key}&units=metric",
        coord.0, coord.1
    );

    // dbg!(&url);
    let url = Url::parse(&url)?;
    // dbg!(&url);
    let resp = reqwest::get(url).await?.json::<Forecast>().await?;
    // dbg!(&resp);

    Ok(resp)
}
