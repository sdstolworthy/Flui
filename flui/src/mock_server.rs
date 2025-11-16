#[cfg(feature = "httpmock")]
use httpmock::prelude::*;

#[cfg(feature = "httpmock")]
pub fn start_mock_server() -> (String, httpmock::MockServer) {
    let server = httpmock::MockServer::start();

    // Load the sample flight data
    const SAMPLE_DATA: &str = include_str!("../../flightaware/sample_flight_aware.json");

    // Set up a wildcard mock for any flight query
    server.mock(|when, then| {
        when.method(GET).path_matches(r"^/flights/.*");
        then.status(200)
            .header("content-type", "application/json")
            .body(SAMPLE_DATA);
    });

    let base_url = server.base_url();
    println!("🚀 Mock FlightAware server started at: {}", base_url);
    println!("   All flight queries will return sample data");

    (base_url, server)
}
