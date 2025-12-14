use std::sync::Arc;

use crate::prometheus_exp;
use crate::{
    cli::CliArgs,
    typesense::{metrics::get_typesense_metrics, stats::get_typesense_stats},
};

use axum::extract::State;
use axum::{routing::get, Router};
use futures::future;
use futures::future::join_all;
use tokio::signal;

pub(crate) async fn start_metrics_server(args: CliArgs) {
    tracing_subscriber::fmt::init();

    let shared_cli_args: Arc<CliArgs> = Arc::new(args.clone());

    let app = Router::new()
        .route("/", get(root))
        .route("/metrics", get(metrics_route_handler))
        .with_state(shared_cli_args);

    let bind_address = format!("{}:{}", args.exporter_bind_address, args.exporter_bind_port);
    println!("Starting exporter server at {:?}", bind_address);
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to run Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to run signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn metrics_route_handler(State(args): State<Arc<CliArgs>>) -> String {
    let client = reqwest::Client::new();
    let targets = args.typesense_targets();

    let scrapes = join_all(targets.into_iter().map(|target| {
        let args = args.clone();
        let client = client.clone();
        async move {
            let (metrics_res, stats_res) = future::join(
                get_typesense_metrics(&client, args.as_ref(), &target),
                get_typesense_stats(&client, args.as_ref(), &target),
            )
            .await;

            prometheus_exp::TargetScrape {
                host: target.host,
                port: target.port,
                metrics: metrics_res.ok(),
                stats: stats_res.ok(),
            }
        }
    }))
    .await;

    prometheus_exp::generate_metrics(scrapes)
}
