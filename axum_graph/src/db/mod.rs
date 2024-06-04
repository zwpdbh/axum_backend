use sqlx::Row;
use std::error::Error;
use tracing::info;

pub mod bookstore;

pub const DB_FOR_DEV: &str = "postgres://postgres:postgres@localhost:5432/myapp";

pub async fn test(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let res = sqlx::query("SELECT 1 + 1 as sum").fetch_one(pool).await?;

    let sum: i32 = res.get("sum");
    info!("1 + 1 = {sum}");

    Ok(())
}
