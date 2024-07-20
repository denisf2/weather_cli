mod app_settings;
mod cli_arguments;
mod ip2geo_client;
mod owm_client;
mod utils;

pub use cli_arguments::CliArgs;
pub use owm_client::json_structs::Forecast;

use std::path::PathBuf;
use exitfailure::ExitFailure;
use owm_client::api_wrapper::get_forcast;
use utils::get_coord_by_city;

pub async fn get_forecast(conf_path: PathBuf, cli: &CliArgs) -> Result<Box<Forecast>, ExitFailure> {
    let api_key = app_settings::get_api_keys(conf_path.as_path(), cli).await?;

    let coord = get_coord_by_city(cli, &api_key).await?;
    // dbg!(&coord);

    let forecast = get_forcast((coord.lat, coord.lon), api_key.owm_key.as_str(), cli).await?;
    // dbg!(&forecast);

    Ok(forecast)
}
