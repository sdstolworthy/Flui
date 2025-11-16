// Visual demonstration of what the alert looks like in the terminal

use flui::{FlightStatus, FlightStatusViewModel};
use chrono::{Utc, Duration};

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║           LANDING ALERT VISUAL DEMONSTRATION                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
    
    println!("When your flight is approaching landing, the TUI will:");
    println!();
    
    // Test with a flight 15 minutes out
    let arrival_time = Utc::now() + Duration::minutes(15);
    let flight = FlightStatusViewModel {
        flight_number: "AA100".to_string(),
        status: FlightStatus::EnRoute,
        scheduled_departure: Some("2025-11-18T09:00:00Z".to_string()),
        scheduled_arrival: Some(arrival_time.to_rfc3339()),
        estimated_departure: Some("2025-11-18T09:00:00Z".to_string()),
        estimated_arrival: Some(arrival_time.to_rfc3339()),
        actual_departure: Some("2025-11-18T09:05:00Z".to_string()),
        actual_arrival: None,
        progress_percent: Some(90),
        origin_airport: Some("SFO".to_string()),
        destination_airport: Some("LAX".to_string()),
    };
    
    println!("  Flight: {} from {} to {}", 
        flight.flight_number, 
        flight.origin_airport.as_ref().unwrap(),
        flight.destination_airport.as_ref().unwrap()
    );
    println!("  Time remaining: {}", flight.time_remaining().unwrap());
    println!("  Progress: {:.0}%", flight.progress_percentage());
    println!();
    
    println!("ALERT FEATURES ACTIVATED:");
    println!();
    println!("  1. 🔔 TERMINAL BELL");
    println!("     - Rings once when alert threshold is first crossed");
    println!("     - Audible notification to catch your attention");
    println!();
    println!("  2. ⚠️  RED BLINKING BORDERS");
    println!("     - All UI panels have red, rapidly blinking borders");
    println!("     - VERY noticeable in peripheral vision");
    println!();
    println!("  3. 🚨 'LANDING SOON' WARNING");
    println!("     - Flight Information title changes to:");
    println!("       \"Flight: AA100 ⚠️  LANDING SOON ⚠️\"");
    println!("     - Flight Progress title changes to:");
    println!("       \"⚠️  Flight Progress - LANDING SOON  ⚠️\"");
    println!();
    println!("  4. 🎨 RED HIGHLIGHTED TEXT");
    println!("     - Flight number changes from cyan to red");
    println!("     - Bold formatting for extra visibility");
    println!();
    println!("  5. 🔄 CONTINUOUS UPDATES");
    println!("     - Alerts continue until plane lands");
    println!("     - Time remaining updates every refresh cycle");
    println!();
    
    println!("CONFIGURATION:");
    println!();
    println!("  Default threshold: 30 minutes before landing");
    println!("  Customize with: --alert-threshold-minutes <N>");
    println!();
    println!("  Examples:");
    println!("    --alert-threshold-minutes 15   (alert 15 min before)");
    println!("    --alert-threshold-minutes 45   (alert 45 min before)");
    println!("    --alert-threshold-minutes 10   (alert 10 min before)");
    println!();
    
    println!("PERFECT FOR:");
    println!();
    println!("  ✈️  Working while waiting at the airport");
    println!("  💻 Deep focus programming sessions");
    println!("  🎧 Wearing headphones (visual alerts)");
    println!("  👀 Keeping flight status in peripheral vision");
    println!();
    
    println!("═══════════════════════════════════════════════════════════════");
    println!("The alert is designed to be IMPOSSIBLE to miss while coding!");
    println!("═══════════════════════════════════════════════════════════════\n");
}
