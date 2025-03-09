mod routes;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use sqlx::{pool, postgres::PgPoolOptions, Pool, Postgres};
use routes::health_route::health_checker_handler;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }

    dotenv().ok();
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("Database url must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("DB connection successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to DB");
            std::process::exit(1);
        }
    };
    
    println!("Server started...");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(actix_web::web::Data::new(AppState { db: pool.clone() }))
            .service(health_checker_handler)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await

}
