// Demonstration of the flight path visualization

use flui::{FlightStatus, FlightStatusViewModel};

fn main() {
    println!("\n=== Flight Path Visualization Demo ===\n");
    
    // Test different progress levels
    let test_cases = vec![
        (0, "Departure"),
        (25, "En Route (25%)"),
        (50, "Mid-Flight"),
        (75, "En Route (75%)"),
        (100, "Arrived"),
    ];
    
    for (progress, label) in test_cases {
        println!("{}: {}% complete", label, progress);
        
        let flight = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: if progress == 100 { FlightStatus::OnTime } else { FlightStatus::EnRoute },
            scheduled_departure: Some("2025-11-18T09:00:00Z".to_string()),
            scheduled_arrival: Some("2025-11-18T15:00:00Z".to_string()),
            estimated_departure: Some("2025-11-18T09:00:00Z".to_string()),
            estimated_arrival: Some("2025-11-18T15:00:00Z".to_string()),
            actual_departure: Some("2025-11-18T09:05:00Z".to_string()),
            actual_arrival: if progress == 100 { Some("2025-11-18T15:10:00Z".to_string()) } else { None },
            progress_percent: Some(progress),
            origin_airport: Some("NRT".to_string()),
            destination_airport: Some("HND".to_string()),
        };
        
        // Simulate the flight path rendering
        print_flight_path(&flight, 60);
        println!();
    }
}

fn print_flight_path(vm: &FlightStatusViewModel, width: usize) {
    let progress = vm.progress_percentage();
    let origin = vm.origin_airport.as_deref().unwrap_or("???");
    let destination = vm.destination_airport.as_deref().unwrap_or("???");
    
    // Airport codes
    println!("  {:<width$}{:>width$}", origin, destination, width = width / 2);
    
    // Flight path
    let path_width = width.saturating_sub(2);
    let airplane_pos = ((path_width as f64 * progress / 100.0).round() as usize).min(path_width);
    
    print!("  ●");
    for i in 0..path_width {
        if i == airplane_pos {
            print!("✈");
        } else if i < airplane_pos {
            print!("─");
        } else {
            print!("─");
        }
    }
    println!("●");
}
