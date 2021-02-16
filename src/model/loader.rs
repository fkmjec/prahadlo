use crate::model::primitive_gtfs::*;
use crate::model::state_representation::*;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use chrono::NaiveDate;
use geo_types::Point;
use proj::Proj;

const MAX_PEDESTRIAN_DIST: f32 = 500.0;
const PEDESTRIAN_SPEED: f32 = 3.6;
const BASE_PEDESTRIAN_TRANSFER_TIME: f32 = 60.0;

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
    assert_eq!(route.is_night, false);
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
    assert_eq!(service.monday, false);
    assert_eq!(service.tuesday, false);
    assert_eq!(service.wednesday, false);
    assert_eq!(service.thursday, false);
    assert_eq!(service.friday, false);
    assert_eq!(service.saturday, true);
    assert_eq!(service.sunday, false);
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

/// Converts stop coordinates in WGS84 to UTM coordinates in zone 33U
fn get_stop_coords_in_utm(stops: &HashMap<String, Stop>) -> HashMap<String, Point<f32>> {
    let mut stop_coords: HashMap<String, Point<f32>> = HashMap::new();
    for (stop_id, stop) in stops {
        let from = "EPSG:4326";
        let to = "EPSG:32633";
        let wsg_to_utm = Proj::new_known_crs(&from, &to, None).unwrap();
        let wsg_coords = Point::new(stop.stop_lon, stop.stop_lat);
        let coords = wsg_to_utm.convert(wsg_coords).unwrap();
        stop_coords.insert(stop_id.clone(), coords);
    }
    return stop_coords;
}

/// Takes stop coords in utm and a maximum connections distance. Divides the stops into squares of
/// size max_connection_dist * max_connection_dist.
fn calculate_proximity_squares(
    utm_coords: &HashMap<String, Point<f32>>,
    max_connection_dist: f32,
) -> HashMap<(i32, i32), Vec<String>> {
    let mut squares: HashMap<(i32, i32), Vec<String>> = HashMap::new();
    for (stop_id, utm) in utm_coords {
        let square_coords = (
            (utm.x() / max_connection_dist) as i32,
            (utm.y() / max_connection_dist) as i32,
        );
        if squares.contains_key(&square_coords) {
            squares
                .get_mut(&square_coords)
                .unwrap()
                .push(String::from(stop_id));
        } else {
            squares.insert(square_coords, vec![String::from(stop_id)]);
        }
    }
    return squares;
}

/// Takes squares of sizes max_conn_dist times max_conn_dist that contain stops in utm coordinates,
/// and it efficiently computes connections between stops closer than max_conn_dist. (efficiently means faster than
/// O(N^2) N being the number of all stops.
fn get_pedestrian_connections(
    utm_coords: &HashMap<String, Point<f32>>,
    squares: &HashMap<(i32, i32), Vec<String>>,
    max_conn_dist: f32,
) -> HashMap<String, Vec<(String, f32)>> {
    let mut connections: HashMap<String, Vec<(String, f32)>> = HashMap::new();
    for ((x, y), stop_ids) in squares {
        for stop_id in stop_ids {
            let coord = utm_coords.get(stop_id).unwrap();
            for dx in -1..2 {
                for dy in -1..2 {
                    if let Some(near_stop_ids) = squares.get(&(x + dx, y + dy)) {
                        for near_id in near_stop_ids {
                            let near_coord = utm_coords.get(near_id).unwrap();
                            let distance = (coord.x() - near_coord.x()).abs()
                                + (coord.y() - near_coord.y()).abs();
                            if (distance <= max_conn_dist) {
                                if let Some(connection) = connections.get_mut(stop_id) {
                                    connection.push((String::from(near_id), distance));
                                } else {
                                    connections.insert(
                                        String::from(stop_id),
                                        vec![(String::from(near_id), distance)],
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return connections;
}

/// Creates a node, adds it to the node vector, returns the id
fn create_node(nodes: &mut Vec<Node>, stop: Option<Rc<Stop>>, trip: Option<Rc<Trip>>, time: &u32) -> usize {
    let node = Node::new(stop, trip, &nodes.len(), time);
    nodes.push(node);
    return node.node_id.clone();
}

pub fn load_transport_network(path: &Path) -> Network {
    let mut stops = load_stops(path);
    let routes = load_routes(path);
    let mut services = load_services(path);
    load_service_exceptions(path, &mut services);
    let mut trips = load_trips(path);
    load_stop_times(path, &mut trips);
    let mut nodes: Vec<Node> = Vec::new();
    let mut arrival_nodes: Vec<usize> = Vec::new();
    for trip in trips.values() {
        let mut prev_transport = None;
        for i in 0..trip.stop_times.len() {
            // TODO add stop and trip correctly
            let mut transport = create_node(&mut nodes, None, None, &stop_times[i].departure_time);
            // add edge from previous transport node
            match prev_transport {
                Some(id) => nodes[prev_transport].add_edge(&transport),
                None => (),
            }
            let mut dep = create_node(&mut nodes, None, None, &stop_times[i].departure_time);
            stops.get(&stop_times.stop_id).unwrap().add_dep_node(&dep);
            let mut arr = create_node(&mut nodes,None, None, &stop_times[i].arrival_time + MINIMAL_TRANSFER_TIME);
            arrival_nodes.push(arr);
            nodes[transport].add_edge(&arr);
            nodes[dep].add_edge(&transport);
            prev_transport = Some(transport);
        }
    }

    println!("Finalizing stops...");
    for stop in stops.values_mut() {
        stop.finalize(&mut nodes);
    }

    println!("Calculating pedestrian connections...");
    let utm_coords = get_stop_coords_in_utm(&stops);
    let squares = calculate_proximity_squares(&utm_coords, MAX_PEDESTRIAN_DIST);
    let connections = get_pedestrian_connections(&utm_coords, &squares, MAX_PEDESTRIAN_DIST);

    println!("Adding edges between arrival and departure nodes...");
    for arr_node in arrival_nodes {
        // TODO add edges from arrival nodes to departure node chain
    }

    return Network::new(stops, routes, trips, services, nodes);
}
