use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = SqliteConnectOptions::new()
        .filename("sqlite.db")
        .create_if_missing(true)
        .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Incremental);

    let pool = SqlitePool::connect_with(opts).await?;
    sqlx::migrate!().run(&pool).await?;

    let output = sqlx::query!(
        "
        SELECT
            foo, bar
        FROM
            yellow_things
        LIMIT
            5
        "
    )
    .fetch_all(&pool)
    .await?;

    dbg!(output);

    Ok(())
}
