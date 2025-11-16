use crate::flight_status::FlightStatusViewModel;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

pub fn render_flight_status(frame: &mut Frame, view_model: &FlightStatusViewModel) {
    let area = frame.area();
    
    // Create layout with 4 rows for our 4 elements
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Flight number
            Constraint::Length(3), // Status
            Constraint::Length(3), // Estimated arrival
            Constraint::Length(3), // Progress bar
        ])
        .split(area);
    
    // Flight Number
    let flight_number_text = format!("Flight: {}", view_model.flight_number);
    let flight_number = Paragraph::new(flight_number_text)
        .block(Block::default().borders(Borders::ALL).title("Flight Information"))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    frame.render_widget(flight_number, chunks[0]);
    
    // Flight Status
    let status_color = match view_model.status {
        crate::flight_status::FlightStatus::OnTime => Color::Green,
        crate::flight_status::FlightStatus::Delayed => Color::Yellow,
        crate::flight_status::FlightStatus::Cancelled => Color::Red,
        crate::flight_status::FlightStatus::EnRoute => Color::Blue,
    };
    
    let status_text = format!("Status: {}", view_model.status);
    let status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(status_color).add_modifier(Modifier::BOLD));
    frame.render_widget(status, chunks[1]);
    
    // Estimated Arrival Time
    let arrival_time = view_model
        .formatted_arrival_time()
        .unwrap_or_else(|| "N/A".to_string());
    let arrival_text = format!("Estimated Arrival: {}", arrival_time);
    let arrival = Paragraph::new(arrival_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    frame.render_widget(arrival, chunks[2]);
    
    // Progress Bar
    let progress = view_model.progress_percentage();
    let progress_label = format!("{:.0}% Complete", progress);
    
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Flight Progress"))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .label(progress_label)
        .ratio(progress / 100.0);
    frame.render_widget(gauge, chunks[3]);
}

// Keep the old calculate_progress function for backwards compatibility in tests
// but it's no longer used in the UI
#[allow(dead_code)]
fn calculate_progress(view_model: &FlightStatusViewModel) -> f64 {
    // For now, return a default based on status
    // In the future, we can calculate based on actual/estimated times and distance
    match view_model.status {
        crate::flight_status::FlightStatus::OnTime => {
            // If we have actual departure but no actual arrival, assume 50% progress
            if view_model.actual_departure.is_some() && view_model.actual_arrival.is_none() {
                50.0
            } else if view_model.actual_arrival.is_some() {
                100.0
            } else {
                0.0
            }
        }
        crate::flight_status::FlightStatus::EnRoute => 50.0,
        crate::flight_status::FlightStatus::Cancelled => 0.0,
        crate::flight_status::FlightStatus::Delayed => {
            if view_model.actual_departure.is_some() {
                50.0
            } else {
                0.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flight_status::FlightStatus;
    
    #[test]
    fn test_calculate_progress_scheduled() {
        let vm = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::OnTime,
            scheduled_departure: Some("2025-11-16T10:00:00Z".to_string()),
            scheduled_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            estimated_departure: Some("2025-11-16T10:00:00Z".to_string()),
            estimated_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            actual_departure: None,
            actual_arrival: None,
            progress_percent: Some(0),
        };
        
        assert_eq!(calculate_progress(&vm), 0.0);
    }
    
    #[test]
    fn test_calculate_progress_enroute() {
        let vm = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::EnRoute,
            scheduled_departure: Some("2025-11-16T10:00:00Z".to_string()),
            scheduled_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            estimated_departure: Some("2025-11-16T10:00:00Z".to_string()),
            estimated_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            actual_departure: Some("2025-11-16T10:05:00Z".to_string()),
            actual_arrival: None,
            progress_percent: Some(0),
        };
        
        assert_eq!(calculate_progress(&vm), 50.0);
    }
    
    #[test]
    fn test_calculate_progress_completed() {
        let vm = FlightStatusViewModel {
            flight_number: "AA100".to_string(),
            status: FlightStatus::OnTime,
            scheduled_departure: Some("2025-11-16T10:00:00Z".to_string()),
            scheduled_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            estimated_departure: Some("2025-11-16T10:00:00Z".to_string()),
            estimated_arrival: Some("2025-11-16T14:00:00Z".to_string()),
            actual_departure: Some("2025-11-16T10:05:00Z".to_string()),
            actual_arrival: Some("2025-11-16T14:10:00Z".to_string()),
            progress_percent: Some(100),
        };
        
        assert_eq!(calculate_progress(&vm), 100.0);
    }
}
