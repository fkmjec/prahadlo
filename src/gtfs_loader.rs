extern crate csv;

use crate::transport_data_structures::*;
use std::path::Path;
use std::collections::HashMap;

enum StopsFields {
    StopId,
    StopName,
    StopLat,
    StopLon,
    ZoneId,
    StopUrl,
    LocationType,
    ParentStation,
    WheelchairBoarding,
    LevelId,
    PlatformCode,
}

enum RoutesFields {
    RouteId,
    AgencyId,
    RouteShortName,
    RouteLongName,
    RouteType,
    RouteUrl,
    RouteColor,
    RouteTextColor,
    IsNight,
}

enum TripsFields {
    RouteId,
    ServiceId,
    TripId,
    TripHeadsign,
    TripShortName,
    DirectionId,
    BlockId,
    ShapeId,
    WheelchairAccessible,
    BikesAllowed,
    Exceptional,
    TripOperationType,
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
fn load_stops(path: &Path) -> HashMap<String, Node> {
    // TODO load stops
    HashMap::new()
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
