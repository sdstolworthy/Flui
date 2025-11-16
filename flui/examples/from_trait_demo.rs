// Demonstration of using the From trait to convert FlightAware API responses

use flui::FlightStatusViewModel;

fn main() {
    println!("FlightStatusViewModel implements From<&flightaware::types::BaseFlight>");
    println!();
    println!("Usage examples:");
    println!("  let view_model = FlightStatusViewModel::from(&flight);");
    println!("  let view_model: FlightStatusViewModel = (&flight).into();");
    println!();
    println!("This makes conversion idiomatic and type-safe!");
}
