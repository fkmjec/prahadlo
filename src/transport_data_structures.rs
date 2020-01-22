pub mod transport_data_structures {
    use std::collections::HashMap;

    struct Route {
        agency_id: String,
        route_short_name: String,
        route_long_name: String,
        route_type: i32, // TODO replace with an enum
    }

    struct Trip {
        route_id: String,
        service_id: String,
    }

    struct Node {
        id: String,
        name: String,
        lat: f32,
        lon: f32,
        location_type: i32,
        timezone: String,
        edges: Vec<Edge>,
    }

    struct Edge {
        leaves_at: i32, // probably since the start of the week, but I need to make this straight in the next week
        duration: i32,
        trip_id: Option<String>,
        mean: Option<i32>, // TODO replace with an enum
        target_node: String,
    }

    struct Graph {
        routes: HashMap<String, Route>,
        trips: HashMap<String, Trip>,
        nodes: HashMap<String, Node>,
    }
} 
