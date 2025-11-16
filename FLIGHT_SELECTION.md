# Flight Selection Logic

## Overview

When querying FlightAware for a flight by its flight number (e.g., "AA100"), the API may return multiple flights with that same flight number. This is because the same flight number is often reused for multiple scheduled flights throughout the day or week.

## Selection Strategy

The application implements intelligent flight selection to show the most relevant flight to the user:

### Target Time Calculation
```
Target Time = Current Time - 2 hours
```

### Selection Algorithm

1. **Filter by Estimated Arrival**: Consider only flights that have an estimated arrival time
2. **Calculate Distance**: For each flight, calculate the time distance from its estimated arrival to the target time
3. **Select Closest**: Choose the flight whose estimated arrival is closest to the target time
4. **Fallback**: If no flights have estimated arrival times, select the first flight in the list

### Rationale

Using "current time minus 2 hours" as the target ensures:
- **Recent Flights**: Shows flights that recently landed or are currently in progress
- **Active Tracking**: Prioritizes flights users are most likely interested in tracking
- **Balance**: Not too far in the past (shows completed flights) nor only future flights

### Example

Current time: `2025-11-16T13:21:30Z`  
Target time: `2025-11-16T11:21:30Z`

Given three AA100 flights:
- Flight 1: Estimated arrival `10:00:00Z` (1h 21m before target) ❌
- Flight 2: Estimated arrival `11:30:00Z` (8m 30s after target) ✅ **Selected**
- Flight 3: Estimated arrival `14:00:00Z` (2h 38m after target) ❌

Flight 2 is selected as its arrival time is closest to the target.

## Implementation

The selection logic is implemented in `src/main.rs`:

```rust
fn select_relevant_flight(
    flights: &[flightaware::types::GetFlightResponseFlightsItem],
) -> Option<&flightaware::types::GetFlightResponseFlightsItem>
```

This function:
- Returns `None` if the flight list is empty
- Finds the flight with minimum time distance to target
- Falls back to the first flight if no estimated arrivals exist

## Testing

The selection logic is tested through:
- Unit tests for edge cases (empty list)
- Integration tests with real API response data
- Manual testing with the mock server
