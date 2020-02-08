use crate::transport_data_structures::*;
use std::str::FromStr;
use std::num::ParseIntError;
use std::path::Path;
use std::fs::File;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::collections::HashMap;
use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Agency {
    pub agency_id: String,
    pub agency_name: String,
    pub agency_url: String,
    pub agency_timezone: String,
    pub agency_lang: String,
    pub agency_phone: String,
}

#[derive(Debug, Deserialize)]
struct Stop {
    pub stop_id: String,
    pub stop_name: String,
    pub stop_lat: f32,
    pub stop_lon: f32,
    pub zone_id: String,
    pub stop_url: Option<String>,
    pub location_type: i32,
    pub parent_station: Option<String>,
    pub wheelchair_boarding: Option<i32>,
    pub level_id: Option<i32>,
    pub platform_code: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct Route {
    pub route_id: String,
    pub agency_id: String,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_type: i32,
    pub route_url: Option<String>,
    pub route_color: Option<String>,
    pub route_text_color: Option<String>,
    pub is_night: String,
}

#[derive(Debug, Deserialize)]
struct Trip {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: Option<String>,
    pub trip_short_name: Option<String>,
    pub direction_id: i32,
    pub block_id: Option<String>,
    pub shape_id: Option<String>,
    pub wheelchair_accessible: Option<i32>,
    pub bikes_allowed: i32,
    pub exceptional: i32,
    pub trip_operation_type: i32,
}

#[derive(Debug, Deserialize)]
struct StopTime {
    pub trip_id: String,
    pub arrival_time: NaiveTime,
    pub departure_time: NaiveTime,
    pub stop_id: String,
    pub stop_sequence: i32,
    pub stop_headsign: Option<String>,
    pub pickup_type: i32,
    pub drop_off_type: i32,
    pub shape_dist_travelled: i32,
}

#[derive(Debug, Deserialize)]
struct Service {
    pub service_id: String, 
    pub monday: i32,
    pub tuesday:i32,
    pub wednesday: i32,
    pub thursday: i32,
    pub friday: i32,
    pub saturday: i32,
    pub sunday: i32,
    pub start_date: String, // Placeholder String
    pub end_date: String, // Placeholder String
    #[serde(default = "Vec::new", skip_deserializing)]
    pub exceptions: Vec<ServiceException>,
}

#[derive(Debug, Deserialize)]
struct ServiceException {
    pub service_id: String,
    pub date: String, // Placeholder String
    pub exception_type: i32,
}

/// Loads the contents of stops.txt
/// # Arguments
/// * path - the path to the directory stops.txt is located in
fn load_stops(path: &Path) -> HashMap<String, Node> {
    let mut stops = HashMap::new();
    let mut file_path_buf = path.to_path_buf();
    file_path_buf.push(Path::new("stops.txt"));
    let file = File::open(file_path_buf.as_path()).unwrap(); // No need for error handling, if this fails, we want to panic
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: Stop = result.unwrap();
        let node = Node::new(record.stop_name.clone(), record.stop_lat.clone(), record.stop_lon.clone(), record.location_type.clone());
        stops.insert(record.stop_id.clone(), node);
    }
    return stops;
}

/// Loads the contents of routes.txt
/// # Arguments
/// * path - the path to the directory routes.txt is located in
fn load_routes(path: &Path) -> HashMap<String, Route> {
    let mut routes = HashMap::new();
    let mut file_path_buf = path.to_path_buf();
    file_path_buf.push(Path::new("routes.txt"));
    let file = File::open(file_path_buf.as_path()).unwrap(); // No need for error handling, if this fails, we want to panic
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: Route = result.unwrap();
        routes.insert(record.route_id.clone(), record);
    }
    return routes;
}

/// Loads the contents of trips.txt
/// # Arguments
/// * path - the path to the directory trips.txt is located in
fn load_trips(path: &Path) -> HashMap<String, Trip> {
    let mut trips = HashMap::new();
    let mut file_path_buf = path.to_path_buf();
    file_path_buf.push(Path::new("trips.txt"));
    let file = File::open(file_path_buf.as_path()).unwrap(); // No need for error handling, if this fails, we want to panic
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: Trip = result.unwrap();
        trips.insert(record.trip_id.clone(), record);
    }
    return trips;
}

/// Parses a string in YYYYMMDD format into NaiveDate
/// # Arguments
/// * raw_ymd - YYYYMMDD
fn parse_ymd(raw_ymd: &str) -> NaiveDate {
    NaiveDate::from_ymd(raw_ymd[0..4].parse::<i32>().unwrap(), raw_ymd[4..6].parse::<u32>().unwrap(), raw_ymd[6..].parse::<u32>().unwrap())
}

/// Loads the contents of services.txt and service_dates.txt
/// # Arguments
/// * path - the path to the directory the files are located in
fn load_services(path: &Path) -> HashMap<String, Service> {
    let mut services = HashMap::new();
    let mut file_path_buf = path.to_path_buf();
    file_path_buf.push(Path::new("calendar.txt"));
    let file = File::open(file_path_buf.as_path()).unwrap(); // No need for error handling, if this fails, we want to panic
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: Service = result.unwrap();
        services.insert(record.service_id.clone(), record);
    }
    return services;
}

/// Loads service exceptions from calendar_dates.txt and adds them to the HashMap
/// # Arguments
/// * path - the path to the gtfs directory
/// * services - loaded contents of calendar.txt
fn load_service_exceptions(path: &Path, services: &mut HashMap<String, Service>) {
    let mut file_path_buf = path.to_path_buf();
    file_path_buf.push(Path::new("calendar_dates.txt"));
    let file = File::open(file_path_buf.as_path()).unwrap(); // No need for error handling, if this fails, we want to panic
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: ServiceException = result.unwrap();
        services.get_mut(&record.service_id).unwrap().exceptions.push(record);
    }
}

/// Creates the edges to be used in the inner workings of the search algorithm
/// # Arguments
/// * path - the path to the gtfs directory
/// * stops - loaded contents of stops.txt
/// * routes - loaded contents of routes.txt
/// * trips - loaded contents of trips.txt
/// * services - loaded contents of services.txt and service_dates.txt, specifying the dates
/// the service is available on
fn fill_edges(path: &Path, stops: &mut HashMap<String, Node>, routes: &HashMap<String, Route>, trips: &HashMap<String, Trip>, services: &HashMap<String, Service>) {
    let mut file_path_buf = path.to_path_buf();
    file_path_buf.push(Path::new("calendar_dates.txt"));
    let file = File::open(file_path_buf.as_path()).unwrap(); // No need for error handling, if this fails, we want to panic
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {

    } 
}

pub fn load_transport_network(path: &Path) -> Network {
    // TODO load individual GTFS files 
    Network::new(HashMap::new())
} 
