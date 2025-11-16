use crate::flight_status::FlightStatusViewModel;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
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
            Constraint::Length(6), // Flight path progress bar (taller for airports + info + path)
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
    
    // Flight Path Progress Bar
    render_flight_path(frame, chunks[3], view_model);
}

fn render_flight_path(frame: &mut Frame, area: ratatui::layout::Rect, view_model: &FlightStatusViewModel) {
    let progress = view_model.progress_percentage();
    
    // Get airport codes, default to "???" if not available
    let origin = view_model.origin_airport.as_deref().unwrap_or("???");
    let destination = view_model.destination_airport.as_deref().unwrap_or("???");
    
    // Calculate available width for the path (subtract borders and padding)
    let available_width = area.width.saturating_sub(4) as usize; // 2 for borders, 2 for padding
    
    // Build the flight path visualization
    let mut lines = vec![];
    
    // Line 1: Airport codes
    let airport_line = format!("{:<width$}{:>width$}", 
        origin, 
        destination,
        width = available_width / 2
    );
    lines.push(Line::from(Span::styled(airport_line, Style::default().fg(Color::White))));
    
    // Line 2: Progress info centered (percent and time remaining)
    let progress_info = build_progress_info(view_model, available_width);
    lines.push(progress_info);
    
    // Line 3: The flight path with airplane
    let path = build_flight_path(available_width, progress);
    lines.push(path);
    
    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Flight Progress"))
        .alignment(Alignment::Left);
    
    frame.render_widget(paragraph, area);
}

fn build_progress_info(view_model: &FlightStatusViewModel, width: usize) -> Line<'static> {
    let progress = view_model.progress_percentage();
    let time_remaining = view_model.time_remaining().unwrap_or_else(|| "N/A".to_string());
    
    let info_text = format!("{:.0}% • {}", progress, time_remaining);
    let padding = (width.saturating_sub(info_text.len())) / 2;
    
    let centered_text = format!("{:padding$}{}", "", info_text, padding = padding);
    
    Line::from(Span::styled(centered_text, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
}

fn build_flight_path(width: usize, progress: f64) -> Line<'static> {
    if width < 10 {
        return Line::from("");
    }
    
    // Calculate airplane position (0-100% maps to start-end of path)
    let progress_clamped = progress.clamp(0.0, 100.0);
    let path_width = width.saturating_sub(2); // Leave room for dots at each end
    let airplane_pos = ((path_width as f64 * progress_clamped / 100.0).round() as usize).min(path_width.saturating_sub(1));
    
    let mut spans = vec![];
    
    // Origin dot
    spans.push(Span::styled("●", Style::default().fg(Color::White)));
    
    // Build the path
    for i in 0..path_width {
        if i == airplane_pos {
            // Airplane emoji or character
            spans.push(Span::styled("✈", Style::default().fg(Color::Cyan)));
        } else if i < airplane_pos {
            // Trail behind the airplane
            spans.push(Span::styled("─", Style::default().fg(Color::Yellow)));
        } else {
            // Empty path ahead
            spans.push(Span::styled("─", Style::default().fg(Color::DarkGray)));
        }
    }
    
    // Destination dot
    spans.push(Span::styled("●", Style::default().fg(Color::White)));
    
    Line::from(spans)
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
            origin_airport: None,
            destination_airport: None,
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
            origin_airport: None,
            destination_airport: None,
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
            origin_airport: None,
            destination_airport: None,
        };
        
        assert_eq!(calculate_progress(&vm), 100.0);
    }
}
