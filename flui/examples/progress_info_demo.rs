// Demonstration of progress info with time remaining

use flui::{FlightStatus, FlightStatusViewModel};
use chrono::{Utc, Duration};

fn main() {
    println!("\n=== Flight Progress Info Demo ===\n");
    
    // Create a flight that arrives in 2.5 hours
    let arrival_time = Utc::now() + Duration::hours(2) + Duration::minutes(30);
    
    let flight = FlightStatusViewModel {
        flight_number: "AA100".to_string(),
        status: FlightStatus::EnRoute,
        scheduled_departure: Some("2025-11-18T09:00:00Z".to_string()),
        scheduled_arrival: Some(arrival_time.to_rfc3339()),
        estimated_departure: Some("2025-11-18T09:00:00Z".to_string()),
        estimated_arrival: Some(arrival_time.to_rfc3339()),
        actual_departure: Some("2025-11-18T09:05:00Z".to_string()),
        actual_arrival: None,
        progress_percent: Some(45),
        origin_airport: Some("NRT".to_string()),
        destination_airport: Some("HND".to_string()),
    };
    
    println!("Flight: {}", flight.flight_number);
    println!("Progress: {:.0}%", flight.progress_percentage());
    println!("Time Remaining: {}", flight.time_remaining().unwrap_or("N/A".to_string()));
    println!();
    
    // Simulate the progress info display
    let width: usize = 60;
    let info_text = format!("{:.0}% • {}", 
        flight.progress_percentage(), 
        flight.time_remaining().unwrap_or("N/A".to_string())
    );
    let padding = (width.saturating_sub(info_text.len())) / 2;
    println!("{:padding$}{}", "", info_text, padding = padding);
    
    // Test with arrived flight
    println!("\n--- Arrived Flight ---");
    let arrived_flight = FlightStatusViewModel {
        flight_number: "AA200".to_string(),
        status: FlightStatus::OnTime,
        scheduled_departure: Some("2025-11-18T09:00:00Z".to_string()),
        scheduled_arrival: Some("2025-11-18T15:00:00Z".to_string()),
        estimated_departure: Some("2025-11-18T09:00:00Z".to_string()),
        estimated_arrival: Some("2025-11-18T15:00:00Z".to_string()),
        actual_departure: Some("2025-11-18T09:05:00Z".to_string()),
        actual_arrival: Some("2025-11-18T15:10:00Z".to_string()),
        progress_percent: Some(100),
        origin_airport: Some("SFO".to_string()),
        destination_airport: Some("LAX".to_string()),
    };
    
    println!("Progress: {:.0}%", arrived_flight.progress_percentage());
    println!("Time Remaining: {}", arrived_flight.time_remaining().unwrap_or("N/A".to_string()));
}
