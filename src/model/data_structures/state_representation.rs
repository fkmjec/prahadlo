use crate::model::data_structures::primitive_gtfs::*;
use core::cmp::Ordering;
use serde::Deserialize;
use std::collections::{BinaryHeap, HashMap};

// TODO should contain the data structures used for actual searching in the graph and the mechanisms to construct them

pub static MINIMAL_TRANSFER_TIME: u32 = 0;


// TODO move stop to primitive_gtfs, it logically doesn't really belong here
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
        // sort nodes by departure times
        self.departure_nodes
            .sort_by(|a, b| nodes[*a].get_time().cmp(&nodes[*b].get_time()));
        // add edges between them
        if self.dep_node_count() >= 2 {
            for index in 0..self.dep_node_count() - 2 {
                let dep = self.get_dep_node(index);
                nodes[dep].add_edge(dep);
            }
        }
        self.finalized = true;
    }

    pub fn get_earliest_dep(
        &self,
        time: u32,
        nodes: &Vec<Node>,
    ) -> Result<Option<usize>, &str> {
        if self.finalized {
            let mut l: i32 = 0;
            let mut r = self.dep_node_count() as i32 - 1;
            let mut best = None;
            while l <= r {
                let middle = (l + r) / 2;
                let addr = self.get_dep_node(middle as usize);
                if nodes[addr].get_time() >= time {
                    best = Some(addr);
                    r = middle - 1;
                }
                if nodes[self.get_dep_node(middle as usize)].get_time() < time {
                    l = middle + 1;
                }
            }
            Ok(best)
        } else {
            Err("Trying to get earliest next departure on a Stop that is not finalized.")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    // TODO Does node need to know its ID?
    pub node_id: usize,
    time: u32,
    edges: Vec<usize>, // neighbour list, neighbours represented by ID
}

impl Node {
    pub fn new(node_id: usize, time: u32) -> Node {
        Node {
            node_id: node_id,
            time: time,
            edges: Vec::new(),
        }
    }

    pub fn get_time(&self) -> u32 {
        self.time
    }

    pub fn get_edges(&self) -> &Vec<usize> {
        &self.edges
    }

    pub fn add_edge(&mut self, node_id: usize) {
        &self.edges.push(node_id);
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.node_id == other.node_id
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.get_time().cmp(&self.get_time())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
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
    /*
    pub fn find_connection(
        &self,
        dep_stop_id: &str,
        target_stop_id: &str,
        time: u32,
    ) -> Result<Option<u32>, &str> {
        let mut dists = vec![-1; self.nodes.len()];
        let mut came_from: Vec<i32> = vec![-1; self.nodes.len()];
        let mut heap = BinaryHeap::new();
        let start = self
            .stops
            .get(dep_stop_id)
            .ok_or("Stop not found.")?
            .get_earliest_dep(time, &self.nodes)?
            .ok_or("There is no departure from the stop after the selected time")?;
        dists[start] = time as i32;
        heap.push(start);

        while let Some(popped) = heap.pop() {
            println!("POPPED! {:?}", self.nodes[popped]);
            let node_struct = &self.nodes[popped];
            if node_struct.stop_id.as_str() == target_stop_id {
                let mut index = popped;
                while came_from[index] != -1 {
                    println!("{:?}", self.nodes[index]);
                    index = came_from[index] as usize;
                }
                return Ok(Some(node_struct.get_time() - time));
            }
            for edge in node_struct.get_edges() {
                if dists[edge.target_node] == -1 || ((node_struct.get_time() + edge.cost()) as i32) < dists[edge.target_node] {
                    heap.push(edge.target_node); // TODO solve this inefficient bullcrap
                    dists[edge.target_node] = (node_struct.get_time() + edge.cost()) as i32;
                    came_from[edge.target_node] = popped as i32;
                }
            }
            dists[popped] = node_struct.get_time() as i32;
        }
        return Ok(None);
    }
    */
}
