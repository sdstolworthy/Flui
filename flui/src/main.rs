use clap::Parser;
use flightaware::Client;
use std::fmt;

mod flight_status;
use flight_status::{FlightStatus, FlightStatusViewModel, FlightStatusViewModelBuilder};

mod api_converter;
mod ui;

#[cfg(feature = "httpmock")]
mod mock_server;

#[derive(Debug)]
pub enum ConfigurationError {
    MissingFlightNumber,
    MissingApiKey,
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigurationError::MissingFlightNumber => {
                write!(
                    f,
                    "Flight number is required. Provide via --flight-number flag or FLIGHT_NUMBER environment variable"
                )
            }
            ConfigurationError::MissingApiKey => {
                write!(
                    f,
                    "FlightAware API key is required. Provide via --api-key flag or FLIGHTAWARE_API_KEY environment variable"
                )
            }
        }
    }
}

impl std::error::Error for ConfigurationError {}

#[derive(Parser, Debug)]
#[command(name = "flui")]
#[command(about = "Flight tracker application", long_about = None)]
struct CliArgs {
    #[arg(long, env = "FLIGHT_NUMBER")]
    flight_number: Option<String>,

    #[arg(long, env = "FLIGHTAWARE_API_KEY")]
    api_key: Option<String>,

    #[arg(long, env = "REFRESH_INTERVAL", default_value = "5")]
    refresh_interval: u64,
}

#[derive(Debug)]
pub struct Config {
    pub flight_number: String,
    pub flight_aware_api_key: String,
    pub refresh_interval: u64,
}

impl Config {
    pub fn from_options(
        flight_number: Option<String>,
        api_key: Option<String>,
        refresh_interval: u64,
    ) -> Result<Self, ConfigurationError> {
        let flight_number = flight_number.ok_or(ConfigurationError::MissingFlightNumber)?;
        let flight_aware_api_key = api_key.ok_or(ConfigurationError::MissingApiKey)?;

        Ok(Config {
            flight_number,
            flight_aware_api_key,
            refresh_interval,
        })
    }
}

fn create_flightaware_client(http_client: reqwest::Client, base_url: Option<&str>) -> Client {
    let url = base_url.unwrap_or("https://aeroapi.flightaware.com/aeroapi");
    Client::new_with_client(url, http_client)
}

fn create_authenticated_http_client(api_key: &str) -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "x-apikey",
        reqwest::header::HeaderValue::from_str(api_key).expect("Invalid API key"),
    );

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to build HTTP client")
}

fn get_config() -> Result<Config, ConfigurationError> {
    let args = CliArgs::parse();
    Config::from_options(args.flight_number, args.api_key, args.refresh_interval)
}

/// Select the most relevant flight from a list of flights
/// Returns the flight whose estimated arrival time is closest to (current_time - 2 hours)
fn select_relevant_flight(
    flights: &[flightaware::types::GetFlightResponseFlightsItem],
) -> Option<&flightaware::types::GetFlightResponseFlightsItem> {
    use chrono::{DateTime, Utc};

    if flights.is_empty() {
        return None;
    }

    // Target time is 2 hours ago from now
    let now = Utc::now();
    let target_time = now - chrono::Duration::hours(2);

    // Find the flight with estimated arrival closest to target time
    flights
        .iter()
        .filter_map(|flight| {
            flight.estimated_on.as_ref().map(|arrival| {
                let diff = if arrival > &target_time {
                    *arrival - target_time
                } else {
                    target_time - *arrival
                };
                (diff, flight)
            })
        })
        .min_by_key(|(diff, _)| *diff)
        .map(|(_, flight)| flight)
        .or_else(|| flights.first())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config().unwrap();

    // Start mock server if httpmock feature is enabled
    #[cfg(feature = "httpmock")]
    let (_mock_base_url, _mock_server) = {
        let (base_url, server) = mock_server::start_mock_server();
        (base_url.clone(), server)
    };

    // Use mock server URL if available, otherwise use production
    #[cfg(feature = "httpmock")]
    let base_url = Some(_mock_base_url.as_str());

    #[cfg(not(feature = "httpmock"))]
    let base_url: Option<&str> = None;

    let http_client = create_authenticated_http_client(&config.flight_aware_api_key);
    let client = create_flightaware_client(http_client, base_url);

    // Fetch initial flight data
    let initial_flight_status = client
        .get_flight(&config.flight_number, None, None, None, None, None)
        .await;

    let initial_view_model = match initial_flight_status {
        Ok(response) => {
            if let Some(flight) = select_relevant_flight(&response.flights) {
                FlightStatusViewModel::from(flight)
            } else {
                println!("No flight data found for {}", config.flight_number);
                return Ok(());
            }
        }
        Err(e) => {
            println!("Error fetching flight data: {}", e);
            return Ok(());
        }
    };

    // Create channel for flight updates
    let (tx, mut rx) = tokio::sync::mpsc::channel::<FlightStatusViewModel>(10);

    // Spawn background task to fetch flight updates
    let flight_number = config.flight_number.clone();
    let refresh_interval = config.refresh_interval;
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(refresh_interval));
        interval.tick().await; // Skip first tick (we already have initial data)

        loop {
            for i in 0..11 {
                interval.tick().await;

                let flight_status = client
                    .get_flight(&flight_number, None, None, None, None, None)
                    .await;

                if let Ok(response) = flight_status {
                    if let Some(flight) = select_relevant_flight(&response.flights) {
                        let view_model = FlightStatusViewModel::from(flight);
                        let mut builder = FlightStatusViewModelBuilder::from(view_model);
                        builder.progress_percent(Some((i * 10) as i64)); // simulate progress
                        let view_model = builder.build().unwrap();
                        // change
                        if tx.send(view_model).await.is_err() {
                            // Channel closed, exit task
                            break;
                        }
                    }
                }
            }
        }
    });

    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Current view model
    let mut current_view_model = initial_view_model;

    // Event loop
    use crossterm::event::{self, Event, KeyCode};
    loop {
        // Draw the UI
        terminal.draw(|frame| {
            ui::render_flight_status(frame, &current_view_model);
        })?;

        // Check for updates or user input (with timeout)
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }

        // Check for flight updates (non-blocking)
        if let Ok(updated_view_model) = rx.try_recv() {
            current_view_model = updated_view_model;
        }
    }

    // Restore terminal
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_options_with_both_values() {
        let result = Config::from_options(
            Some("AA100".to_string()),
            Some("test-api-key".to_string()),
            5,
        );

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.flight_number, "AA100");
        assert_eq!(config.flight_aware_api_key, "test-api-key");
        assert_eq!(config.refresh_interval, 5);
    }

    #[test]
    fn test_config_from_options_missing_flight_number() {
        let result = Config::from_options(None, Some("test-api-key".to_string()), 5);

        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigurationError::MissingFlightNumber => (),
            _ => panic!("Expected MissingFlightNumber error"),
        }
    }

    #[test]
    fn test_config_from_options_missing_api_key() {
        let result = Config::from_options(Some("AA100".to_string()), None, 5);

        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigurationError::MissingApiKey => (),
            _ => panic!("Expected MissingApiKey error"),
        }
    }

    #[test]
    fn test_config_from_options_missing_both() {
        let result = Config::from_options(None, None, 5);

        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigurationError::MissingFlightNumber => (),
            _ => panic!("Expected MissingFlightNumber error"),
        }
    }

    #[test]
    fn test_configuration_error_display_flight_number() {
        let error = ConfigurationError::MissingFlightNumber;
        let message = format!("{}", error);
        assert!(message.contains("Flight number is required"));
        assert!(message.contains("--flight-number"));
        assert!(message.contains("FLIGHT_NUMBER"));
    }

    #[test]
    fn test_configuration_error_display_api_key() {
        let error = ConfigurationError::MissingApiKey;
        let message = format!("{}", error);
        assert!(message.contains("FlightAware API key is required"));
        assert!(message.contains("--api-key"));
        assert!(message.contains("FLIGHTAWARE_API_KEY"));
    }

    #[test]
    fn test_select_relevant_flight_empty() {
        let flights = vec![];
        let result = select_relevant_flight(&flights);
        assert!(result.is_none());
    }

    #[test]
    fn test_select_relevant_flight_picks_closest_to_target() {
        use chrono::{TimeZone, Utc};

        // Create test JSON with 3 flights with different arrival times
        // We're testing at time 2025-11-16T13:24:30Z (current time)
        // Target time would be 2025-11-16T11:24:30Z (2 hours ago)
        let json_data = r#"{
            "flights": [
                {
                    "ident": "AA100-OLD",
                    "ident_icao": "AAL100",
                    "ident_iata": "AA100",
                    "fa_flight_id": "AAL100-1234-old",
                    "operator": "AAL",
                    "operator_icao": "AAL",
                    "operator_iata": "AA",
                    "flight_number": "100",
                    "registration": "N12345",
                    "atc_ident": null,
                    "inbound_fa_flight_id": null,
                    "codeshares": [],
                    "codeshares_iata": [],
                    "blocked": false,
                    "diverted": false,
                    "cancelled": false,
                    "position_only": false,
                    "origin": null,
                    "destination": null,
                    "departure_delay": 0,
                    "arrival_delay": 0,
                    "filed_ete": null,
                    "scheduled_out": null,
                    "estimated_out": null,
                    "actual_out": null,
                    "scheduled_off": "2025-11-16T07:00:00Z",
                    "estimated_off": "2025-11-16T07:00:00Z",
                    "actual_off": "2025-11-16T07:05:00Z",
                    "scheduled_on": "2025-11-16T10:00:00Z",
                    "estimated_on": "2025-11-16T10:00:00Z",
                    "actual_on": "2025-11-16T10:10:00Z",
                    "scheduled_in": null,
                    "estimated_in": null,
                    "actual_in": null,
                    "progress_percent": 100,
                    "status": "Landed",
                    "aircraft_type": "B738",
                    "route_distance": null,
                    "filed_airspeed": null,
                    "filed_altitude": null,
                    "route": null,
                    "baggage_claim": null,
                    "seats_cabin_business": null,
                    "seats_cabin_coach": null,
                    "seats_cabin_first": null,
                    "gate_origin": null,
                    "gate_destination": null,
                    "terminal_origin": null,
                    "terminal_destination": null,
                    "type": "Airline",
                    "actual_runway_off": null,
                    "actual_runway_on": null,
                    "foresight_predictions_available": false
                },
                {
                    "ident": "AA100-CURRENT",
                    "ident_icao": "AAL100",
                    "ident_iata": "AA100",
                    "fa_flight_id": "AAL100-1234-current",
                    "operator": "AAL",
                    "operator_icao": "AAL",
                    "operator_iata": "AA",
                    "flight_number": "100",
                    "registration": "N12346",
                    "atc_ident": null,
                    "inbound_fa_flight_id": null,
                    "codeshares": [],
                    "codeshares_iata": [],
                    "blocked": false,
                    "diverted": false,
                    "cancelled": false,
                    "position_only": false,
                    "origin": null,
                    "destination": null,
                    "departure_delay": 0,
                    "arrival_delay": 0,
                    "filed_ete": null,
                    "scheduled_out": null,
                    "estimated_out": null,
                    "actual_out": null,
                    "scheduled_off": "2025-11-16T10:00:00Z",
                    "estimated_off": "2025-11-16T10:00:00Z",
                    "actual_off": "2025-11-16T10:05:00Z",
                    "scheduled_on": "2025-11-16T11:30:00Z",
                    "estimated_on": "2025-11-16T11:30:00Z",
                    "actual_on": null,
                    "scheduled_in": null,
                    "estimated_in": null,
                    "actual_in": null,
                    "progress_percent": 45,
                    "status": "En Route",
                    "aircraft_type": "B738",
                    "route_distance": null,
                    "filed_airspeed": null,
                    "filed_altitude": null,
                    "route": null,
                    "baggage_claim": null,
                    "seats_cabin_business": null,
                    "seats_cabin_coach": null,
                    "seats_cabin_first": null,
                    "gate_origin": null,
                    "gate_destination": null,
                    "terminal_origin": null,
                    "terminal_destination": null,
                    "type": "Airline",
                    "actual_runway_off": null,
                    "actual_runway_on": null,
                    "foresight_predictions_available": false
                },
                {
                    "ident": "AA100-FUTURE",
                    "ident_icao": "AAL100",
                    "ident_iata": "AA100",
                    "fa_flight_id": "AAL100-1234-future",
                    "operator": "AAL",
                    "operator_icao": "AAL",
                    "operator_iata": "AA",
                    "flight_number": "100",
                    "registration": "N12347",
                    "atc_ident": null,
                    "inbound_fa_flight_id": null,
                    "codeshares": [],
                    "codeshares_iata": [],
                    "blocked": false,
                    "diverted": false,
                    "cancelled": false,
                    "position_only": false,
                    "origin": null,
                    "destination": null,
                    "departure_delay": 0,
                    "arrival_delay": 0,
                    "filed_ete": null,
                    "scheduled_out": null,
                    "estimated_out": null,
                    "actual_out": null,
                    "scheduled_off": "2025-11-16T14:00:00Z",
                    "estimated_off": "2025-11-16T14:00:00Z",
                    "actual_off": null,
                    "scheduled_on": "2025-11-16T16:00:00Z",
                    "estimated_on": "2025-11-16T16:00:00Z",
                    "actual_on": null,
                    "scheduled_in": null,
                    "estimated_in": null,
                    "actual_in": null,
                    "progress_percent": 0,
                    "status": "Scheduled",
                    "aircraft_type": "B738",
                    "route_distance": null,
                    "filed_airspeed": null,
                    "filed_altitude": null,
                    "route": null,
                    "baggage_claim": null,
                    "seats_cabin_business": null,
                    "seats_cabin_coach": null,
                    "seats_cabin_first": null,
                    "gate_origin": null,
                    "gate_destination": null,
                    "terminal_origin": null,
                    "terminal_destination": null,
                    "type": "Airline",
                    "actual_runway_off": null,
                    "actual_runway_on": null,
                    "foresight_predictions_available": false
                }
            ],
            "links": null,
            "num_pages": 1
        }"#;

        let response: flightaware::types::GetFlightResponse =
            serde_json::from_str(json_data).expect("Failed to parse test JSON");

        // At current time 2025-11-16T13:24:30Z, target is 11:24:30Z
        // Flight 1 (OLD):     Arrives 10:00:00 - Distance from target: 1h 24m 30s
        // Flight 2 (CURRENT): Arrives 11:30:00 - Distance from target: 5m 30s  ← CLOSEST
        // Flight 3 (FUTURE):  Arrives 16:00:00 - Distance from target: 4h 35m 30s

        let selected = select_relevant_flight(&response.flights);

        assert!(selected.is_some());
        let flight = selected.unwrap();

        // Should select AA100-CURRENT as it's closest to target time (11:24:30)
        assert_eq!(flight.ident, "AA100-CURRENT");
        assert_eq!(flight.fa_flight_id, "AAL100-1234-current");
    }

    #[test]
    fn test_select_relevant_flight_fallback_to_first_when_no_estimated_arrival() {
        let json_data = r#"{
            "flights": [
                {
                    "ident": "AA100-FIRST",
                    "ident_icao": "AAL100",
                    "ident_iata": "AA100",
                    "fa_flight_id": "AAL100-first",
                    "operator": "AAL",
                    "operator_icao": "AAL",
                    "operator_iata": "AA",
                    "flight_number": "100",
                    "registration": "N12345",
                    "atc_ident": null,
                    "inbound_fa_flight_id": null,
                    "codeshares": [],
                    "codeshares_iata": [],
                    "blocked": false,
                    "diverted": false,
                    "cancelled": true,
                    "position_only": false,
                    "origin": null,
                    "destination": null,
                    "departure_delay": 0,
                    "arrival_delay": 0,
                    "filed_ete": null,
                    "scheduled_out": null,
                    "estimated_out": null,
                    "actual_out": null,
                    "scheduled_off": "2025-11-16T10:00:00Z",
                    "estimated_off": null,
                    "actual_off": null,
                    "scheduled_on": "2025-11-16T14:00:00Z",
                    "estimated_on": null,
                    "actual_on": null,
                    "scheduled_in": null,
                    "estimated_in": null,
                    "actual_in": null,
                    "progress_percent": 0,
                    "status": "Cancelled",
                    "aircraft_type": null,
                    "route_distance": null,
                    "filed_airspeed": null,
                    "filed_altitude": null,
                    "route": null,
                    "baggage_claim": null,
                    "seats_cabin_business": null,
                    "seats_cabin_coach": null,
                    "seats_cabin_first": null,
                    "gate_origin": null,
                    "gate_destination": null,
                    "terminal_origin": null,
                    "terminal_destination": null,
                    "type": "Airline",
                    "actual_runway_off": null,
                    "actual_runway_on": null,
                    "foresight_predictions_available": false
                }
            ],
            "links": null,
            "num_pages": 1
        }"#;

        let response: flightaware::types::GetFlightResponse =
            serde_json::from_str(json_data).expect("Failed to parse test JSON");

        let selected = select_relevant_flight(&response.flights);

        assert!(selected.is_some());
        // Should fall back to first flight when none have estimated_on
        assert_eq!(selected.unwrap().ident, "AA100-FIRST");
    }
}
