use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
    extract::State,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use serde::Serialize;
use dotenvy::dotenv;
use std::env;

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
    version: String,
}

#[derive(Serialize)]
struct Agent {
    id: String,
    name: String,
    skill: String,
}

#[derive(Clone)]
struct AppState {
    agents: Arc<RwLock<Vec<Agent>>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "xylo_core=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState {
        agents: Arc::new(RwLock::new(vec![
            Agent {
                id: "1".to_string(),
                name: "Data Summarizer".to_string(),
                skill: "Text Summarization".to_string(),
            },
        ])),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/agents", get(list_agents))
        .route("/api/v1/agents/negotiate", post(negotiate_agent))
        .with_state(state)
        .layer(cors);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse::<u16>().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Xylo Core listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn negotiate_agent(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Placeholder for P2P negotiation logic using libp2p and SP1 zkVM proofs
    tracing::info!("Negotiation requested: {:?}", payload);
    (StatusCode::ACCEPTED, Json(serde_json::json!({
        "status": "negotiating",
        "proof_requested": true,
        "zk_vm": "SP1"
    })))
}

async fn health_check() -> (StatusCode, Json<HealthCheckResponse>) {
    let response = HealthCheckResponse {
        status: "UP".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    (StatusCode::OK, Json(response))
}

async fn list_agents() -> (StatusCode, Json<Vec<Agent>>) {
    let agents = vec![
        Agent {
            id: "1".to_string(),
            name: "Data Summarizer".to_string(),
            skill: "Text Summarization".to_string(),
        },
        Agent {
            id: "2".to_string(),
            name: "Market Analyst".to_string(),
            skill: "Financial Analysis".to_string(),
        },
    ];
    (StatusCode::OK, Json(agents))
}
