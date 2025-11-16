pub mod flight_status;
pub mod api_converter;

pub use flight_status::{FlightStatus, FlightStatusViewModel};
pub use api_converter::determine_flight_status;
