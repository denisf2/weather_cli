// #[warn(dead_code)]

mod app_settings;
mod ip2geo_client;
mod owm_client;

use exitfailure::ExitFailure;
use std::env;

use owm_client::json_structs::Forecast;

fn print_cli_help() {
    println!("->> {:<12} - print_cli_help", "HELP");

    // write using help
    println!("Getting forcast\r\n");
    println!("Usage:\r\n\tpass as arguments city and country code");
    println!("\texample: London GB");
}

fn print_forecast(args: &[String], forecast: Forecast) {
    println!("->> {:<12} - print_forecast", "OUTPUT");

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

    // get api keys from locale file
    let api_key = app_settings::get_api_keys(&args[3]).await?;

    // get coordinate by city / country
    let coord = owm_client::api_wrapper::get_coords(
        city_code.as_str(),
        country_code.as_str(),
        api_key.owm_key.as_str(),
    )
    .await?;
    // dbg!(&coord);

    // get forcast by coordinate
    let forecast = owm_client::api_wrapper::get_forcast(coord, api_key.owm_key.as_str()).await?;

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
