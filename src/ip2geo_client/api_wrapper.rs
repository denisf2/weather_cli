use super::json_structs::Root;
use exitfailure::ExitFailure;
use futures::executor::block_on;
use reqwest::Url;
use std::net::IpAddr;

const IP2GEO_API: &str = "https://api.ipgeolocation.io/ipgeo";
#[derive(Debug)]
pub struct Coord {
    lat: f64,
    lon: f64,
}

pub fn get_coord(api_key: &str) -> Coord {
    println!("->> {:<12} - get_coord", "IPGEOLOCATION");

    let ipaddr = get_ip();
    let coord = get_coord_service(ipaddr, api_key);
    let _a = block_on(coord).unwrap();

    _a
}

async fn get_coord_service(ip: IpAddr, api_key: &str) -> Result<Coord, ExitFailure> {
    println!("->> {:<12} - get_coord_service", "IPGEOLOCATION");

    let url = format!("{IP2GEO_API}?apiKey={api_key}&ip={ip}");
    let url = Url::parse(&url)?;

    let resp = reqwest::get(url).await?.json::<Root>().await?;
    let lat = resp.latitude.as_str().parse::<f64>().unwrap();
    let lon = resp.longitude.as_str().parse::<f64>().unwrap();

    Ok(Coord { lat, lon })
}

fn get_ip() -> IpAddr {
    println!("->> {:<12} - get_ip", "IPGEOLOCATION");

    let result = external_ip::get_ip();
    let value: Option<IpAddr> = block_on(result);
    dbg!(&value);

    value.unwrap()
}

// get geo coords by ip
