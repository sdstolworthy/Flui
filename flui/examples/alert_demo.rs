// Demonstration of landing alert feature

use flui::{FlightStatus, FlightStatusViewModel};
use chrono::{Utc, Duration};

fn main() {
    println!("\n=== Landing Alert Feature Demo ===\n");
    
    // Test 1: Flight more than 30 minutes away (no alert)
    let arrival_time = Utc::now() + Duration::minutes(45);
    let flight = FlightStatusViewModel {
        flight_number: "AA100".to_string(),
        status: FlightStatus::EnRoute,
        scheduled_departure: Some("2025-11-18T09:00:00Z".to_string()),
        scheduled_arrival: Some(arrival_time.to_rfc3339()),
        estimated_departure: Some("2025-11-18T09:00:00Z".to_string()),
        estimated_arrival: Some(arrival_time.to_rfc3339()),
        actual_departure: Some("2025-11-18T09:05:00Z".to_string()),
        actual_arrival: None,
        progress_percent: Some(60),
        origin_airport: Some("NRT".to_string()),
        destination_airport: Some("HND".to_string()),
    };
    
    println!("Test 1: Flight arriving in 45 minutes");
    println!("  Alert threshold: 30 minutes");
    println!("  Is approaching landing? {}", flight.is_approaching_landing(30));
    println!("  Expected: false (too far away)");
    println!();
    
    // Test 2: Flight within 30 minutes (alert!)
    let arrival_time = Utc::now() + Duration::minutes(20);
    let flight2 = FlightStatusViewModel {
        flight_number: "UA200".to_string(),
        status: FlightStatus::EnRoute,
        scheduled_departure: Some("2025-11-18T09:00:00Z".to_string()),
        scheduled_arrival: Some(arrival_time.to_rfc3339()),
        estimated_departure: Some("2025-11-18T09:00:00Z".to_string()),
        estimated_arrival: Some(arrival_time.to_rfc3339()),
        actual_departure: Some("2025-11-18T09:05:00Z".to_string()),
        actual_arrival: None,
        progress_percent: Some(85),
        origin_airport: Some("SFO".to_string()),
        destination_airport: Some("LAX".to_string()),
    };
    
    println!("Test 2: Flight arriving in 20 minutes");
    println!("  Alert threshold: 30 minutes");
    println!("  Is approaching landing? {}", flight2.is_approaching_landing(30));
    println!("  Expected: true (within threshold)");
    println!("  Time remaining: {}", flight2.time_remaining().unwrap_or("N/A".to_string()));
    println!();
    
    // Test 3: Already arrived (no alert)
    let flight3 = FlightStatusViewModel {
        flight_number: "DL300".to_string(),
        status: FlightStatus::OnTime,
        scheduled_departure: Some("2025-11-18T09:00:00Z".to_string()),
        scheduled_arrival: Some("2025-11-18T15:00:00Z".to_string()),
        estimated_departure: Some("2025-11-18T09:00:00Z".to_string()),
        estimated_arrival: Some("2025-11-18T15:00:00Z".to_string()),
        actual_departure: Some("2025-11-18T09:05:00Z".to_string()),
        actual_arrival: Some("2025-11-18T15:10:00Z".to_string()),
        progress_percent: Some(100),
        origin_airport: Some("JFK".to_string()),
        destination_airport: Some("ORD".to_string()),
    };
    
    println!("Test 3: Flight already arrived");
    println!("  Alert threshold: 30 minutes");
    println!("  Is approaching landing? {}", flight3.is_approaching_landing(30));
    println!("  Expected: false (already landed)");
    println!("  Time remaining: {}", flight3.time_remaining().unwrap_or("N/A".to_string()));
    println!();
    
    println!("Alert Features:");
    println!("  ⚠️  Red blinking borders");
    println!("  ⚠️  'LANDING SOON' warning in title");
    println!("  🔔 Terminal bell on first alert");
    println!("  ✈️  Flight info highlighted in red");
}
