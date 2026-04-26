use serde::{Deserialize, Serialize};

const SECS_BETWEEN_USES: i64 = 3600 * 3; // 3 hours

#[derive(Debug, Deserialize, Serialize)]
pub struct PokemonCommandRecord {
    pub discord_id: String,
    pub last_used_at: i64,
    pub times_used: i64,
}

impl PokemonCommandRecord {
    pub fn enough_time_since_timestamp(&self, timestamp: i64) -> Result<(), i64> {
        let time_remaining = (timestamp - SECS_BETWEEN_USES) - self.last_used_at;
        if time_remaining < 0 {
            return Err(time_remaining.abs());
        }
        Ok(())
    }
}
