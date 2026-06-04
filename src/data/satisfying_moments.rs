use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug, sqlx::FromRow, Clone)]
pub struct SatisfyingMoments {
    pub id: i64,
    pub user_id: i64,
    pub description: String,
    pub thoughts: Option<String>,
    pub why_it_matters: Option<String>,
    pub values_alignment: Option<String>,
    pub lived_at: Option<NaiveDate>,
}
