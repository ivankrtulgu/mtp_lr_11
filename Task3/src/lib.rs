use axum::{
    response::Json,
    routing::get,
    Router,
    http::{Request, Response},
    body::Body,
};
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::info;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RootResponse {
    message: String,
    version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct HealthResponse {
    status: String,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        let host = std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        if host.is_empty() {
            return Err("APP_HOST cannot be empty".to_string());
        }

        let port_str = std::env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());
        let port: u16 = port_str.parse().map_err(|e| {
            format!("Invalid APP_PORT '{}': {}", port_str, e)
        })?;

        if port == 0 {
            return Err("APP_PORT cannot be 0".to_string());
        }

        Ok(Self { host, port })
    }

    pub fn socket_addr(&self) -> Result<std::net::SocketAddr, String> {
        format!("{}:{}", self.host, self.port)
            .parse()
            .map_err(|e| format!("Invalid socket address '{}:{}': {}", self.host, self.port, e))
    }
}

pub fn init_logging() {
    use tracing_subscriber::EnvFilter;
    
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .json()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .init();
}

async fn root_handler() -> Json<RootResponse> {
    Json(RootResponse {
        message: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
    })
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    tracing::info_span!(
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                })
                .on_response(|response: &Response<Body>, latency: Duration, _span: &tracing::Span| {
                    info!(
                        status = %response.status(),
                        latency_ms = latency.as_millis(),
                        "response completed"
                    );
                }),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env_defaults() {
        std::env::remove_var("APP_HOST");
        std::env::remove_var("APP_PORT");
        let config = AppConfig::from_env().unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
    }

    #[test]
    fn test_config_socket_addr() {
        let config = AppConfig {
            host: "0.0.0.0".to_string(),
            port: 3000,
        };
        let addr = config.socket_addr().unwrap();
        assert_eq!(addr.ip().to_string(), "0.0.0.0");
        assert_eq!(addr.port(), 3000);
    }

    #[test]
    fn test_config_invalid_port() {
        std::env::set_var("APP_PORT", "invalid");
        let result = AppConfig::from_env();
        assert!(result.is_err());
        std::env::set_var("APP_PORT", "3000");
    }

    #[test]
    fn test_config_port_zero() {
        std::env::set_var("APP_PORT", "0");
        let result = AppConfig::from_env();
        assert!(result.is_err());
        std::env::set_var("APP_PORT", "3000");
    }

    #[test]
    fn test_config_empty_host() {
        std::env::set_var("APP_HOST", "");
        let result = AppConfig::from_env();
        assert!(result.is_err());
        std::env::set_var("APP_HOST", "0.0.0.0");
    }

    #[test]
    fn test_root_response_serialization() {
        let response = RootResponse {
            message: "ok".to_string(),
            version: "0.1.0".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"message\":\"ok\""));
        assert!(json.contains("\"version\":\"0.1.0\""));
    }

    #[test]
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"status\":\"healthy\""));
    }

    #[test]
    fn test_root_response_deserialization() {
        let json = r#"{"message":"ok","version":"0.1.0"}"#;
        let response: RootResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.message, "ok");
        assert_eq!(response.version, "0.1.0");
    }

    #[test]
    fn test_health_response_deserialization() {
        let json = r#"{"status":"healthy"}"#;
        let response: HealthResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, "healthy");
    }
}
