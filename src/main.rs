// #[warn(dead_code)]

mod app_settings;
mod ip2geo_client;
mod owm_client;

use clap::Parser;
use exitfailure::ExitFailure;
use owm_client::json_structs::Forecast;

#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    // City name
    #[arg(short = 'C', long)]
    city: Option<String>,

    // Country
    #[arg(short = 'O', long)]
    country: Option<String>,

    // The path to the file to read
    #[arg(short, long, value_name = "FILE")]
    config: std::path::PathBuf,
}

fn print_forecast(city: &str, country: &str, forecast: Forecast) {
    println!("->> {:<12} - print_forecast", "OUTPUT");

    println!(
        "Temperature in {city} {country} is {}°C",
        forecast.main.temp
    );
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // get clap args
    let cli = CliArgs::parse();

    if !cli.config.is_file() {
        println!("File {} does not exist", cli.config.to_str().unwrap());
        return Ok(());
    }

    // get api keys from locale file
    let api_key = app_settings::get_api_keys(cli.config.as_path()).await?;

    let coord = match (cli.city, cli.country) {
        (Some(c), Some(co)) => {
            // get coordinate by city / country
            owm_client::api_wrapper::get_coords(c.as_str(), co.as_str(), api_key.owm_key.as_str())
                .await?
        }

        (_, _) => {
            // condition key to find forecast by local ip
            let c = ip2geo_client::api_wrapper::get_coord(api_key.ip2geo_key.as_str());
            (c.lat, c.lon)
        }
    };
    dbg!(&coord);

    // get forcast by coordinate
    let forecast = owm_client::api_wrapper::get_forcast(coord, api_key.owm_key.as_str()).await?;

    // print forecast
    print_forecast("city", "country", forecast);

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
