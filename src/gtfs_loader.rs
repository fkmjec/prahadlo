use crate::transport_data_structures::*;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

enum StopFields {
    Id,
    Name,
    Lat,
    Lon,
    ZoneId,
    Url,
    LocationType,
    ParentStation,
    WheelchairBoarding,
    LevelId,
    PlatformCode,
}

enum RouteFields {
    Id,
    AgencyId,
    ShortName,
    LongName,
    Type,
    Url,
    Color,
    TextColor,
    IsNight,
}

enum TripFields {
    RouteId,
    ServiceId,
    Id,
    Headsign,
    ShortName,
    DirectionId,
    BlockId,
    ShapeId,
    WheelchairAccessible,
    BikesAllowed,
    Exceptional,
    OperationType,
}

enum StopTimesFields {
    TripId,
    ArrivalTime,
    DepartureTime,
    StopId,
    StopSequence,
    StopHeadsign,
    PickupType,
    DropOffType,
    ShapeDistTraveled,
}

enum CalendarFields {
    ServiceId,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    StartDate,
    EndDate,
}

enum CalendarDatesFields {
    ServiceId,
    Date,
    ExceptionType,
}

/// Loads the contents of stops.txt
/// # Arguments
/// * path - the path to the directory stops.txt is located in
pub fn load_stops(path: &Path) -> HashMap<String, Node> {
    let mut stops = HashMap::new();
    let mut file_path_buf = path.to_path_buf();
    file_path_buf.push(Path::new("stops.txt"));
    let file = File::open(file_path_buf.as_path()).unwrap(); // No need for error handling, if this fails, we want to panic
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result.unwrap();
        let stop_id = String::from(record.get(StopFields::Id as usize).unwrap());
        let name = String::from(record.get(StopFields::Name as usize).unwrap());
        let lat: f32 = record.get(StopFields::Lat as usize).unwrap().parse().unwrap();
        let lon: f32 = record.get(StopFields::Lon as usize).unwrap().parse().unwrap();
        let location_type: i32 = record.get(StopFields::LocationType as usize).unwrap().parse().unwrap();
        let node = Node::new(name, lat, lon, location_type);
        stops.insert(stop_id, node);
    }
    return stops;
}

/// Loads the contents of routes.txt
/// # Arguments
/// * path - the path to the directory routes.txt is located in
fn load_routes(path: &Path) -> HashMap<String, Trip> {
    // TODO load routes
    HashMap::new()
}

/// Loads the contents of trips.txt
/// # Arguments
/// * path - the path to the directory trips.txt is located in
fn load_trips(path: &Path) -> HashMap<String, Route> {
    // TODO load trips
    HashMap::new()
}

/// Loads the contents of services.txt and service_dates.txt
/// # Arguments
/// * path - the path to the directory the files are located in
fn load_services(path: &Path) -> HashMap<String, Service> {
    // TODO load services
    HashMap::new()
}

/// Creates the edges to be used in the inner workings of the search algorithm
/// # Arguments
/// * path - the path to the gtfs directory
/// * stops - loaded contents of stops.txt
/// * routes - loaded contents of routes.txt
/// * trips - loaded contents of trips.txt
/// * services - loaded contents of services.txt and service_dates.txt, specifying the dates
/// the service is available on
fn fill_edges(path: &Path, stops: &mut HashMap<String, Node>, routes: &HashMap<String, Route>, trips: &HashMap<String, Trip>, services: &HashMap<String, Service>) {
    // TODO fill edges
}

pub fn load_transport_network(path: &Path) -> Network {
    // TODO load individual GTFS files 
    Network::new(HashMap::new(), HashMap::new(), HashMap::new())
} 
