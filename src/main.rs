mod model;

use std::path::Path;

use crate::model::data_structures::Node;
use crate::model::loader;

fn main() {
    println!("Hello, world! Prahadlo here!");
    let graph = loader::load_transport_network(Path::new("data/"));
    for i in 0..12 {
        println!(
            "{}",
            graph
                .find_connection("U675Z1P", "U115Z1P", i * 3600)
                .unwrap()
                .unwrap(),
        );
    }
}
