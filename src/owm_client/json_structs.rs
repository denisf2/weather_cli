use serde::{Deserialize, Serialize};


/// get coordinates
pub type CoordsVec = Vec<CityCoord>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CityCoord {
    pub name: String,
    #[serde(rename = "local_names")]
    pub local_names: Option<LocalNames>,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalNames {
    pub kn: Option<String>,
    pub tg: Option<String>,
    pub gu: Option<String>,
    pub tw: Option<String>,
    pub co: Option<String>,
    pub ka: Option<String>,
    #[serde(rename = "feature_name")]
    pub feature_name: Option<String>,
    pub am: Option<String>,
    pub sw: Option<String>,
    pub av: Option<String>,
    pub ht: Option<String>,
    pub jv: Option<String>,
    pub tt: Option<String>,
    pub mi: Option<String>,
    pub sr: Option<String>,
    pub om: Option<String>,
    pub is: Option<String>,
    pub su: Option<String>,
    pub my: Option<String>,
    pub si: Option<String>,
    pub st: Option<String>,
    pub gd: Option<String>,
    pub wa: Option<String>,
    pub th: Option<String>,
    pub lv: Option<String>,
    pub sm: Option<String>,
    pub hy: Option<String>,
    pub mk: Option<String>,
    pub kw: Option<String>,
    pub ur: Option<String>,
    pub io: Option<String>,
    pub mr: Option<String>,
    pub vi: Option<String>,
    pub hu: Option<String>,
    pub ko: Option<String>,
    pub pa: Option<String>,
    pub bs: Option<String>,
    pub pl: Option<String>,
    pub ay: Option<String>,
    pub el: Option<String>,
    pub sh: Option<String>,
    pub et: Option<String>,
    pub ps: Option<String>,
    pub oc: Option<String>,
    pub sk: Option<String>,
    pub fa: Option<String>,
    pub se: Option<String>,
    pub ga: Option<String>,
    pub lb: Option<String>,
    pub ln: Option<String>,
    pub km: Option<String>,
    pub ro: Option<String>,
    pub lt: Option<String>,
    pub de: Option<String>,
    pub sl: Option<String>,
    pub tk: Option<String>,
    pub hr: Option<String>,
    pub gn: Option<String>,
    pub ia: Option<String>,
    pub mt: Option<String>,
    pub sv: Option<String>,
    pub hi: Option<String>,
    pub sd: Option<String>,
    pub it: Option<String>,
    pub af: Option<String>,
    pub fi: Option<String>,
    pub ny: Option<String>,
    pub lo: Option<String>,
    pub or: Option<String>,
    pub fy: Option<String>,
    pub mn: Option<String>,
    pub os: Option<String>,
    pub bh: Option<String>,
    pub no: Option<String>,
    pub yi: Option<String>,
    pub uz: Option<String>,
    pub pt: Option<String>,
    pub id: Option<String>,
    pub en: Option<String>,
    pub rm: Option<String>,
    pub sn: Option<String>,
    pub bm: Option<String>,
    pub kk: Option<String>,
    pub bg: Option<String>,
    pub cs: Option<String>,
    pub bi: Option<String>,
    pub kl: Option<String>,
    pub an: Option<String>,
    pub bn: Option<String>,
    pub kv: Option<String>,
    pub ky: Option<String>,
    pub zh: Option<String>,
    pub uk: Option<String>,
    pub nv: Option<String>,
    pub da: Option<String>,
    pub gl: Option<String>,
    pub ba: Option<String>,
    pub es: Option<String>,
    pub so: Option<String>,
    pub ru: Option<String>,
    pub ml: Option<String>,
    pub sa: Option<String>,
    pub fr: Option<String>,
    pub ku: Option<String>,
    pub cy: Option<String>,
    pub ie: Option<String>,
    pub be: Option<String>,
    pub he: Option<String>,
    pub sc: Option<String>,
    pub yo: Option<String>,
    pub eu: Option<String>,
    pub ab: Option<String>,
    pub te: Option<String>,
    pub fj: Option<String>,
    pub eo: Option<String>,
    pub li: Option<String>,
    pub ja: Option<String>,
    pub gv: Option<String>,
    pub nl: Option<String>,
    pub na: Option<String>,
    pub ar: Option<String>,
    pub sq: Option<String>,
    pub vo: Option<String>,
    pub tl: Option<String>,
    pub fo: Option<String>,
    pub br: Option<String>,
    pub ha: Option<String>,
    pub tr: Option<String>,
    pub zu: Option<String>,
    pub cv: Option<String>,
    pub az: Option<String>,
    pub mg: Option<String>,
    pub ca: Option<String>,
    pub ff: Option<String>,
    pub ug: Option<String>,
    pub qu: Option<String>,
    pub ne: Option<String>,
    pub ms: Option<String>,
    pub ascii: Option<String>,
    pub cu: Option<String>,
    pub ee: Option<String>,
    pub ce: Option<String>,
    pub bo: Option<String>,
    pub ta: Option<String>,
    pub wo: Option<String>,
    pub to: Option<String>,
    pub nn: Option<String>,
    pub ig: Option<String>,
}

/// get forecast
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub rain: Option<Rain>,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    #[serde(rename = "temp_min")]
    pub temp_min: f64,
    #[serde(rename = "temp_max")]
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(rename = "sea_level")]
    pub sea_level: Option<i64>,
    #[serde(rename = "grnd_level")]
    pub grnd_level: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rain {
    #[serde(rename = "1h")]
    pub n1h: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clouds {
    pub all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub id: i64,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}
