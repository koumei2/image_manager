use std::str::FromStr;

pub async fn init() -> anyhow::Result<()> {
    let db_filename = super::CONFIG
        .get()
        .unwrap()
        .get_string("DATABASE_FILE")
        .unwrap();
    let connection_options = sqlx::sqlite::SqliteConnectOptions::from_str(&db_filename).unwrap();
    //        .create_if_missing(true)
    //        .journal_mode(SqliteJournalMode::Wal)
    //        .synchronous(SqliteSynchronous::Normal)
    //        .busy_timeout(pool_timeout); // default 5sec
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        //        .connect_timeout(pool_timeout)
        .connect_with(connection_options)
        .await?;
    let _ = super::DB.set(pool);
    Ok(())
}

pub fn get_pool() -> &'static sqlx::Pool<sqlx::sqlite::Sqlite> {
    super::DB.get().unwrap()
}
