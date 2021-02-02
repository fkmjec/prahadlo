mod model;

use std::path::Path;

use crate::model::data_structures::Node;
use crate::model::loader;

fn main() {
    println!("Hello, world! Prahadlo here!");
    let graph = loader::load_transport_network(Path::new("data/"));
    println!("{}", graph.find_connection("U675S1", "U115S1", 1100).unwrap().unwrap());
}
