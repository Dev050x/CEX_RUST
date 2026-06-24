use sqlx::{Pool, Postgres};

pub struct AppState {
    pub pool: Pool<Postgres>,
    pub jwt_secret: String,
}
