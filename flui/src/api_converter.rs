use crate::flight_status::{FlightStatus, FlightStatusViewModel};
use chrono::{DateTime, Utc};

impl From<&flightaware::types::BaseFlight> for FlightStatusViewModel {
    fn from(flight: &flightaware::types::BaseFlight) -> Self {
        let status = determine_flight_status_base(flight);
        
        // Extract airport codes (prefer IATA, fallback to ICAO)
        let origin_airport = flight.origin.as_ref().and_then(|o| {
            o.code_iata.clone().or_else(|| o.code_icao.clone())
        });
        
        let destination_airport = flight.destination.as_ref().and_then(|d| {
            d.code_iata.clone().or_else(|| d.code_icao.clone())
        });
        
        FlightStatusViewModel {
            flight_number: flight.ident.clone(),
            status,
            scheduled_departure: datetime_to_string(flight.scheduled_off.as_ref()),
            scheduled_arrival: datetime_to_string(flight.scheduled_on.as_ref()),
            estimated_departure: datetime_to_string(flight.estimated_off.as_ref()),
            estimated_arrival: datetime_to_string(flight.estimated_on.as_ref()),
            actual_departure: datetime_to_string(flight.actual_off.as_ref()),
            actual_arrival: datetime_to_string(flight.actual_on.as_ref()),
            progress_percent: flight.progress_percent,
            origin_airport,
            destination_airport,
        }
    }
}

// Also implement From for GetFlightResponseFlightsItem (which is actually the same as BaseFlight in structure)
impl From<&flightaware::types::GetFlightResponseFlightsItem> for FlightStatusViewModel {
    fn from(flight: &flightaware::types::GetFlightResponseFlightsItem) -> Self {
        let status = determine_flight_status_response_item(flight);
        
        // Extract airport codes (prefer IATA, fallback to ICAO)
        let origin_airport = flight.origin.as_ref().and_then(|o| {
            o.code_iata.clone().or_else(|| o.code_icao.clone())
        });
        
        let destination_airport = flight.destination.as_ref().and_then(|d| {
            d.code_iata.clone().or_else(|| d.code_icao.clone())
        });
        
        FlightStatusViewModel {
            flight_number: flight.ident.clone(),
            status,
            scheduled_departure: datetime_to_string(flight.scheduled_off.as_ref()),
            scheduled_arrival: datetime_to_string(flight.scheduled_on.as_ref()),
            estimated_departure: datetime_to_string(flight.estimated_off.as_ref()),
            estimated_arrival: datetime_to_string(flight.estimated_on.as_ref()),
            actual_departure: datetime_to_string(flight.actual_off.as_ref()),
            actual_arrival: datetime_to_string(flight.actual_on.as_ref()),
            progress_percent: flight.progress_percent,
            origin_airport,
            destination_airport,
        }
    }
}

fn datetime_to_string(dt: Option<&DateTime<Utc>>) -> Option<String> {
    dt.map(|d| d.to_rfc3339())
}

fn determine_flight_status_base(flight: &flightaware::types::BaseFlight) -> FlightStatus {
    if flight.cancelled {
        return FlightStatus::Cancelled;
    }
    
    if flight.actual_off.is_some() && flight.actual_on.is_none() {
        return FlightStatus::EnRoute;
    }
    
    if let Some(delay) = flight.departure_delay
        && delay > 0 {
            return FlightStatus::Delayed;
        }
    
    if let Some(delay) = flight.arrival_delay
        && delay > 0 {
            return FlightStatus::Delayed;
        }
    
    FlightStatus::OnTime
}

fn determine_flight_status_response_item(flight: &flightaware::types::GetFlightResponseFlightsItem) -> FlightStatus {
    if flight.cancelled {
        return FlightStatus::Cancelled;
    }
    
    if flight.actual_off.is_some() && flight.actual_on.is_none() {
        return FlightStatus::EnRoute;
    }
    
    if let Some(delay) = flight.departure_delay
        && delay > 0 {
            return FlightStatus::Delayed;
        }
    
    if let Some(delay) = flight.arrival_delay
        && delay > 0 {
            return FlightStatus::Delayed;
        }
    
    FlightStatus::OnTime
}

/// Determine flight status based on FlightAware flight data
/// This is pub for testing purposes
pub fn determine_flight_status(flight: &flightaware::types::BaseFlight) -> FlightStatus {
    determine_flight_status_base(flight)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_from_conversion() {
        use flightaware::types::{BaseFlight, BaseFlightType};
        use chrono::TimeZone;
        
        let flight = BaseFlight {
            ident: "AA100".to_string(),
            ident_iata: None,
            ident_icao: None,
            fa_flight_id: "test".to_string(),
            operator: None,
            operator_iata: None,
            operator_icao: None,
            flight_number: None,
            registration: None,
            atc_ident: None,
            inbound_fa_flight_id: None,
            codeshares: None,
            codeshares_iata: None,
            blocked: false,
            diverted: false,
            cancelled: false,
            position_only: false,
            origin: None,
            destination: None,
            departure_delay: Some(0),
            arrival_delay: Some(0),
            filed_ete: None,
            scheduled_out: None,
            estimated_out: None,
            actual_out: None,
            scheduled_off: Some(Utc.with_ymd_and_hms(2025, 11, 16, 10, 0, 0).unwrap()),
            estimated_off: Some(Utc.with_ymd_and_hms(2025, 11, 16, 10, 0, 0).unwrap()),
            actual_off: None,
            scheduled_on: Some(Utc.with_ymd_and_hms(2025, 11, 16, 14, 0, 0).unwrap()),
            estimated_on: Some(Utc.with_ymd_and_hms(2025, 11, 16, 14, 0, 0).unwrap()),
            actual_on: None,
            scheduled_in: None,
            estimated_in: None,
            actual_in: None,
            progress_percent: None,
            status: "Scheduled".to_string(),
            aircraft_type: None,
            route_distance: None,
            filed_airspeed: None,
            filed_altitude: None,
            route: None,
            baggage_claim: None,
            seats_cabin_business: None,
            seats_cabin_coach: None,
            seats_cabin_first: None,
            gate_origin: None,
            gate_destination: None,
            terminal_origin: None,
            terminal_destination: None,
            type_: BaseFlightType::Airline,
            actual_runway_off: None,
            actual_runway_on: None,
        };
        
        // Test using From trait
        let view_model = FlightStatusViewModel::from(&flight);
        assert_eq!(view_model.flight_number, "AA100");
        assert_eq!(view_model.status, FlightStatus::OnTime);
        
        // Test using into()
        let view_model2: FlightStatusViewModel = (&flight).into();
        assert_eq!(view_model2.flight_number, "AA100");
    }
    
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
    
    #[test]
    fn test_status_determination_cancelled() {
        use flightaware::types::{BaseFlight, BaseFlightType};
        
        let flight = BaseFlight {
            ident: "AA100".to_string(),
            ident_iata: None,
            ident_icao: None,
            fa_flight_id: "test".to_string(),
            operator: None,
            operator_iata: None,
            operator_icao: None,
            flight_number: None,
            registration: None,
            atc_ident: None,
            inbound_fa_flight_id: None,
            codeshares: None,
            codeshares_iata: None,
            blocked: false,
            diverted: false,
            cancelled: true,  // Cancelled
            position_only: false,
            origin: None,
            destination: None,
            departure_delay: Some(0),
            arrival_delay: Some(0),
            filed_ete: None,
            scheduled_out: None,
            estimated_out: None,
            actual_out: None,
            scheduled_off: None,
            estimated_off: None,
            actual_off: None,
            scheduled_on: None,
            estimated_on: None,
            actual_on: None,
            scheduled_in: None,
            estimated_in: None,
            actual_in: None,
            progress_percent: None,
            status: "Cancelled".to_string(),
            aircraft_type: None,
            route_distance: None,
            filed_airspeed: None,
            filed_altitude: None,
            route: None,
            baggage_claim: None,
            seats_cabin_business: None,
            seats_cabin_coach: None,
            seats_cabin_first: None,
            gate_origin: None,
            gate_destination: None,
            terminal_origin: None,
            terminal_destination: None,
            type_: BaseFlightType::Airline,
            actual_runway_off: None,
            actual_runway_on: None,
        };
        
        assert_eq!(determine_flight_status(&flight), FlightStatus::Cancelled);
    }
    
    #[test]
    fn test_status_determination_delayed() {
        use flightaware::types::{BaseFlight, BaseFlightType};
        
        let flight = BaseFlight {
            ident: "AA100".to_string(),
            ident_iata: None,
            ident_icao: None,
            fa_flight_id: "test".to_string(),
            operator: None,
            operator_iata: None,
            operator_icao: None,
            flight_number: None,
            registration: None,
            atc_ident: None,
            inbound_fa_flight_id: None,
            codeshares: None,
            codeshares_iata: None,
            blocked: false,
            diverted: false,
            cancelled: false,
            position_only: false,
            origin: None,
            destination: None,
            departure_delay: Some(900),  // 15 minutes delay
            arrival_delay: Some(0),
            filed_ete: None,
            scheduled_out: None,
            estimated_out: None,
            actual_out: None,
            scheduled_off: None,
            estimated_off: None,
            actual_off: None,
            scheduled_on: None,
            estimated_on: None,
            actual_on: None,
            scheduled_in: None,
            estimated_in: None,
            actual_in: None,
            progress_percent: None,
            status: "Scheduled".to_string(),
            aircraft_type: None,
            route_distance: None,
            filed_airspeed: None,
            filed_altitude: None,
            route: None,
            baggage_claim: None,
            seats_cabin_business: None,
            seats_cabin_coach: None,
            seats_cabin_first: None,
            gate_origin: None,
            gate_destination: None,
            terminal_origin: None,
            terminal_destination: None,
            type_: BaseFlightType::Airline,
            actual_runway_off: None,
            actual_runway_on: None,
        };
        
        assert_eq!(determine_flight_status(&flight), FlightStatus::Delayed);
    }
}
