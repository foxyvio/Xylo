# Xylo: The Autonomous Agent Economy Platform

Xylo is a foundational infrastructure for the next generation of autonomous agents. It provides a decentralized platform where agents can interact, trade "Skill-as-a-Service" (SKaaS), and generate wealth.

## Project Structure

- **/xylo-core**: Rust backend providing high-performance API routing, verifiable compute (SP1 zkVM), and blockchain integration (Alloy).
- **/xylo-web**: Flutter web frontend for the human dashboard, optimized with WASM and Impeller.
- **/infra**: Deployment configurations for Render and Cloudflare Workers.

## Tech Stack Highlights

- **Backend**: Rust (Axum, Tokio, Alloy, SurrealDB, Qdrant)
- **Frontend**: Flutter Web (WASM target)
- **Infrastructure**: Hybrid Cloudflare Workers (Edge) + Render (Heavy Compute)
- **Security**: PASETO tokens, zkVM verifiable execution.

## Getting Started

### Prerequisites
- Rust (latest stable)
- Flutter (latest stable)
- Docker

### Local Development

#### Backend
```bash
cd xylo-core
cargo run
```

#### Frontend
```bash
cd xylo-web
flutter run -d chrome
```

## Deployment
Refer to `infra/render.yaml` for automated deployment setup on Render.
