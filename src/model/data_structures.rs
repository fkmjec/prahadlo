use chrono::NaiveDate;
use serde::{de, de::Unexpected, Deserialize, Deserializer};
use std::collections::HashMap;

pub static MINIMAL_TRANSFER_TIME: u32 = 60;

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
pub struct Stop {
    pub stop_id: String,
    pub stop_name: String,
    pub stop_lat: f32,
    pub stop_lon: f32,
    pub zone_id: String,
    pub stop_url: Option<String>,
    pub location_type: u8,
    pub parent_station: Option<String>,
    pub wheelchair_boarding: Option<i32>,
    pub level_id: Option<String>,
    pub platform_code: Option<String>,
    #[serde(default = "Vec::new", skip_deserializing)]
    departure_nodes: Vec<usize>,
    #[serde(default = "Stop::default_state", skip_deserializing)]
    finalized: bool,
}

impl Stop {
    pub fn default_state() -> bool {
        false
    }

    pub fn get_dep_node(&self, index: usize) -> usize {
        self.departure_nodes[index]
    }

    pub fn add_dep_node(&mut self, dep_node: usize) -> Result<(), &str> {
        if !self.finalized {
            &self.departure_nodes.push(dep_node);
            Ok(())
        } else {
            Err("Tried to add a new departure node to an already finalized Stop.")
        }
    }

    pub fn dep_node_count(&self) -> usize {
        self.departure_nodes.len()
    }

    /// Adds the departure transfer chain, locks the departure nodes
    pub fn finalize(&mut self, nodes: &mut Vec<Node>) {
        self.departure_nodes
            .sort_by(|a, b| nodes[*a].get_time().cmp(&nodes[*b].get_time()));
        if self.dep_node_count() >= 2 {
            for index in 0..self.dep_node_count() - 2 {
                let dep = self.get_dep_node(index);
                let dep_time = nodes[dep].get_time();
                let arr = self.get_dep_node(index + 1);
                let arr_time = nodes[arr].get_time();
                nodes[dep].add_edge(Edge::new(dep_time, arr_time, None, arr));
            }
        }
        self.finalized = true;
    }

    pub fn get_earliest_dep(
        &self,
        arr_time: u32,
        nodes: &Vec<Node>,
    ) -> Result<Option<usize>, &str> {
        if self.finalized {
            let mut l: i32 = 0;
            let mut r = self.dep_node_count() as i32 - 1;
            let mut best = None;
            while l <= r {
                let middle = (l + r) / 2;
                let addr = self.get_dep_node(middle as usize);
                if nodes[addr].get_time() >= arr_time {
                    best = Some(addr);
                    r = middle - 1;
                }
                if nodes[self.get_dep_node(middle as usize)].get_time() < arr_time {
                    l = middle + 1;
                }
            }
            Ok(best)
        } else {
            Err("Trying to get earliest next departure on a Stop that is not finalized.")
        }
    }
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

#[derive(Debug, Clone)]
pub enum NodeKind {
    Arr,
    Dep,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub stop_id: String,
    node_kind: NodeKind,
    time: u32,
    edges: Vec<Edge>,
}

impl Node {
    pub fn new(stop_id: String, node_kind: NodeKind, time: u32) -> Node {
        Node {
            stop_id: stop_id,
            node_kind: node_kind,
            time: time,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        &self.edges.push(edge);
    }

    pub fn get_time(&self) -> u32 {
        self.time
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    departs_at: u32,
    arrives_at: u32,
    trip_id: Option<String>,
    target_node: usize,
}

impl Edge {
    pub fn new(
        departs_at: u32,
        arrives_at: u32,
        trip_id: Option<String>,
        target_node: usize,
    ) -> Edge {
        Edge {
            departs_at: departs_at,
            arrives_at: arrives_at,
            trip_id: trip_id,
            target_node: target_node,
        }
    }

    /// returns the cost of the edge in seconds
    pub fn cost(&self) -> u32 {
        return &self.arrives_at - &self.departs_at;
    }
}

#[derive(Debug)]
pub struct Network {
    stops: HashMap<String, Stop>,
    routes: HashMap<String, Route>,
    trips: HashMap<String, Trip>,
    services: HashMap<String, Service>,
    nodes: Vec<Node>,
}

impl Network {
    pub fn new(
        stops: HashMap<String, Stop>,
        routes: HashMap<String, Route>,
        trips: HashMap<String, Trip>,
        services: HashMap<String, Service>,
        nodes: Vec<Node>,
    ) -> Network {
        Network {
            stops: stops,
            routes: routes,
            trips: trips,
            services: services,
            nodes: nodes,
        }
    }
}
