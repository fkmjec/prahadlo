use crate::model::data_structures::*;
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// Loads the contents of stops.txt
/// # Arguments
/// * path - the path to the directory stops.txt is located in
fn load_stops(path: &Path) -> HashMap<String, Stop> {
    let mut stops = HashMap::new();
    let mut file_path_buf = path.to_path_buf();
    file_path_buf.push(Path::new("stops.txt"));
    let file = File::open(file_path_buf.as_path()).unwrap(); // No need for error handling, if this fails, we want to panic
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: Stop = result.unwrap();
        stops.insert(record.stop_id.clone(), record);
    }
    return stops;
}

#[test]
fn test_stop_loading() {
    let stops = load_stops(Path::new("test_data/"));
    assert_eq!(1, stops.len());
    let stop = stops.get("U50S1").unwrap();
    assert_eq!(stop.stop_id, "U50S1");
    assert_eq!(stop.stop_name, "Budějovická");
    assert_eq!(stop.stop_lat, 50.04441);
    assert_eq!(stop.stop_lon, 14.44879);
    assert_eq!(stop.zone_id, "P");
    assert_eq!(stop.stop_url, None);
    assert_eq!(stop.location_type, 1);
    assert_eq!(stop.parent_station, None);
    assert_eq!(stop.wheelchair_boarding, Some(1));
    assert_eq!(stop.level_id, None);
    assert_eq!(stop.platform_code, None);
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

#[test]
fn test_route_loading() {
    let routes = load_routes(Path::new("test_data/"));
    assert_eq!(1, routes.len());
    let route = routes.get("L991").unwrap();
    assert_eq!(route.route_id, "L991");
    assert_eq!(route.agency_id, "99");
    assert_eq!(route.route_short_name, "A");
    assert_eq!(
        route.route_long_name,
        "Nemocnice Motol - Petřiny - Skalka - Depo Hostivař"
    );
    assert_eq!(route.route_type, 1);
    assert_eq!(
        route.route_url,
        Some(String::from("https://pid.cz/linka/A"))
    );
    assert_eq!(route.route_color, Some(String::from("00A562")));
    assert_eq!(route.route_text_color, Some(String::from("FFFFFF")));
    assert_eq!(route.is_night, 0);
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

#[test]
fn test_trip_loading() {
    let trips = load_trips(Path::new("test_data/"));
    assert_eq!(trips.len(), 1);
    let trip = trips.get("991_1411_191224").unwrap();
    assert_eq!(trip.route_id, "L991");
    assert_eq!(trip.service_id, "0000010-1");
    assert_eq!(trip.trip_id, "991_1411_191224");
    assert_eq!(trip.trip_headsign, Some(String::from("Nemocnice Motol")));
    assert_eq!(trip.trip_short_name, None);
    assert_eq!(trip.direction_id, 0);
    assert_eq!(trip.block_id, None);
    assert_eq!(trip.shape_id, Some(String::from("L991V1")));
    assert_eq!(trip.wheelchair_accessible, Some(1));
    assert_eq!(trip.bikes_allowed, Some(1));
    assert_eq!(trip.exceptional, Some(0));
    assert_eq!(trip.trip_operation_type, Some(1));
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

#[test]
fn test_service_loading() {
    let services = load_services(Path::new("test_data/"));
    assert_eq!(services.len(), 1);
    let service = services.get("0000010-1").unwrap();
    assert_eq!(service.monday, 0);
    assert_eq!(service.tuesday, 0);
    assert_eq!(service.wednesday, 0);
    assert_eq!(service.thursday, 0);
    assert_eq!(service.friday, 0);
    assert_eq!(service.saturday, 1);
    assert_eq!(service.sunday, 0);
    assert_eq!(service.start_date, NaiveDate::from_ymd(2020, 1, 25));
    assert_eq!(service.end_date, NaiveDate::from_ymd(2020, 2, 7))
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
        services
            .get_mut(&record.service_id)
            .unwrap()
            .exceptions
            .push(record);
    }
}

fn load_stop_times(path: &Path, trips: &mut HashMap<String, Trip>) {
    let mut file_path_buf = path.to_path_buf();
    file_path_buf.push(Path::new("stop_times.txt"));
    let file = File::open(file_path_buf.as_path()).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let stop_time: StopTime = result.unwrap();
        trips
            .get_mut(&stop_time.trip_id)
            .unwrap()
            .stop_times
            .push(stop_time);
    }
    for trip in trips.values_mut() {
        trip.stop_times
            .sort_by(|a, b| a.stop_sequence.cmp(&b.stop_sequence));
    }
}

pub fn load_transport_network(path: &Path) -> Network {
    let mut stops = load_stops(path);
    let routes = load_routes(path);
    let mut services = load_services(path);
    load_service_exceptions(path, &mut services);
    let mut trips = load_trips(path);
    load_stop_times(path, &mut trips);
    let mut nodes: Vec<Node> = Vec::new();
    let mut current_node_index: usize = 0;
    for trip in trips.values() {
        for i in 1..trip.stop_times.len() {
            let mut dep_node = Node::new(
                trip.stop_times[i - 1].stop_id.clone(),
                NodeKind::Dep(trip.stop_times[i - 1].departure_time.clone()),
            );
            let route = routes.get(trip.route_id.as_str()).unwrap();
            dep_node.add_edge(Edge::new(
                trip.stop_times[i - 1].departure_time.clone(),
                trip.stop_times[i].arrival_time.clone(),
                Some(trip.trip_id.clone()),
                route.route_type,
                current_node_index + 1,
            ));
            nodes.push(dep_node);
            let stop = stops.get_mut(&trip.stop_times[i - 1].stop_id).unwrap();
            stop.departure_nodes.push(current_node_index);
            let arr_node = Node::new(
                trip.stop_times[i].stop_id.clone(),
                NodeKind::Arr(trip.stop_times[i].arrival_time.clone()),
            );
            current_node_index += 2;
            nodes.push(arr_node);
        }
    }
    return Network::new(nodes);
}
