use sqlx::sqlite::SqlitePoolOptions;
use dotenvy::dotenv;
use std::env;
use crate::database::db_ops::insert_schedule;
use crate::models::schedule::Schedule;

//pub mod commands;
pub mod database;
pub mod models;
//pub mod scheduler;
//pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;


    // DB接続プールの作成
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // マイグレーションの実行
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    insert_schedule(&pool, Schedule {
        day_of_week: 0,
        period: 1,
        subject_name: "数学".to_string(),
        room_name: Some("301教室".to_string()),
    })
        .await?;

    // 以降、poiseのデータ（Context）にpoolを渡して起動
    Ok(())
}
