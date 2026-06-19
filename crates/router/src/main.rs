mod routes;
mod types;
mod error;

use actix_web::{App, HttpServer, middleware::Logger, web};
use sqlx_postgres::PostgresDb;
use crate::types::app::AppState;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let host = "127.0.0.0";
    let port = 8080;
    
    println!("Starting server at PORT: {port}");
    
    let app_state = web::Data::new(AppState{
        pool: PostgresDb::new().await.unwrap().get_pg_connection().unwrap().clone(),
        jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::configure)
            .wrap(Logger::default())
    })
    .bind((host, port))?
    .run()
    .await
}
