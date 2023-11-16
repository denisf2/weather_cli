// #[warn(dead_code)]

mod app_settings;
mod ip2geo_client;
mod owm_client;

use clap::Parser;
use exitfailure::ExitFailure;
use ip2geo_client::api_wrapper::Coord;
use owm_client::json_structs::Forecast;

#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    // City name
    #[arg(short = 'C', long)]
    city: Option<String>,

    // Country
    #[arg(short = 'O', long)]
    country: Option<String>,

    // The path to the file to read
    #[arg(short, long, value_name = "FILE")]
    config: std::path::PathBuf,

    // verbose flag
    #[arg(short, long)]
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

    if !cli.config.is_file() {
        println!("File {} does not exist", cli.config.to_str().unwrap());
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
