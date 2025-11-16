// Demonstration of the complete flight path visualization with progress info

use flui::{FlightStatus, FlightStatusViewModel};
use chrono::{Utc, Duration};

fn main() {
    println!("\n=== Complete Flight Path Visualization ===\n");
    
    // Create a flight that arrives in 1.5 hours
    let arrival_time = Utc::now() + Duration::hours(1) + Duration::minutes(30);
    
    let flight = FlightStatusViewModel {
        flight_number: "AA100".to_string(),
        status: FlightStatus::EnRoute,
        scheduled_departure: Some("2025-11-18T09:00:00Z".to_string()),
        scheduled_arrival: Some(arrival_time.to_rfc3339()),
        estimated_departure: Some("2025-11-18T09:00:00Z".to_string()),
        estimated_arrival: Some(arrival_time.to_rfc3339()),
        actual_departure: Some("2025-11-18T09:05:00Z".to_string()),
        actual_arrival: None,
        progress_percent: Some(55),
        origin_airport: Some("NRT".to_string()),
        destination_airport: Some("HND".to_string()),
    };
    
    println!("┌────────────────── Flight Progress ──────────────────┐");
    render_flight_path(&flight, 60);
    println!("└─────────────────────────────────────────────────────┘");
}

fn render_flight_path(vm: &FlightStatusViewModel, width: usize) {
    let progress = vm.progress_percentage();
    let origin = vm.origin_airport.as_deref().unwrap_or("???");
    let destination = vm.destination_airport.as_deref().unwrap_or("???");
    
    // Line 1: Airport codes
    println!("│ {:<width$}{:>width$} │", origin, destination, width = width / 2);
    
    // Line 2: Progress info
    let time_remaining = vm.time_remaining().unwrap_or_else(|| "N/A".to_string());
    let info_text = format!("{:.0}% • {}", progress, time_remaining);
    let padding = (width.saturating_sub(info_text.len())) / 2;
    println!("│ {:padding$}{} │", "", info_text, padding = padding);
    
    // Line 3: Flight path
    let path_width = width.saturating_sub(2);
    let airplane_pos = ((path_width as f64 * progress / 100.0).round() as usize).min(path_width.saturating_sub(1));
    
    print!("│ ●");
    for i in 0..path_width {
        if i == airplane_pos {
            print!("✈");
        } else if i < airplane_pos {
            print!("─");
        } else {
            print!("─");
        }
    }
    println!("● │");
}
