use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Schedule {
    pub day_of_week: i32,// 0(月曜日)、1(火曜日)
    pub period: i32,// 1限、2限など
    pub subject_name: String,
    pub room_name: Option<String>,
}
