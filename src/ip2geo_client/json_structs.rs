use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub ip: String,
    #[serde(rename = "continent_code")]
    pub continent_code: String,
    #[serde(rename = "continent_name")]
    pub continent_name: String,
    #[serde(rename = "country_code2")]
    pub country_code2: String,
    #[serde(rename = "country_code3")]
    pub country_code3: String,
    #[serde(rename = "country_name")]
    pub country_name: String,
    #[serde(rename = "country_name_official")]
    pub country_name_official: String,
    #[serde(rename = "country_capital")]
    pub country_capital: String,
    #[serde(rename = "state_prov")]
    pub state_prov: String,
    #[serde(rename = "state_code")]
    pub state_code: String,
    pub district: String,
    pub city: String,
    pub zipcode: String,
    pub latitude: String,
    pub longitude: String,
    #[serde(rename = "is_eu")]
    pub is_eu: bool,
    #[serde(rename = "calling_code")]
    pub calling_code: String,
    #[serde(rename = "country_tld")]
    pub country_tld: String,
    pub languages: String,
    #[serde(rename = "country_flag")]
    pub country_flag: String,
    #[serde(rename = "geoname_id")]
    pub geoname_id: String,
    pub isp: String,
    #[serde(rename = "connection_type")]
    pub connection_type: String,
    pub organization: String,
    pub currency: Currency,
    #[serde(rename = "time_zone")]
    pub time_zone: TimeZone,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeZone {
    pub name: String,
    pub offset: i64,
    #[serde(rename = "offset_with_dst")]
    pub offset_with_dst: i64,
    #[serde(rename = "current_time")]
    pub current_time: String,
    #[serde(rename = "current_time_unix")]
    pub current_time_unix: f64,
    #[serde(rename = "is_dst")]
    pub is_dst: bool,
    #[serde(rename = "dst_savings")]
    pub dst_savings: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceRespond {
    Data(Box<Location>),
    Error(Message),
}
