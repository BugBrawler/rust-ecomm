use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cookies;
mod extractors;
mod handlers;
mod products;
mod routers;
mod views;

static DB: once_cell::sync::Lazy<surrealdb::Surreal<surrealdb::engine::remote::ws::Client>> =
    once_cell::sync::Lazy::new(|| surrealdb::Surreal::init());

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_ecomm=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    DB.connect::<surrealdb::engine::remote::ws::Ws>("localhost:8000")
        .await
        .unwrap();
    DB.use_ns("test").use_db("test").await.unwrap();

    let db = surrealdb::Surreal::new::<surrealdb::engine::remote::ws::Ws>("localhost:8000")
        .await
        .unwrap();

    db.use_ns("test").use_db("test").await.unwrap();

    let db = std::sync::Arc::new(db);

    let app = routers::Router::build().with_state(AppState { db });

    tracing::debug!("listening on {}", "0.0.0.0:4321");
    axum::Server::bind(&"0.0.0.0:4321".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type Database = std::sync::Arc<surrealdb::Surreal<surrealdb::engine::remote::ws::Client>>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Database,
}
