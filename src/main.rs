use sqlx::sqlite::SqlitePoolOptions;
use dotenvy::dotenv;
use std::env;
use crate::database::db_ops;

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
    sqlx::migrate!("./migrations").run(&pool).await?;

    // 以降、poiseのデータ（Context）にpoolを渡して起動
    Ok(())
}

async fn seed_data(pool: &sqlx::SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 月曜日の1限に数学を入れる
    db_ops::insert_schedule(
        pool,
        0,              // day_of_week: 月曜日
        1,              // period: 1限
        "数学I",         // subject
        Some("301教室".to_string()) // room
    ).await?;

    println!("初期データの投入が完了しました。");
    Ok(())
}
