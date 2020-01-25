extern crate chrono;

use std::collections::HashMap;
use chrono::{Date, FixedOffset};

pub struct Route {
    agency_id: String,
    route_short_name: String,
    route_long_name: String,
    route_type: i32, // TODO replace with an enum
}

pub struct Trip {
    route_id: String,
    service_id: String,
}

pub struct Service {
    service_days: u8,
    start_date: Date<FixedOffset>,
    end_date: Date<FixedOffset>,
}

pub struct Node {
    id: String,
    name: String,
    lat: f32,
    lon: f32,
    location_type: i32,
    timezone: String,
    edges: Vec<Edge>,
}

pub struct Edge {
    leaves_at: i32, // probably since the start of the week, but I need to make this straight in the next week
    duration: i32,
    trip_id: Option<String>,
    mean: Option<i32>, // TODO replace with an enum
    target_node: String,
}

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
