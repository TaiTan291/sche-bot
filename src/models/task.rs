use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub deadline: NaiveDateTime,
    pub is_completed: bool,
}
