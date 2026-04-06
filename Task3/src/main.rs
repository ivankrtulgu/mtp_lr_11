use app::{create_router, init_logging, AppConfig};
use tokio::signal;
use tracing::{error, info};

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received SIGINT");
        }
        _ = terminate => {
            info!("Received SIGTERM");
        }
    }
}

#[tokio::main]
async fn main() {
    init_logging();

    let config = match AppConfig::from_env() {
        Ok(c) => c,
        Err(e) => {
            error!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };

    let addr = match config.socket_addr() {
        Ok(a) => a,
        Err(e) => {
            error!("Address parse error: {}", e);
            std::process::exit(1);
        }
    };

    let app = create_router();

    info!("Starting server on {}", addr);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            error!("Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    };

    let server = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal());

    info!("Server is ready, waiting for shutdown signal");

    if let Err(e) = server.await {
        error!("Server error: {}", e);
        std::process::exit(1);
    }

    info!("Server shut down gracefully");
}
