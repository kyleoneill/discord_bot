use super::Database;

use crate::models::leaderboard::{LeaderboardEntry, LeaderboardType};

use sqlx::{Error, SqlitePool};

impl Database {
    pub async fn get_leaderboard(
        pool: &SqlitePool,
        leaderboard_type: &LeaderboardType,
    ) -> Result<Vec<LeaderboardEntry>, Error> {
        match leaderboard_type {
            LeaderboardType::Positive => {
                sqlx::query_as!(
                    LeaderboardEntry,
                    r#"
                    SELECT username, positive_credit as vote_count FROM social_credit
                    ORDER BY positive_credit DESC
                    LIMIT 5
                    "#
                )
                .fetch_all(pool)
                .await
            }
            LeaderboardType::Negative => {
                sqlx::query_as!(
                    LeaderboardEntry,
                    r#"
                    SELECT username, negative_credit as vote_count FROM social_credit
                    ORDER BY negative_credit DESC
                    LIMIT 5
                    "#
                )
                .fetch_all(pool)
                .await
            }
            LeaderboardType::Total => {
                sqlx::query_as!(
                    LeaderboardEntry,
                    r#"
                        SELECT username, (positive_credit + negative_credit) AS vote_count
                        FROM social_credit
                        ORDER BY vote_count DESC
                        LIMIT 5
                    "#
                )
                .fetch_all(pool)
                .await
            }
            LeaderboardType::Sum => {
                sqlx::query_as!(
                    LeaderboardEntry,
                    r#"
                        SELECT username, (positive_credit - negative_credit) AS vote_count
                        FROM social_credit
                        ORDER BY vote_count DESC
                        LIMIT 5
                    "#
                )
                .fetch_all(pool)
                .await
            }
        }
    }
}
