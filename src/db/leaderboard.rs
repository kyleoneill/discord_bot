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
                    SELECT users.username, social_credit.positive_credit as vote_count
                    FROM social_credit
                    INNER JOIN users
                    ON social_credit.discord_id = users.discord_id
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
                    SELECT users.username, social_credit.negative_credit as vote_count
                    FROM social_credit
                    INNER JOIN users
                    ON social_credit.discord_id = users.discord_id
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
                        SELECT users.username, (social_credit.positive_credit + social_credit.negative_credit) AS vote_count
                        FROM social_credit
                        INNER JOIN users
                        ON social_credit.discord_id = users.discord_id
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
                        SELECT users.username, (social_credit.positive_credit - social_credit.negative_credit) AS vote_count
                        FROM social_credit
                        INNER JOIN users
                        ON social_credit.discord_id = users.discord_id
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
