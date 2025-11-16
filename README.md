# Flight UI (flui)

A terminal-based flight tracking application using the FlightAware AeroAPI.

## Features

- Track flight status in real-time
- View flight schedules, delays, and current status
- Clean separation between API layer and view models
- Mock server support for development

## Development

### Running with Mock Server

To avoid hitting the real FlightAware API during development, use the `httpmock` feature:

```bash
cargo run --features httpmock -- --flight-number AA100 --api-key fake-key
```

This will:
- Start a local HTTP mock server
- Return sample flight data for any flight query
- Allow you to test the application without a real API key

### Running with Real API

To use the real FlightAware API:

```bash
cargo run -- --flight-number AA100 --api-key YOUR_ACTUAL_KEY
```

Or use environment variables:

```bash
export FLIGHT_NUMBER=AA100
export FLIGHTAWARE_API_KEY=your_key_here
cargo run
```

## Testing

Run all tests:

```bash
cargo test
```

Run specific test suites:

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test flight_api_test
```

## Architecture

- **flui** - Main application with CLI and view models
- **flightaware** - Generated SDK from FlightAware OpenAPI specification

### Key Components

- `flight_status.rs` - View model for flight status display
- `api_converter.rs` - Converts API responses to view models using `From` trait
- `mock_server.rs` - Optional mock HTTP server for development
- `main.rs` - CLI application entry point

## Configuration

The application accepts flight number and API key via:
1. Command line flags: `--flight-number` and `--api-key`
2. Environment variables: `FLIGHT_NUMBER` and `FLIGHTAWARE_API_KEY`
