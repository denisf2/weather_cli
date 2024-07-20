// #[warn(dead_code)]
extern crate failure;

use clap::Parser;
use exitfailure::ExitFailure;
use simple_home_dir::*;
use weather_cli::{get_forecast, CliArgs, Forecast};

const DEFAULT_CONFIG_PATH: &str = "~/.weather_cli";

fn print_forecast(forecast: Box<Forecast>, cli: &CliArgs) {
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

    conf_path
        .try_exists()
        .expect("The config file should exist and contains api keys");

    let forecast = get_forecast(conf_path, &cli).await?;
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
