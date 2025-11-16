use clap::Parser;
use flightaware::Client;
use std::fmt;

mod flight_status;
use flight_status::{FlightStatus, FlightStatusViewModel};

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
}

#[derive(Debug)]
pub struct Config {
    pub flight_number: String,
    pub flight_aware_api_key: String,
}

impl Config {
    pub fn from_options(
        flight_number: Option<String>,
        api_key: Option<String>,
    ) -> Result<Self, ConfigurationError> {
        let flight_number = flight_number.ok_or(ConfigurationError::MissingFlightNumber)?;
        let flight_aware_api_key = api_key.ok_or(ConfigurationError::MissingApiKey)?;

        Ok(Config {
            flight_number,
            flight_aware_api_key,
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
    Config::from_options(args.flight_number, args.api_key)
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

    let flight_status = client
        .get_flight(&config.flight_number, None, None, None, None, None)
        .await;

    let flight_view_model = match flight_status {
        Ok(response) => {
            if let Some(flight) = response.flights.first() {
                FlightStatusViewModel::from(flight)
            } else {
                println!("{response:#?}");
                println!("No flight data found for {}", config.flight_number);
                return Ok(());
            }
        }
        Err(e) => {
            println!("Error fetching flight data: {}", e);
            return Ok(());
        }
    };

    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;
    
    // Draw the UI
    terminal.draw(|frame| {
        ui::render_flight_status(frame, &flight_view_model);
    })?;
    
    // Wait for user input before exiting (press 'q' or ESC to quit)
    use crossterm::event::{self, Event, KeyCode};
    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                break;
            }
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
        let result =
            Config::from_options(Some("AA100".to_string()), Some("test-api-key".to_string()));

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.flight_number, "AA100");
        assert_eq!(config.flight_aware_api_key, "test-api-key");
    }

    #[test]
    fn test_config_from_options_missing_flight_number() {
        let result = Config::from_options(None, Some("test-api-key".to_string()));

        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigurationError::MissingFlightNumber => (),
            _ => panic!("Expected MissingFlightNumber error"),
        }
    }

    #[test]
    fn test_config_from_options_missing_api_key() {
        let result = Config::from_options(Some("AA100".to_string()), None);

        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigurationError::MissingApiKey => (),
            _ => panic!("Expected MissingApiKey error"),
        }
    }

    #[test]
    fn test_config_from_options_missing_both() {
        let result = Config::from_options(None, None);

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
}
