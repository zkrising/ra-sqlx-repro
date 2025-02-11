use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = SqliteConnectOptions::new()
        .filename("sqlite.db")
        .create_if_missing(true)
        .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Incremental);

    let pool = SqlitePool::connect_with(opts).await?;
    sqlx::migrate!().run(&pool).await?;

    let out = sqlx::query!(
        "
        SELECT
            duck, quack
        FROM
            red_things
        LIMIT
            5
        "
    )
    .fetch_all(&pool)
    .await?;

    dbg!(out);

    Ok(())
}
