use exitfailure::ExitFailure;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(rename = "owm_key")]
    pub owm_key: String,
    #[serde(rename = "ip2geo_key")]
    pub ip2geo_key: String,
}

pub async fn get_api_keys(path: &Path) -> Result<Settings, ExitFailure> {
    println!("->> {:<12} - get_api_keys", "SETTINGS");

    let mut file = File::open(&path)?;
    let mut data = String::new();
    let _ = file.read_to_string(&mut data)?;
    
    let settings: Settings = serde_json::from_str(data.as_str())?;

    Ok(settings)
}
