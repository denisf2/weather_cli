// #[warn(dead_code)]

#[path = "./json/openweathermap.rs"]
mod openweathermap;

use exitfailure::ExitFailure;
use reqwest::Url;
use std::io::Read;
use std::path::Path;
use std::{env, fs::File};

use crate::openweathermap::{CoordsVec, Forecast};

fn print_cli_help() {
    // write using help
    println!("Getting forcast\r\n");
    println!("Usage:\r\n\tpass as arguments city and country code");
    println!("\texample: London GB");
}

async fn get_coords(
    city_name: &str,
    country_code: &str,
    api_key: &str,
) -> Result<(f64, f64), ExitFailure> {
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

async fn get_forcast(coord: (f64, f64), api_key: &str) -> Result<Forecast, ExitFailure> {
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

async fn get_api_key(path: &str) -> Result<String, ExitFailure> {
    let path = Path::new(path);

    let mut file = File::open(&path)?;
    let mut s = String::new();
    let _ = file.read_to_string(&mut s)?;

    Ok(s)
}

fn print_forecast(args: &[String], forecast: Forecast) {
    println!(
        "Temperature in {} {} is {:?}°C",
        args[1], args[2], forecast.main.temp
    );
}


#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // get cli args
    let args = env::args().collect::<Vec<String>>();
    // dbg!(&args);

    //check cli arguments number
    if args.len() < 2 {
        print_cli_help();

        return Ok(());
    }

    let city_code = &args[1];
    let country_code = &args[2];

    // get api key from locale file
    let api_key = get_api_key(&args[3]).await?;

    // get coordinate by city / country
    let coord = get_coords(city_code.as_str(), country_code.as_str(), api_key.as_str()).await?;
    // dbg!(&coord);

    // get forcast by coordinate
    let forecast = get_forcast(coord, api_key.as_str()).await?;

    // print forecast
    print_forecast(args.as_slice(), forecast);

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
