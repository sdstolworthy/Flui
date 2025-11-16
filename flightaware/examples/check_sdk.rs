use flightaware::Client;

fn main() {
    // Create a client (not actually connecting, just showing it compiles)
    let client = Client::new("https://aeroapi.flightaware.com/aeroapi");
    println!("FlightAware SDK client created successfully!");
    println!("Client type: {:?}", std::any::type_name_of_val(&client));
}
