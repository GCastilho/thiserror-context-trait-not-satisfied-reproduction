use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;
use thiserror::Error;
use thiserror_context::{impl_context, Context};

#[derive(Debug, Error)]
enum InnerA {
    #[error("InnerA Sqlx")]
    Sqlx(#[from] sqlx::Error),
}

impl_context!(OuterA(InnerA));

// Uncoment these lines for error to occur
// #[derive(Debug, Error)]
// enum InnerB {
//     #[error("InnerB Sqlx")]
//     Sqlx(#[from] sqlx::Error),
// }

// impl_context!(OuterB(InnerB));

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let database_url = env::var("DATABASE_URL").expect("env DATABASE_URL must be present");
    let mut pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("connect");

    outer_a(&mut pool).await.expect("outer_a");
}

async fn outer_a(pool: &mut Pool<Sqlite>) -> Result<(), OuterA> {
    let mut tx = pool.begin().await?;
    sqlx::query!("CREATE TABLE IF NOT EXISTS some_table (id INTEGER PRIMARY KEY AUTOINCREMENT)")
        .execute(&mut *tx)
        .await
        .context("outer_a")?;

    Ok(())
}
