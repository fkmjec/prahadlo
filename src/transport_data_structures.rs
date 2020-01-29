use std::collections::HashMap;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Route {
    agency_id: String,
    short_name: String,
    long_name: String,
    route_type: i32, // TODO replace with an enum
}

impl Route {
    pub fn new(agency_id: String, short_name: String, long_name: String, route_type: i32) -> Route {
        Route {agency_id: agency_id, short_name: short_name, long_name: long_name, route_type: route_type}
    }
}

#[derive(Debug)]
pub struct Trip {
    route_id: String,
    service_id: String,
}

impl Trip {
    pub fn new(route_id: String, service_id: String) -> Trip {
        Trip {route_id: route_id, service_id: service_id}
    }
}

#[derive(Debug)]
pub struct Node {
    name: String,
    lat: f32,
    lon: f32,
    location_type: i32,
    edges: Vec<Edge>,
}

impl Node {
    pub fn new(name: String, lat: f32, lon: f32, location_type: i32) -> Node {
        Node {name: name, lat: lat, lon: lon, location_type: location_type, edges: Vec::new()}
    }
}

#[derive(Debug)]
pub struct Edge {
    leaves_at: i32, // probably since the start of the week, but I need to make this straight
    duration: i32,
    trip_id: Option<String>,
    mean: Option<i32>, // TODO replace with an enum
    target_node: String,
}

#[derive(Debug)]
pub struct Network {
    routes: HashMap<String, Route>,
    trips: HashMap<String, Trip>,
    nodes: HashMap<String, Node>,
}

impl Network {
    pub fn new(routes: HashMap<String, Route>, trips: HashMap<String, Trip>, nodes: HashMap<String, Node>) -> Network {
        Network {routes: routes, trips: trips, nodes: nodes}
    }
}
