use sqlx::SqlitePool;
//use crate::models::task::Task;
use crate::models::schedule::Schedule;

/*
pub async fn get_upcoming_tasks(pool: &SqlitePool) -> sqlx::Result<Vec<Task>> {
    let tasks = sqlx::query_as!(
        Task,
        "SELECT id, title, description, deadline, is_completed FROM tasks WHERE is_completed = 0 ORDER BY deadline ASC"
    )
    .fetch_all(pool)
    .await?;
    Ok(tasks)
}
*/

pub async fn insert_schedule(
    pool: &SqlitePool,
    schedule: Schedule,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO schedules (day_of_week, period, subject_name, room_name)
         VALUES (?, ?, ?, ?)",
        schedule.day_of_week,
        schedule.period,
        schedule.subject_name,
        schedule.room_name
    )
    .execute(pool)
    .await?;

    println!("successefull input");

    Ok(())
}
