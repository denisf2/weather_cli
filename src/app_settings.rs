use exitfailure::ExitFailure;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub async fn get_api_key(path: &str) -> Result<String, ExitFailure> {
    println!("->> {:<12} - get_api_key", "SETTINGS");

    let path = Path::new(path);

    let mut file = File::open(&path)?;
    let mut s = String::new();
    let _ = file.read_to_string(&mut s)?;

    Ok(s)
}
