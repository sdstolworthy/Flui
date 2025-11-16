// Demonstration of the FlightStatusViewModel
// This shows how the view model will be used to display flight information

use flui::{FlightStatus, FlightStatusViewModel};

fn main() {
    // Example 1: Flight that departed on time and is en route
    let flight1 = FlightStatusViewModel {
        flight_number: "AA100".to_string(),
        status: FlightStatus::EnRoute,
        scheduled_departure: Some("2025-11-16T10:00:00Z".to_string()),
        scheduled_arrival: Some("2025-11-16T14:00:00Z".to_string()),
        estimated_departure: Some("2025-11-16T10:00:00Z".to_string()),
        estimated_arrival: Some("2025-11-16T14:05:00Z".to_string()),
        actual_departure: Some("2025-11-16T10:02:00Z".to_string()),
        actual_arrival: None,
        progress_percent: Some(45),
    };

    println!("Flight: {}", flight1.flight_number);
    println!("Status: {}", flight1.status);
    println!("Scheduled Departure: {:?}", flight1.scheduled_departure);
    println!("Departure Time: {:?}", flight1.departure_time());
    println!("Scheduled Arrival: {:?}", flight1.scheduled_arrival);
    println!("Arrival Time: {:?}", flight1.arrival_time());
    println!("Progress: {}%", flight1.progress_percentage());
    println!();

    // Example 2: Delayed flight
    let flight2 = FlightStatusViewModel {
        flight_number: "DL456".to_string(),
        status: FlightStatus::Delayed,
        scheduled_departure: Some("2025-11-16T12:00:00Z".to_string()),
        scheduled_arrival: Some("2025-11-16T15:30:00Z".to_string()),
        estimated_departure: Some("2025-11-16T13:15:00Z".to_string()),
        estimated_arrival: Some("2025-11-16T16:45:00Z".to_string()),
        actual_departure: None,
        actual_arrival: None,
        progress_percent: Some(0),
    };

    println!("Flight: {}", flight2.flight_number);
    println!("Status: {}", flight2.status);
    println!("Scheduled Departure: {:?}", flight2.scheduled_departure);
    println!("Estimated Departure: {:?}", flight2.estimated_departure);
    println!("Departure Time: {:?}", flight2.departure_time());
    println!("Progress: {}%", flight2.progress_percentage());
    println!();

    // Example 3: Cancelled flight
    let flight3 = FlightStatusViewModel {
        flight_number: "UA789".to_string(),
        status: FlightStatus::Cancelled,
        scheduled_departure: Some("2025-11-16T16:00:00Z".to_string()),
        scheduled_arrival: Some("2025-11-16T20:00:00Z".to_string()),
        estimated_departure: None,
        estimated_arrival: None,
        actual_departure: None,
        actual_arrival: None,
        progress_percent: None,
    };

    println!("Flight: {}", flight3.flight_number);
    println!("Status: {}", flight3.status);
    println!("Scheduled Departure: {:?}", flight3.scheduled_departure);
    println!("Departure Time: {:?}", flight3.departure_time());
    println!("Progress: {}%", flight3.progress_percentage());
}
