// Demonstration of formatted arrival times

use flui::{FlightStatus, FlightStatusViewModel};

fn main() {
    // Example with UTC timestamp
    let flight = FlightStatusViewModel {
        flight_number: "AA100".to_string(),
        status: FlightStatus::EnRoute,
        scheduled_departure: Some("2025-11-18T09:40:00Z".to_string()),
        scheduled_arrival: Some("2025-11-18T18:30:00Z".to_string()),
        estimated_departure: Some("2025-11-18T09:40:00Z".to_string()),
        estimated_arrival: Some("2025-11-18T18:30:00Z".to_string()),
        actual_departure: Some("2025-11-18T09:42:00Z".to_string()),
        actual_arrival: None,
        progress_percent: Some(45),
    };

    println!("Flight: {}", flight.flight_number);
    println!("Status: {}", flight.status);
    println!();
    println!("Raw arrival time:       {:?}", flight.arrival_time());
    println!("Formatted arrival time: {:?}", flight.formatted_arrival_time());
    println!();
    println!("The formatted time is converted to your system's local timezone");
    println!("and displayed in a human-readable format!");
}
