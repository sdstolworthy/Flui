use httpmock::prelude::*;
use flui::FlightStatusViewModel;

const SAMPLE_FLIGHT_RESPONSE: &str = include_str!("../../flightaware/sample_flight_aware.json");

#[tokio::test]
async fn test_fetch_flight_with_mock_server() {
    let server = MockServer::start();

    // Set up the mock endpoint with actual sample data
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/flights/HAL824");
        then.status(200)
            .header("content-type", "application/json")
            .body(SAMPLE_FLIGHT_RESPONSE);
    });

    // Create a FlightAware client pointing to the mock server
    let client = flightaware::Client::new(&server.base_url());

    // Make the API call
    let result = client.get_flight(
        "HAL824",     // ident (matches sample data)
        None,         // fa_flight_id
        None,         // end
        None,         // ident_type
        None,         // max_pages
        None,         // start
    ).await;

    // Verify the request was made
    mock.assert();

    // Verify we got a successful response
    assert!(result.is_ok(), "API call should succeed: {:?}", result.err());
    
    let response = result.unwrap();
    assert!(!response.flights.is_empty(), "Should have at least one flight");
    
    let flight = &response.flights[0];
    
    // Verify the flight data
    assert_eq!(flight.ident, "HAL824");
    
    // Convert to view model using From trait
    let view_model = FlightStatusViewModel::from(flight);
    
    // Verify the view model was created
    assert_eq!(view_model.flight_number, "HAL824");
    println!("View model created successfully with status: {:?}", view_model.status);
}
