# Flight UI (flui)

A terminal-based flight tracking application using the FlightAware AeroAPI with a beautiful TUI.

**Perfect for monitoring your flight while programming!** Get visual alerts when your flight is approaching landing so you never miss your boarding call.

## Features

- **Real-time flight tracking** with FlightAware AeroAPI
- **Landing alerts** - Visual notifications when flight is approaching landing:
  - 🔔 Terminal bell on first alert
  - ⚠️ Red blinking borders 
  - Bold "LANDING SOON" warnings
  - Configurable threshold (default: 30 minutes)
- **Terminal UI** displaying:
  - Flight number
  - Current status (On Time, Delayed, Cancelled, En Route)
  - Estimated arrival time (converted to local timezone)
  - Progress percentage and time remaining
  - Animated flight path with airplane icon
  - Origin and destination airports
- **Intelligent flight selection** - automatically selects the most relevant flight when multiple flights share the same flight number
- Clean separation between API layer and view models
- Mock server support for development

## Usage

The TUI will display flight information in a clean terminal interface and automatically refresh every 3 minutes (configurable). Press `q` or `ESC` to exit.

### Configuration Options

All configuration can be set via command line flags or environment variables:

| Flag | Environment Variable | Default | Description |
|------|---------------------|---------|-------------|
| `--flight-number` | `FLIGHT_NUMBER` | (required) | Flight number to track (e.g., AA100) |
| `--api-key` | `FLIGHTAWARE_API_KEY` | (required) | Your FlightAware API key |
| `--refresh-interval` | `REFRESH_INTERVAL` | 180 | Seconds between API updates |
| `--alert-threshold-minutes` | `ALERT_THRESHOLD_MINUTES` | 30 | Minutes before landing to trigger alerts |

Example with custom alert threshold (alert 15 minutes before landing):

```bash
cargo run -- --flight-number AA100 --api-key YOUR_KEY --alert-threshold-minutes 15
```

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
- Display the TUI with sample data

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

### Configuring Refresh Interval

By default, the application refreshes flight data every 5 seconds. You can customize this:

```bash
# Refresh every 10 seconds
cargo run -- --flight-number AA100 --api-key YOUR_KEY --refresh-interval 10

# Or via environment variable
export REFRESH_INTERVAL=10
cargo run -- --flight-number AA100 --api-key YOUR_KEY
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

# UI module tests
cargo test ui::tests
```

## Architecture

- **flui** - Main application with CLI, view models, and TUI
- **flightaware** - Generated SDK from FlightAware OpenAPI specification

### Key Components

- `flight_status.rs` - View model for flight status display
- `api_converter.rs` - Converts API responses to view models using `From` trait
- `ui.rs` - Terminal UI rendering with ratatui
- `mock_server.rs` - Optional mock HTTP server for development
- `main.rs` - CLI application entry point

### UI Features

The terminal UI uses [ratatui](https://ratatui.rs/) to provide:
- Color-coded status indicators (Green=OnTime, Yellow=Delayed, Red=Cancelled, Blue=EnRoute)
- **Animated flight path progress bar** with:
  - Origin and destination airport codes (IATA/ICAO)
  - Airplane icon (✈) showing current position
  - Yellow trail behind the airplane showing distance traveled
  - Gray path ahead showing remaining distance
  - Dots marking departure and arrival airports
- Clean bordered layout with clear information hierarchy
- Responsive design that adapts to terminal size
- **Timezone-aware formatting** - arrival times are automatically converted to your local timezone (e.g., "Nov 18, 2025 at 2:30 PM EST")
- **Auto-refresh** - Flight data updates every 5 seconds (configurable)

## Configuration

The application accepts flight number and API key via:
1. Command line flags: `--flight-number` and `--api-key`
2. Environment variables: `FLIGHT_NUMBER` and `FLIGHTAWARE_API_KEY`
