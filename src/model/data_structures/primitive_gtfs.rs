use chrono::NaiveDate;
use serde::{de, de::Unexpected, Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Agency {
    pub agency_id: String,
    pub agency_name: String,
    pub agency_url: String,
    pub agency_timezone: String,
    pub agency_lang: String,
    pub agency_phone: String,
}

#[derive(Debug, Deserialize)]
pub struct Route {
    pub route_id: String,
    pub agency_id: String,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_type: u32,
    pub route_url: Option<String>,
    pub route_color: Option<String>,
    pub route_text_color: Option<String>,
    #[serde(deserialize_with = "bool_from_int")]
    pub is_night: bool,
}

#[derive(Debug, Deserialize)]
pub struct Trip {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: Option<String>,
    pub trip_short_name: Option<String>,
    pub direction_id: u8,
    pub block_id: Option<String>,
    pub shape_id: Option<String>,
    pub wheelchair_accessible: Option<u8>,
    pub bikes_allowed: Option<u8>,
    pub exceptional: Option<u8>,
    pub trip_operation_type: Option<u8>,
    #[serde(default = "Vec::new", skip_deserializing)]
    pub stop_times: Vec<StopTime>,
}

#[derive(Debug, Deserialize)]
pub struct StopTime {
    pub trip_id: String,
    #[serde(deserialize_with = "deserialize_time")]
    // time of the day in seconds
    pub arrival_time: u32,
    #[serde(deserialize_with = "deserialize_time")]
    // time of the day in seconds
    pub departure_time: u32,
    pub stop_id: String,
    pub stop_sequence: u32,
    pub stop_headsign: Option<String>,
    pub pickup_type: u8,
    pub drop_off_type: u8,
    pub shape_dist_travelled: Option<f32>,
}

fn deserialize_ymd<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(NaiveDate::parse_from_str(&s, "%Y%m%d").unwrap())
}

fn deserialize_time<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let hms: Vec<u32> = s.split(":").map(|x| x.parse::<u32>().unwrap()).collect();
    return Ok(3600 * hms[0] + 60 * hms[1] + hms[2]);
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub service_id: String,
    #[serde(deserialize_with = "bool_from_int")]
    pub monday: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub tuesday: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub wednesday: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub thursday: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub friday: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub saturday: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub sunday: bool,
    #[serde(deserialize_with = "deserialize_ymd")]
    pub start_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_ymd")]
    pub end_date: NaiveDate,
    #[serde(default = "Vec::new", skip_deserializing)]
    pub exceptions: Vec<ServiceException>,
}

#[derive(Debug, Deserialize)]
pub struct ServiceException {
    pub service_id: String,
    #[serde(deserialize_with = "deserialize_ymd")]
    pub date: NaiveDate,
    pub exception_type: u8,
}
