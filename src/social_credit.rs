use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SocialCredit {
    pub username: String,
    pub positive_credit: i64,
    pub negative_credit: i64,
    pub traded_credit: i64,
}

impl SocialCredit {
    pub fn sum_score(&self) -> usize {
        ((self.positive_credit + self.traded_credit) - self.negative_credit) as usize
    }
}
