mod entities;
mod query;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::routing::get;
use axum::Router;
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
};
use axum::{http::StatusCode, Json};
use axum::{middleware, Server};
use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use std::env;
use std::future::ready;
use tokio::signal;
use tracer::opentelemetry::trace::TraceContextExt;
use tracer::tracing_opentelemetry::OpenTelemetrySpanExt;
use tracer::{info, span, Instrument, Level};

lazy_static! {
    static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT").map_or(None, |data| Some(
        data.parse().expect("DEPTH_LIMIT is not a number")
    ));
    static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
        .map_or(None, |data| {
            Some(data.parse().expect("COMPLEXITY_LIMIT is not a number"))
        });
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/ws"),
    ))
}

pub async fn graphql_handler(
    Extension(schema): Extension<query::ServiceSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let span = span!(Level::INFO, "graphql_execution");
    info!("Processing GraphQL request");

    let response = async move { schema.execute(req.into_inner()).await }
        .instrument(span.clone())
        .await;
    info!("Processing GraphQL request finished");

    response
        .extension(
            "traceId",
            async_graphql::Value::String(format!(
                "{}",
                span.context().span().span_context().trace_id()
            )),
        )
        .into()
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Serialize)]
struct Health {
    healthy: bool,
}

pub(crate) async fn health() -> impl IntoResponse {
    let health = Health { healthy: true };

    (StatusCode::OK, Json(health))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracer::opentelemetry::global::shutdown_tracer_provider();
}
pub async fn run(port: i32) {
    let connection = sea_orm::Database::connect(std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Could not connect to database");
    let state = AppState { conn: connection };

    let schema = Schema::build(query::Query, EmptyMutation, EmptySubscription)
        // .data(connection)
        .finish();

    let prometheus_recorder = tracer::observability::metrics::create_prometheus_recorder();

    let address = format!("0.0.0.0:{}", port);
    info!("Service starting at address: {}", address);

    let app = Router::new()
        .route("/", get(health))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .route("/metrics", get(move || ready(prometheus_recorder.render())))
        .route_layer(middleware::from_fn(
            tracer::observability::metrics::track_metrics,
        ))
        .layer(Extension(schema))
        .with_state(state);

    Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
