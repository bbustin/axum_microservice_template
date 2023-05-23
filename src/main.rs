mod tasks;
mod errors; //makes custom errors available to modules

use std::{env, net::SocketAddr};

use axum::{routing::{get, post, put, delete}, Router, Extension, response::Redirect};
use sqlx::{Sqlite, migrate::MigrateDatabase, Pool};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // initialize tracing
    // Todo: make level configurable in .env?
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // get variables from .env file
    match dotenvy::dotenv() {
        Ok(path) => tracing::debug!(".env read successfully from {}", path.display()),
        Err(error) => panic!("Could not load .env file: {}", error),
    };

    let pool: Pool<Sqlite> = database_setup().await.expect("Could not create database pool");

    // create the app with routes
    let app = Router::new()
    .route("/", get(|| async {Redirect::permanent("/tasks")}))
    .route("/tasks", get(tasks::controller::all_tasks))
    .route("/tasks", post(tasks::controller::new_task))
    .route("/tasks/:id",get(tasks::controller::task))
    .route("/tasks/:id", put(tasks::controller::update_task))
    .route("/tasks/:id", delete(tasks::controller::delete_task))
    .layer(Extension(pool))
    .layer(TraceLayer::new_for_http());

    // run app
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); //todo: make IP and port configurable
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// hopefully will make it easier to switch database types
type GenericPool = Pool<Sqlite>;

//// Database operations
/// - creates SqlLite database if missing
/// - runs sqlx migrations
/// - creates a connection pool
/// 
/// Returns database connection pool
async fn database_setup() 
    -> anyhow::Result<GenericPool> {
    let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL not set");
    tracing::debug!("Using database URL {}", database_url);
    create_database_if_missing(database_url).await;
    let pool = create_database_pool(database_url).await
        .expect("unable to create database pool");
    // run migrations
    sqlx::migrate!().run(&pool).await.unwrap();
    Ok(pool)
}

async fn create_database_if_missing(database_url: &str) {
    if !database_url.starts_with("sqlite://") {
        tracing::debug!("Not using Sqlie database. Skipping automatic database creation");
        return;
    }

    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        tracing::info!("Creating database {}", &database_url);
        match Sqlite::create_database(database_url).await {
            Ok(_) => tracing::info!("Database created successfully"),
            Err(error) => tracing::error!("Error creating database: {}", error)
        }
    } else {
        tracing::debug!("Database already exists");
    }
}

async fn create_database_pool(database_url: &str) 
    -> anyhow::Result<GenericPool> {
    Ok(Pool::connect(database_url).await?)
}