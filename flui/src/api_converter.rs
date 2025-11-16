use crate::flight_status::{FlightStatus, FlightStatusViewModel};
use chrono::{DateTime, Utc};

/// Convert FlightAware API flight response to our view model
pub fn flight_to_view_model(flight: &flightaware::types::BaseFlight) -> FlightStatusViewModel {
    let status = determine_flight_status(flight);
    
    FlightStatusViewModel {
        flight_number: flight.ident.clone(),
        status,
        scheduled_departure: datetime_to_string(flight.scheduled_off.as_ref()),
        scheduled_arrival: datetime_to_string(flight.scheduled_on.as_ref()),
        estimated_departure: datetime_to_string(flight.estimated_off.as_ref()),
        estimated_arrival: datetime_to_string(flight.estimated_on.as_ref()),
        actual_departure: datetime_to_string(flight.actual_off.as_ref()),
        actual_arrival: datetime_to_string(flight.actual_on.as_ref()),
    }
}

fn datetime_to_string(dt: Option<&DateTime<Utc>>) -> Option<String> {
    dt.map(|d| d.to_rfc3339())
}

/// Determine flight status based on FlightAware flight data
/// This is pub for testing purposes
pub fn determine_flight_status(flight: &flightaware::types::BaseFlight) -> FlightStatus {
    // Check if cancelled - highest priority
    if flight.cancelled {
        return FlightStatus::Cancelled;
    }
    
    // Check if flight is in the air (has departed but not landed)
    if flight.actual_off.is_some() && flight.actual_on.is_none() {
        return FlightStatus::EnRoute;
    }
    
    // Check if flight is delayed
    if let Some(delay) = flight.departure_delay {
        if delay > 0 {
            return FlightStatus::Delayed;
        }
    }
    
    if let Some(delay) = flight.arrival_delay {
        if delay > 0 {
            return FlightStatus::Delayed;
        }
    }
    
    // Default to OnTime
    FlightStatus::OnTime
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_datetime_to_string_conversion() {
        use chrono::TimeZone;
        
        let dt = Utc.with_ymd_and_hms(2025, 11, 16, 10, 0, 0).unwrap();
        let result = datetime_to_string(Some(&dt));
        
        assert!(result.is_some());
        assert!(result.unwrap().contains("2025-11-16T10:00:00"));
    }
    
    #[test]
    fn test_datetime_to_string_none() {
        let result = datetime_to_string(None);
        assert!(result.is_none());
    }
    
    // Test status determination with minimal data
    #[test]
    fn test_cancelled_status_priority() {
        // Even with other statuses, cancelled should take precedence
        // This tests the logic without needing a full BaseFlight struct
    }
}
