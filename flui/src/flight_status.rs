use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum FlightStatus {
    OnTime,
    Delayed,
    Cancelled,
    EnRoute,
}

impl fmt::Display for FlightStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FlightStatus::OnTime => write!(f, "On Time"),
            FlightStatus::Delayed => write!(f, "Delayed"),
            FlightStatus::Cancelled => write!(f, "Cancelled"),
            FlightStatus::EnRoute => write!(f, "En Route"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlightStatusViewModel {
    pub flight_number: String,
    pub status: FlightStatus,
    pub scheduled_departure: Option<String>,
    pub scheduled_arrival: Option<String>,
    pub estimated_departure: Option<String>,
    pub estimated_arrival: Option<String>,
    pub actual_departure: Option<String>,
    pub actual_arrival: Option<String>,
}

impl FlightStatusViewModel {
    pub fn departure_time(&self) -> Option<&str> {
        self.actual_departure
            .as_deref()
            .or(self.estimated_departure.as_deref())
    }

    pub fn arrival_time(&self) -> Option<&str> {
        self.actual_arrival
            .as_deref()
            .or(self.estimated_arrival.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flight_status_display() {
        assert_eq!(FlightStatus::OnTime.to_string(), "On Time");
        assert_eq!(FlightStatus::Delayed.to_string(), "Delayed");
        assert_eq!(FlightStatus::Cancelled.to_string(), "Cancelled");
        assert_eq!(FlightStatus::EnRoute.to_string(), "En Route");
    }

    #[test]
    fn test_flight_status_view_model_departure_time_actual() {
        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::EnRoute,
            scheduled_departure: Some("10:00".to_string()),
            scheduled_arrival: Some("14:00".to_string()),
            estimated_departure: Some("10:15".to_string()),
            estimated_arrival: Some("14:20".to_string()),
            actual_departure: Some("10:20".to_string()),
            actual_arrival: None,
        };

        assert_eq!(view_model.departure_time(), Some("10:20"));
    }

    #[test]
    fn test_flight_status_view_model_departure_time_estimated() {
        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::OnTime,
            scheduled_departure: Some("10:00".to_string()),
            scheduled_arrival: Some("14:00".to_string()),
            estimated_departure: Some("10:15".to_string()),
            estimated_arrival: Some("14:20".to_string()),
            actual_departure: None,
            actual_arrival: None,
        };

        assert_eq!(view_model.departure_time(), Some("10:15"));
    }

    #[test]
    fn test_flight_status_view_model_arrival_time_actual() {
        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::OnTime,
            scheduled_departure: Some("10:00".to_string()),
            scheduled_arrival: Some("14:00".to_string()),
            estimated_departure: Some("10:15".to_string()),
            estimated_arrival: Some("14:20".to_string()),
            actual_departure: Some("10:20".to_string()),
            actual_arrival: Some("14:25".to_string()),
        };

        assert_eq!(view_model.arrival_time(), Some("14:25"));
    }

    #[test]
    fn test_flight_status_view_model_arrival_time_estimated() {
        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::Delayed,
            scheduled_departure: Some("10:00".to_string()),
            scheduled_arrival: Some("14:00".to_string()),
            estimated_departure: Some("10:15".to_string()),
            estimated_arrival: Some("14:20".to_string()),
            actual_departure: None,
            actual_arrival: None,
        };

        assert_eq!(view_model.arrival_time(), Some("14:20"));
    }

    #[test]
    fn test_flight_status_view_model_no_times() {
        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::Cancelled,
            scheduled_departure: Some("10:00".to_string()),
            scheduled_arrival: Some("14:00".to_string()),
            estimated_departure: None,
            estimated_arrival: None,
            actual_departure: None,
            actual_arrival: None,
        };

        assert_eq!(view_model.departure_time(), None);
        assert_eq!(view_model.arrival_time(), None);
    }
}
