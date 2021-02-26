mod model;

use std::path::Path;

use crate::model::loader;

fn main() {
    println!("Hello, world! Prahadlo here!");
    let graph = loader::load_transport_network(Path::new("data/"));
}
