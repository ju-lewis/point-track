
use serde::Serialize;

#[derive(Serialize)]
pub struct RaceResult {
    pub comp_no: u32,
    pub boat_name: String,
    pub skipper: String,
    pub navigator: String,
    pub nominated_speed: u32,
    pub penalty: f64,
    pub points_lost: f64, // Note: this may actually be a positive integer (u32)
    pub total_points: f64,
    pub notes: String
}

impl RaceResult {
    pub fn test() -> RaceResult {
        // Creates a struct with test data
        return RaceResult {
            comp_no: 203,
            boat_name: "Test".to_string(),
            skipper: "Skipper".to_string(),
            navigator: "Navigator".to_string(),
            nominated_speed: 8,
            penalty: 3.2,
            points_lost: 4.5,
            total_points: 23.2,
            notes: "Test note".to_string()
        }
    }
}

