use derive_builder::Builder;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlightStatus {
    #[default]
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

impl From<FlightStatusViewModel> for FlightStatusViewModelBuilder {
    fn from(view_model: FlightStatusViewModel) -> Self {
        let mut builder = FlightStatusViewModelBuilder::default();
        builder.flight_number(view_model.flight_number);
        builder.status(view_model.status);
        builder.scheduled_departure(view_model.scheduled_departure);
        builder.scheduled_arrival(view_model.scheduled_arrival);
        builder.estimated_departure(view_model.estimated_departure);
        builder.estimated_arrival(view_model.estimated_arrival);
        builder.actual_departure(view_model.actual_departure);
        builder.actual_arrival(view_model.actual_arrival);
        builder.progress_percent(view_model.progress_percent);
        builder.origin_airport(view_model.origin_airport);
        builder.destination_airport(view_model.destination_airport);
        builder
    }
}

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(into), default)]
pub struct FlightStatusViewModel {
    pub flight_number: String,
    pub status: FlightStatus,
    pub scheduled_departure: Option<String>,
    pub scheduled_arrival: Option<String>,
    pub estimated_departure: Option<String>,
    pub estimated_arrival: Option<String>,
    pub actual_departure: Option<String>,
    pub actual_arrival: Option<String>,
    pub progress_percent: Option<i64>,
    pub origin_airport: Option<String>,
    pub destination_airport: Option<String>,
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

    /// Format arrival time for display in local timezone
    /// Returns a human-readable formatted time string
    pub fn formatted_arrival_time(&self) -> Option<String> {
        use chrono::{DateTime, Local, Utc};

        let time_str = self.arrival_time()?;

        // Parse the ISO 8601 timestamp
        let utc_time: DateTime<Utc> = time_str.parse().ok()?;

        // Convert to local timezone
        let local_time: DateTime<Local> = utc_time.into();

        // Format as: "Nov 18, 2025 at 2:30 PM EST"
        Some(local_time.format("%b %-d, %Y at %-I:%M %p %Z").to_string())
    }

    pub fn progress_percentage(&self) -> f64 {
        self.progress_percent.map(|p| p as f64).unwrap_or(0.0)
    }

    /// Calculate time remaining until arrival
    /// Returns a formatted string like "2h 30m" or None if unavailable
    pub fn time_remaining(&self) -> Option<String> {
        use chrono::{DateTime, Utc};

        // Only calculate if flight hasn't arrived yet
        if self.actual_arrival.is_some() {
            return Some("Arrived".to_string());
        }

        let arrival_str = self.estimated_arrival.as_deref()?;
        let arrival_time: DateTime<Utc> = arrival_str.parse().ok()?;
        let now = Utc::now();

        let duration = arrival_time.signed_duration_since(now);

        if duration.num_seconds() < 0 {
            return Some("Arrived".to_string());
        }

        let hours = duration.num_hours();
        let minutes = (duration.num_minutes() % 60).abs();

        if hours > 0 {
            Some(format!("{}h {}m", hours, minutes))
        } else {
            Some(format!("{}m", minutes))
        }
    }

    /// Check if the flight is approaching landing (within threshold minutes)
    pub fn is_approaching_landing(&self, threshold_minutes: i64) -> bool {
        use chrono::{DateTime, Utc};

        // Already landed
        if self.actual_arrival.is_some() {
            return false;
        }

        let arrival_str = match self.estimated_arrival.as_deref() {
            Some(s) => s,
            None => return false,
        };

        let arrival_time: DateTime<Utc> = match arrival_str.parse() {
            Ok(t) => t,
            Err(_) => return false,
        };

        let now = Utc::now();
        let duration = arrival_time.signed_duration_since(now);

        // Within threshold and not yet arrived
        duration.num_minutes() > 0 && duration.num_minutes() <= threshold_minutes
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
            progress_percent: Some(50),
            origin_airport: None,
            destination_airport: None,
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
            progress_percent: Some(50),
            origin_airport: None,
            destination_airport: None,
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
            progress_percent: Some(100),
            origin_airport: None,
            destination_airport: None,
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
            progress_percent: Some(50),
            origin_airport: None,
            destination_airport: None,
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
            progress_percent: Some(50),
            origin_airport: None,
            destination_airport: None,
        };

        assert_eq!(view_model.departure_time(), None);
        assert_eq!(view_model.arrival_time(), None);
    }

    #[test]
    fn test_progress_percentage_some() {
        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::EnRoute,
            scheduled_departure: Some("10:00".to_string()),
            scheduled_arrival: Some("14:00".to_string()),
            estimated_departure: Some("10:15".to_string()),
            estimated_arrival: Some("14:20".to_string()),
            actual_departure: Some("10:20".to_string()),
            actual_arrival: None,
            progress_percent: Some(45),
            origin_airport: None,
            destination_airport: None,
        };

        assert_eq!(view_model.progress_percentage(), 45.0);
    }

    #[test]
    fn test_progress_percentage_none() {
        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::Cancelled,
            scheduled_departure: Some("10:00".to_string()),
            scheduled_arrival: Some("14:00".to_string()),
            estimated_departure: None,
            estimated_arrival: None,
            actual_departure: None,
            actual_arrival: None,
            progress_percent: None,
            origin_airport: None,
            destination_airport: None,
        };

        assert_eq!(view_model.progress_percentage(), 0.0);
    }

    #[test]
    fn test_formatted_arrival_time() {
        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::OnTime,
            scheduled_departure: Some("2025-11-16T10:00:00Z".to_string()),
            scheduled_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            estimated_departure: Some("2025-11-16T10:00:00Z".to_string()),
            estimated_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            actual_departure: None,
            actual_arrival: None,
            progress_percent: Some(0),
            origin_airport: None,
            destination_airport: None,
        };

        let formatted = view_model.formatted_arrival_time();
        assert!(formatted.is_some());

        // The formatted string should contain the year
        let formatted_str = formatted.unwrap();
        assert!(formatted_str.contains("2025"));

        // Should contain "at" separator
        assert!(formatted_str.contains("at"));
    }

    #[test]
    fn test_formatted_arrival_time_none() {
        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::Cancelled,
            scheduled_departure: None,
            scheduled_arrival: None,
            estimated_departure: None,
            estimated_arrival: None,
            actual_departure: None,
            actual_arrival: None,
            progress_percent: None,
            origin_airport: None,
            destination_airport: None,
        };

        assert!(view_model.formatted_arrival_time().is_none());
    }

    #[test]
    fn test_builder_basic() {
        let view_model = FlightStatusViewModelBuilder::default()
            .flight_number("AA100")
            .status(FlightStatus::OnTime)
            .scheduled_departure(Some("2025-11-16T10:00:00Z".to_string()))
            .scheduled_arrival(Some("2025-11-16T14:00:00Z".to_string()))
            .estimated_departure(Some("2025-11-16T10:00:00Z".to_string()))
            .estimated_arrival(Some("2025-11-16T14:00:00Z".to_string()))
            .actual_departure(None)
            .actual_arrival(None)
            .progress_percent(Some(0))
            .build()
            .unwrap();

        assert_eq!(view_model.flight_number, "AA100");
        assert_eq!(view_model.status, FlightStatus::OnTime);
        assert_eq!(view_model.progress_percent, Some(0));
    }

    #[test]
    fn test_builder_with_none_values() {
        let view_model = FlightStatusViewModelBuilder::default()
            .flight_number("DL456")
            .status(FlightStatus::Cancelled)
            .scheduled_departure(None)
            .scheduled_arrival(None)
            .estimated_departure(None)
            .estimated_arrival(None)
            .actual_departure(None)
            .actual_arrival(None)
            .progress_percent(None)
            .build()
            .unwrap();

        assert_eq!(view_model.flight_number, "DL456");
        assert_eq!(view_model.status, FlightStatus::Cancelled);
        assert!(view_model.formatted_arrival_time().is_none());
    }

    #[test]
    fn test_builder_with_defaults() {
        // Builder sets Option fields to None by default, only requires non-Option fields
        let view_model = FlightStatusViewModelBuilder::default()
            .flight_number("UA789")
            .status(FlightStatus::Delayed)
            .build()
            .unwrap();

        assert_eq!(view_model.flight_number, "UA789");
        assert_eq!(view_model.status, FlightStatus::Delayed);
        // All Option fields should be None when not set
        assert!(view_model.scheduled_departure.is_none());
        assert!(view_model.scheduled_arrival.is_none());
    }

    #[test]
    fn test_time_remaining_arrived() {
        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::OnTime,
            scheduled_departure: Some("2025-11-16T10:00:00Z".to_string()),
            scheduled_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            estimated_departure: Some("2025-11-16T10:00:00Z".to_string()),
            estimated_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            actual_departure: Some("2025-11-16T10:05:00Z".to_string()),
            actual_arrival: Some("2025-11-16T14:10:00Z".to_string()),
            progress_percent: Some(100),
            origin_airport: None,
            destination_airport: None,
        };

        assert_eq!(view_model.time_remaining(), Some("Arrived".to_string()));
    }

    #[test]
    fn test_time_remaining_none() {
        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::OnTime,
            scheduled_departure: Some("2025-11-16T10:00:00Z".to_string()),
            scheduled_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            estimated_departure: Some("2025-11-16T10:00:00Z".to_string()),
            estimated_arrival: None,
            actual_departure: None,
            actual_arrival: None,
            progress_percent: Some(0),
            origin_airport: None,
            destination_airport: None,
        };

        assert_eq!(view_model.time_remaining(), None);
    }

    #[test]
    fn test_is_approaching_landing_true() {
        use chrono::{Duration, Utc};

        // Flight arriving in 20 minutes
        let arrival_time = Utc::now() + Duration::minutes(20);

        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::EnRoute,
            scheduled_departure: Some("2025-11-16T10:00:00Z".to_string()),
            scheduled_arrival: Some(arrival_time.to_rfc3339()),
            estimated_departure: Some("2025-11-16T10:00:00Z".to_string()),
            estimated_arrival: Some(arrival_time.to_rfc3339()),
            actual_departure: Some("2025-11-16T10:05:00Z".to_string()),
            actual_arrival: None,
            progress_percent: Some(85),
            origin_airport: None,
            destination_airport: None,
        };

        assert!(view_model.is_approaching_landing(30));
        assert!(view_model.is_approaching_landing(20));
    }

    #[test]
    fn test_is_approaching_landing_false_too_far() {
        use chrono::{Duration, Utc};

        // Flight arriving in 45 minutes
        let arrival_time = Utc::now() + Duration::minutes(45);

        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::EnRoute,
            scheduled_departure: Some("2025-11-16T10:00:00Z".to_string()),
            scheduled_arrival: Some(arrival_time.to_rfc3339()),
            estimated_departure: Some("2025-11-16T10:00:00Z".to_string()),
            estimated_arrival: Some(arrival_time.to_rfc3339()),
            actual_departure: Some("2025-11-16T10:05:00Z".to_string()),
            actual_arrival: None,
            progress_percent: Some(50),
            origin_airport: None,
            destination_airport: None,
        };

        assert!(!view_model.is_approaching_landing(30));
    }

    #[test]
    fn test_is_approaching_landing_false_already_arrived() {
        let view_model = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::OnTime,
            scheduled_departure: Some("2025-11-16T10:00:00Z".to_string()),
            scheduled_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            estimated_departure: Some("2025-11-16T10:00:00Z".to_string()),
            estimated_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            actual_departure: Some("2025-11-16T10:05:00Z".to_string()),
            actual_arrival: Some("2025-11-16T14:10:00Z".to_string()),
            progress_percent: Some(100),
            origin_airport: None,
            destination_airport: None,
        };

        assert!(!view_model.is_approaching_landing(30));
    }
}
