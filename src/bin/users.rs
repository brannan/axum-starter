use futures::TryStreamExt;
use sqlx::postgres::PgPoolOptions;

use backend::models::user::User;
use backend::utils;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let env = utils::read_env().expect("Unable to read .env file");
    let database_url = env.get("DATABASE_URL").expect("DATABASE_URL not found in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    let null = "n/a".to_string();
    let sql = "SELECT * FROM users ORDER BY email";
    let mut rows = sqlx::query_as::<_, User>(&sql).fetch(&pool);
    while let Some(row) = rows.try_next().await? {
        println!(
            "{}... {} {} {}",
            row.id,
            row.email,
            row.first_name.unwrap_or(null.clone()),
            row.last_name.unwrap_or(null.clone())
        );
    }
    Ok(())
}
