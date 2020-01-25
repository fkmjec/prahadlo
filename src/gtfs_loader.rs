use crate::transport_data_structures::*;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use chrono::NaiveDate;

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
pub fn load_routes(path: &Path) -> HashMap<String, Route> {
    let mut routes = HashMap::new();
    let mut file_path_buf = path.to_path_buf();
    file_path_buf.push(Path::new("routes.txt"));
    let file = File::open(file_path_buf.as_path()).unwrap(); // No need for error handling, if this fails, we want to panic
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result.unwrap();
        let route_id = String::from(record.get(RouteFields::Id as usize).unwrap());
        let agency_id = String::from(record.get(RouteFields::AgencyId as usize).unwrap());
        let short_name = String::from(record.get(RouteFields::ShortName as usize).unwrap());
        let long_name = String::from(record.get(RouteFields::LongName as usize).unwrap());
        let route_type: i32 = record.get(RouteFields::Type as usize).unwrap().parse().unwrap();
        let route = Route::new(agency_id, short_name, long_name, route_type);
        routes.insert(route_id, route);
    }
    return routes;
}

/// Loads the contents of trips.txt
/// # Arguments
/// * path - the path to the directory trips.txt is located in
pub fn load_trips(path: &Path) -> HashMap<String, Trip> {
    let mut trips = HashMap::new();
    let mut file_path_buf = path.to_path_buf();
    file_path_buf.push(Path::new("trips.txt"));
    let file = File::open(file_path_buf.as_path()).unwrap(); // No need for error handling, if this fails, we want to panic
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result.unwrap();
        let trip_id = String::from(record.get(TripFields::Id as usize).unwrap());
        let route_id = String::from(record.get(TripFields::RouteId as usize).unwrap());
        let service_id = String::from(record.get(TripFields::ServiceId as usize).unwrap());
        let trip = Trip::new(route_id, service_id);
        trips.insert(trip_id, trip);
    }
    return trips;
}

/// Parses a string in YYYYMMDD format into NaiveDate
/// # Arguments
/// * raw_ymd - YYYYMMDD
fn parse_ymd(raw_ymd: &str) -> NaiveDate {
    NaiveDate::from_ymd(raw_ymd[0..4].parse::<i32>().unwrap(), raw_ymd[4..6].parse::<u32>().unwrap(), raw_ymd[6..].parse::<u32>().unwrap())
}

/// Loads the contents of services.txt and service_dates.txt
/// # Arguments
/// * path - the path to the directory the files are located in
pub fn load_services(path: &Path) -> HashMap<String, Service> {
    let mut services = HashMap::new();
    let mut file_path_buf = path.to_path_buf();
    file_path_buf.push(Path::new("calendar.txt"));
    let file = File::open(file_path_buf.as_path()).unwrap(); // No need for error handling, if this fails, we want to panic
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result.unwrap();
        let service_id = String::from(record.get(CalendarFields::ServiceId as usize).unwrap());
        let monday = record.get(CalendarFields::Monday as usize).unwrap().parse::<i32>().unwrap() != 0;
        let tuesday = record.get(CalendarFields::Tuesday as usize).unwrap().parse::<i32>().unwrap() != 0;
        let wednesday = record.get(CalendarFields::Wednesday as usize).unwrap().parse::<i32>().unwrap() != 0;
        let thursday = record.get(CalendarFields::Thursday as usize).unwrap().parse::<i32>().unwrap() != 0;
        let friday = record.get(CalendarFields::Friday as usize).unwrap().parse::<i32>().unwrap() != 0;
        let saturday = record.get(CalendarFields::Saturday as usize).unwrap().parse::<i32>().unwrap() != 0;
        let sunday = record.get(CalendarFields::Sunday as usize).unwrap().parse::<i32>().unwrap() != 0;
        let start_date = parse_ymd(record.get(CalendarFields::StartDate as usize).unwrap());
        let end_date = parse_ymd(record.get(CalendarFields::StartDate as usize).unwrap());
        let service = Service {
            monday: monday,
            tuesday: tuesday,
            wednesday: wednesday,
            thursday: thursday,
            friday: friday,
            saturday: saturday,
            sunday: sunday,
            start_date: start_date,
            end_date: end_date,
        };
        services.insert(service_id, service);
    }
    return services;
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
