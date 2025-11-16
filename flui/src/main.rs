use clap::Parser;
use flightaware::Client;
use std::fmt;

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

    pub fn create_client(&self) -> Client {
        Client::new_with_client(
            "https://aeroapi.flightaware.com/aeroapi",
            reqwest::Client::builder()
                .default_headers({
                    let mut headers = reqwest::header::HeaderMap::new();
                    headers.insert(
                        "x-apikey",
                        reqwest::header::HeaderValue::from_str(&self.flight_aware_api_key)
                            .expect("Invalid API key"),
                    );
                    headers
                })
                .build()
                .expect("Failed to build HTTP client"),
        )
    }
}

fn get_config() -> Result<Config, ConfigurationError> {
    let args = CliArgs::parse();
    Config::from_options(args.flight_number, args.api_key)
}

fn main() {
    let config = get_config().unwrap();
    let _client = config.create_client();
    
    println!("Tracking flight: {}", config.flight_number);
    println!("FlightAware API client initialized");
    
    // In the future, we'll use the client to fetch flight data
    // For now, just show that we have the SDK integrated
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
