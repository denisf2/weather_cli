use exitfailure::ExitFailure;

use crate::{
    app_settings::Settings,
    cli_arguments::CliArgs,
    ip2geo_client::{self, api_wrapper::*},
    owm_client::{self, api_wrapper::LocationInfo},
};

pub async fn get_coord_by_city(
    cli: &CliArgs,
    api_key: &Settings,
) -> Result<LocationInfo, ExitFailure> {
    let coord = match (&cli.city, &cli.country) {
        (Some(c), Some(co)) => {
            // get coordinate by city / country
            if cli.verbose {
                println!("->> {:<12} - Obtain location info by city name", "MAIN");
            }
            owm_client::api_wrapper::get_coords(
                c.as_str(),
                co.as_str(),
                api_key.owm_key.as_str(),
                cli,
            )
            .await?
        }

        (_, _) => {
            // condition key to find forecast by local ip
            if cli.verbose {
                println!("->> {:<12} - Detecting location by ip address", "MAIN");
            }
            let Coord { lat, lon } =
                ip2geo_client::api_wrapper::get_coord(api_key.ip2geo_key.as_str(), cli).await?;

            owm_client::api_wrapper::get_city_info(lat, lon, &api_key.owm_key, cli).await?
        }
    };

    Ok(coord)
}
