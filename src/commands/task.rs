#[poise::command(slash_command)]
pub async fn add_task(
    ctx: Context<'_>,
    #[description = "課題名"] title: String,
    #[description = "詳細"] description: Option<String>,
) -> Result<(), Error> {
    let pool = &ctx.data().database; // mainから渡されたDB接続
    let deadline = chrono::Utc::now(); // 本来は引数で日付を受け取ってパースする

    // db_opsの関数を呼び出して保存
    crate::database::db_ops::insert_task(
        pool,
        &title,
        description,
        deadline
    ).await?;

    ctx.say(format!("✅ 課題『{}』を登録しました！", title)).await?;
    Ok(())
}
