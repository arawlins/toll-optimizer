use axum::{
    Router, middleware,
    routing::{get, post},
};
use axum_prometheus::PrometheusMetricLayer;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder, key_extractor::KeyExtractor, GovernorError};
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

mod auth;
mod db;
mod handlers;
mod models;

#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalKeyExtractor;

impl KeyExtractor for GlobalKeyExtractor {
    type Key = ();
    fn extract<B>(&self, _req: &axum::http::Request<B>) -> Result<Self::Key, GovernorError> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable is missing. This is required to connect to PostgreSQL.");
    let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<u32>()
        .unwrap_or(5);

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&db_url)
        .await
        .expect("Failed to connect to PostgreSQL. Please check your connection string and ensure the database is running.");

    // Run database migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations. Please check if the user has enough privileges or if the database is accessible.");

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let frontend_url =
        env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(vec![
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers(vec![
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
        ]);

    let governor_conf = std::sync::Arc::new(
        GovernorConfigBuilder::default()
            .key_extractor(GlobalKeyExtractor)
            .per_second(2)
            .burst_size(5)
            .finish()
            .unwrap(),
    );

    let mut auth_routes = Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login));

    if env::var("DISABLE_RATE_LIMIT").unwrap_or_default() != "true" {
        auth_routes = auth_routes.layer(GovernorLayer {
            config: governor_conf,
        });
    }

    let mut pricing_routes = Router::new()
        .route("/", post(handlers::pricing::get_current_and_next_prices));

    let pricing_governor_conf = std::sync::Arc::new(
        GovernorConfigBuilder::default()
            .per_second(20)
            .burst_size(20)
            .finish()
            .unwrap(),
    );

    if env::var("DISABLE_RATE_LIMIT").unwrap_or_default() != "true" {
        pricing_routes = pricing_routes.layer(GovernorLayer {
            config: pricing_governor_conf,
        });
    }

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .layer(axum::extract::DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB limit
        .nest("/auth", auth_routes)
        .route("/api/analyze", post(handlers::analyze::analyze))
        .nest("/api/pricing", pricing_routes)
        .route("/api/history", get(handlers::analyze::history))
        .route(
            "/metrics",
            get(|| async move { metric_handle.render() })
                .route_layer(middleware::from_fn(auth::basic_auth)),
        )
        .fallback_service(ServeDir::new("dist").fallback(ServeFile::new("dist/index.html")))
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer)
        .layer(cors)
        .with_state(pool);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
