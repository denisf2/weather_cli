// #[warn(dead_code)]

mod app_settings;
mod ip2geo_client;
mod owm_client;

use std::path::PathBuf;

use clap::Parser;
use exitfailure::ExitFailure;
use ip2geo_client::api_wrapper::Coord;
use owm_client::json_structs::Forecast;
use simple_home_dir::*;

const DEFAULT_CONFIG_PATH: &str = "~/.weather_cli";


#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    // City name
    #[arg(short = 'C', long, help = "A city definition")]
    city: Option<String>,

    // Country
    #[arg(short = 'O', long, help = "Country of the city")]
    country: Option<String>,

    // The path to the file to read
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Path to the configuration file"
    )]
    config: Option<PathBuf>,

    // verbose flag
    #[arg(short, long, help = "Verbose mode")]
    pub verbose: bool,
}

fn print_forecast(forecast: Forecast, cli: &CliArgs) {
    if cli.verbose {
        println!("->> {:<12} - print_forecast", "OUTPUT");
    }

    println!(
        "Temperature in {} {} is {}°C",
        forecast.name, forecast.sys.country, forecast.main.temp
    );
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // get clap args
    let cli = CliArgs::parse();

    let conf_path = cli.clone().config.unwrap_or_else(|| {
        let path = expand_tilde(DEFAULT_CONFIG_PATH).unwrap();
        if cli.verbose {
            println!(
                "->> {:<12} - Using default config path: {}",
                "CONFIGURE",
                &path.to_str().unwrap()
            );
        }
        path
    });

    if !conf_path.is_file() {
        if cli.verbose {
            println!(
                "->> {:<12} - File {} does not exist",
                "CONFIGURE",
                conf_path.to_str().unwrap()
            );
        }
        return Ok(());
    }

    let api_key = app_settings::get_api_keys(&conf_path.as_path(), &cli).await?;

    let coord = match (&cli.city, &cli.country) {
        (Some(c), Some(co)) => {
            // get coordinate by city / country
            owm_client::api_wrapper::get_coords(
                c.as_str(),
                co.as_str(),
                api_key.owm_key.as_str(),
                &cli,
            )
            .await?
        }

        (_, _) => {
            // condition key to find forecast by local ip
            let Coord { lat, lon } =
                ip2geo_client::api_wrapper::get_coord(api_key.ip2geo_key.as_str(), &cli).await?;
            owm_client::api_wrapper::get_city_info(lat, lon, &api_key.owm_key, &cli).await?
        }
    };
    // dbg!(&coord);

    let forecast = owm_client::api_wrapper::get_forcast(
        (coord.lat, coord.lon),
        api_key.owm_key.as_str(),
        &cli,
    )
    .await?;
    // dbg!(&forecast);

    print_forecast(forecast, &cli);

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
