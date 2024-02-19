use sqlx::postgres::PgPoolOptions;

pub async fn connect(db_url: &str) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    match {
        let pool = &pool;
        async move {
            let mut conn = pool.acquire().await?;
            sqlx::migrate!("./migrations").run(&mut *conn).await?;
            Ok(())
        }
    }
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
