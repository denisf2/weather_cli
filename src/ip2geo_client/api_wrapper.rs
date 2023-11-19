use super::json_structs::ServiceRespond;
use crate::CliArgs;
use exitfailure::ExitFailure;
use futures::executor::block_on;
use reqwest::Url;
use std::net::IpAddr;

const IP2GEO_API: &str = "https://api.ipgeolocation.io/ipgeo";
#[derive(Debug)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}

pub async fn get_coord(api_key: &str, cli: &CliArgs) -> Result<Coord, ExitFailure> {
    if cli.verbose {
        println!("->> {:<12} - get_coord", "IPGEOLOCATION");
    }

    let ipaddr = get_ip(cli);
    let resp = get_coord_service(ipaddr, api_key, cli).await?;
    Ok(resp)
}

async fn get_coord_service(
    ip: IpAddr,
    api_key: &str,
    cli: &CliArgs,
) -> Result<Coord, failure::Error> {
    if cli.verbose {
        println!("->> {:<12} - get_coord_service", "IPGEOLOCATION");
    }
    // send request
    let url = format!("{IP2GEO_API}?apiKey={api_key}&ip={ip}");

    if cli.verbose {
        println!("->> {:<12} - Sending request: {}", "IPGEOLOCATION", &url);
    }
    let url = Url::parse(&url)?;

    let resp = reqwest::get(url).await?;

    // parse respond
    let resp = resp.json::<ServiceRespond>().await?;
    match resp {
        ServiceRespond::Main(a) => {
            let lat = a.latitude.parse::<f64>()?;
            let lon = a.longitude.parse::<f64>()?;
            Ok(Coord { lat, lon })
        }
        ServiceRespond::Message(m) => Err(failure::err_msg(format!(
            "Failed to parse json couse: {}",
            m.message
        ))),
    }
}

fn get_ip(cli: &CliArgs) -> IpAddr {
    if cli.verbose {
        println!("->> {:<12} - get_ip", "IPGEOLOCATION");
    }

    let result = external_ip::get_ip();
    let value = block_on(result).unwrap();
    // dbg!(&value);

    value
}

// get geo coords by ip
