use crate::command_line::Arguments;
use crate::command_line::BookstoreEx;
use crate::command_line::SubCommand;
use crate::observability::metrics::{create_prometheus_recorder, track_metrics};
use crate::observability::tracing::setup_tracer;
use crate::query::Query;
use crate::routes::graphql::{graphql_handler, graphql_playground};
use crate::routes::health;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::middleware;
use axum::{extract::Extension, routing::get, Router, Server};
use clap::Parser;
use command_line::DieselDemo;
use command_line::SqlCase;
use dotenv::dotenv;
use std::future::ready;
use tokio::signal;
use tracing::info;

mod command_line;
mod db;
mod observability;
mod query;
mod routes;

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

#[tokio::main]
async fn main() {
    let _ = dotenv().ok();
    let _ = setup_tracer();

    let args = Arguments::parse();
    match args.cmd {
        SubCommand::AxumSqlx { port } => {
            let conn = sqlx::postgres::PgPool::connect(db::DB_FOR_DEV)
                .await
                .unwrap();

            let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
                .data(conn)
                .finish();
            let prometheus_recorder = create_prometheus_recorder();

            let address = format!("0.0.0.0:{}", port);
            info!("Service starting at address: {}", address);

            let app = Router::new()
                .route("/", get(health))
                .route("/graphql", get(graphql_playground).post(graphql_handler))
                .route("/metrics", get(move || ready(prometheus_recorder.render())))
                .route_layer(middleware::from_fn(track_metrics))
                .layer(Extension(schema));

            Server::bind(&address.parse().unwrap())
                .serve(app.into_make_service())
                .with_graceful_shutdown(shutdown_signal())
                .await
                .unwrap();
        }
        SubCommand::Sqlx { case } => {
            let conn = sqlx::postgres::PgPool::connect(db::DB_FOR_DEV)
                .await
                .unwrap();

            match case {
                SqlCase::Test => {
                    db::test(&conn).await.unwrap();
                }
                SqlCase::Bookstore { example } => match example {
                    BookstoreEx::Create => {
                        let _ = db::bookstore::create_book_example(&conn).await.unwrap();
                    }
                    BookstoreEx::Update => {
                        let _ = db::bookstore::update_book_example(&conn).await.unwrap();
                    }
                    BookstoreEx::Read { v } => {
                        let _ = db::bookstore::read_book_example(&conn, v).await.unwrap();
                    }
                },
            }
        }
        SubCommand::DieselDemo { demo } => match demo {
            DieselDemo::ShowPost => diesel_demo::show_post(),
            DieselDemo::CreatePost { title, body } => diesel_demo::create_post(&title, &body),
            DieselDemo::UpdatePost { id } => diesel_demo::update_post(id),
            DieselDemo::SelectPost { id } => diesel_demo::select_post(id),
            DieselDemo::DeletePost { target } => diesel_demo::delete_post(&target),
        },
        _ => todo!("not implemented"),
    }
}
