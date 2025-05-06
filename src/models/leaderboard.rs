use serde::Deserialize;
use std::fmt;
use std::fmt::Formatter;

pub enum LeaderboardType {
    Positive,
    Negative,
    Sum,
    Total,
}

impl fmt::Display for LeaderboardType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let thing = match self {
            LeaderboardType::Positive => "Positive",
            LeaderboardType::Negative => "Negative",
            LeaderboardType::Sum => "Sum",
            LeaderboardType::Total => "Total",
        };
        write!(f, "{}", thing)
    }
}

#[derive(Deserialize)]
pub struct LeaderboardEntry {
    pub username: String,
    pub vote_count: i64,
}
