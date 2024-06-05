// use crate::observability::metrics::{create_prometheus_recorder, track_metrics};
// use crate::observability::tracing::setup_tracer;
use crate::query::ServiceSchema;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::EmptyMutation;
use async_graphql::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use async_graphql::EmptySubscription;
use axum::middleware;
use axum::routing::get;
use axum::Router;
use axum::Server;
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
};
use axum::{http::StatusCode, Json};
use opentelemetry::trace::TraceContextExt;
use serde::Serialize;
use std::future::ready;
use tokio::signal;
use tracing::{info, span, Instrument, Level};
use tracing_opentelemetry::OpenTelemetrySpanExt;

mod query;

pub const DB_FOR_DEV: &str = "postgres://postgres:postgres@localhost:5432/myapp";

pub(crate) async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/ws"),
    ))
}

pub(crate) async fn graphql_handler(
    Extension(schema): Extension<ServiceSchema>,
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

#[derive(Serialize)]
struct Health {
    healthy: bool,
}

pub(crate) async fn health() -> impl IntoResponse {
    let health = Health { healthy: true };

    (StatusCode::OK, Json(health))
}

async fn shutdown_signal() {
    // (1)
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

    opentelemetry::global::shutdown_tracer_provider();
}

pub async fn run(port: &str) {
    let conn = sqlx::postgres::PgPool::connect(DB_FOR_DEV).await.unwrap();

    let schema = Schema::build(query::Query, EmptyMutation, EmptySubscription)
        .data(conn)
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
        .layer(Extension(schema));

    Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
