mod model;

use std::path::Path;
use std::io;

use crate::model::loader;

fn main() {
    println!("Hello, world! Prahadlo here!");
    let graph = loader::load_transport_network(Path::new("data/"));
    /*
    loop {
        let mut input = String::new();
        println!("Please provide the two stop IDs separated by space!");
        io::stdin().read_line(&mut input).expect("Unable to read input!");
        let split: Vec<&str> = input.split(" ").collect();

        println!("{}", graph.find_connection(split[0], split[1], 1100).unwrap().unwrap());
    }
    */
}
