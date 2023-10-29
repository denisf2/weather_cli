// #[warn(dead_code)]

#[path = "./json/openweathermap.rs"]
mod openweathermap;

use exitfailure::ExitFailure;
use reqwest::Url;
use std::env;

use crate::openweathermap::{Root, Root3};

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

    let resp = reqwest::get(url).await?.json::<Root>().await?;
    // dbg!(&resp);

    Ok((resp[0].lat,resp[0].lon))
}

async fn get_forcast(coord: (f64, f64), api_key: &str) -> Result<Root3, ExitFailure> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={api_key}&units=metric",
        coord.0, coord.1
    );
    // dbg!(&url);
    let url = Url::parse(&url)?;
    // dbg!(&url);
    let resp = reqwest::get(url).await?.json::<Root3>().await?;
    // dbg!(&resp);

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // get cli args
    let args = env::args().collect::<Vec<String>>();
    // dbg!(&args);

    if args.len() < 2 {
        print_cli_help();

        return Ok(());
    }

    let city_code = &args[1];
    let country_code = &args[2];
    let api_key = &args[3];

    // get coordinate by city / country
    let coord = get_coords(city_code.as_str(), country_code.as_str(), api_key).await?;
    // dbg!(&coord);

    // get forcast by coordinate
    let forecast = get_forcast(coord, api_key).await?;

    // print forecast
    println!("Temperature in {} {} is {:?}°C", args[1], args[2], forecast.main.temp);

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
