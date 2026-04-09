use sqlx::SqlitePool;
use crate::models::task::Task;

/*
pub async fn add_task(pool: &SqlitePool, title: &str, deadline: chrono::DateTime<chrono::Utc>) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO tasks (title, deadline) VALUES (?, ?)",
        title,
        deadline
    )
    .execute(pool)
    .await?;
    Ok(())
}
*/

pub async fn get_upcoming_tasks(pool: &SqlitePool) -> sqlx::Result<Vec<Task>> {
    let tasks = sqlx::query_as!(
        Task,
        "SELECT id, title, description, deadline, is_completed FROM tasks WHERE is_completed = 0 ORDER BY deadline ASC"
    )
    .fetch_all(pool)
    .await?;
    Ok(tasks)
}

pub async fn insert_schedule(
    pool: &SqlitePool,
    day_of_week: i8,
    period: i32,
    subject: &str,
    room: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO schedules (day_of_week, period, subject, room)
         VALUES (?, ?, ?, ?)",
        day_of_week, // 引数で渡された値（例: 0）を入れる
        period,      // 引数で渡された値（例: 1）を入れる
        subject,     // 引数で渡された値（例: "数学"）を入れる
        room         // 引数で渡された値（例: Some("301教室")）を入れる
    )
    .execute(pool)
    .await?;

    Ok(())
}
