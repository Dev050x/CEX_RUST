use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub struct PostgresDb {
    pool: Pool<Postgres>,
}

impl PostgresDb {
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        println!("DB connected");

        sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS users (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
                    username TEXT NOT NULL UNIQUE,
                    password TEXT NOT NULL,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
                    updated_at TIMESTAMPTZ NOT NULL DEFAULT now ()
                );
            "#,
        )
        .execute(&pool)
        .await?;

        return Ok(Self { pool: pool });
    }
    pub fn get_pg_connection(&self) -> Result<Pool<Postgres>, sqlx::Error> {
        Ok(self.pool.clone())
    }
}
