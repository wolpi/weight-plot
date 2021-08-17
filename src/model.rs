use chrono::NaiveDateTime;

pub const TIMESTAMP_FORMAT :&str = "%Y-%m-%d %H:%M:%S";

#[derive(Clone)]
pub struct WeightLine {
    pub timestamp :NaiveDateTime,
    pub weight :f32,
}

impl WeightLine {
    pub fn new() -> WeightLine {
        WeightLine {
            timestamp: NaiveDateTime::parse_from_str("1970-01-01 00:00:00", TIMESTAMP_FORMAT).unwrap(),
            weight: 0.0,
        }
    }
}

pub enum PlotType {
    FULL,
    YEAR,
    MONTH
}
