pub mod presentation;
pub mod infrastructure;
use std::net::SocketAddr;
use axum::{
    routing::{get, },
    Router,
};
use tonic::transport::Server;
use crate::infrastructure::infra_state::InfraState;
use crate::presentation::grpc::pb::user::user_service_server::UserServiceServer;
use crate::presentation::grpc::user_service_handler::UserHandler;

#[tokio::main]
async fn main() {
    // STATES
    let _infra = InfraState::new().await;
    // AXUM REST Config
    let rest_addr: SocketAddr = "0.0.0.0:8080".parse().expect("couldn't parse rest address");
    let api = Router::new().route("/api/v1", get(hello));
    let api_ltr = tokio::net::TcpListener::bind(rest_addr)
        .await.expect("couldn't start axum tcp server");
    // Axum-Tonic GRPC Config
    let grpc_addr: SocketAddr = "0.0.0.0:50051"
        .parse()
        .expect("couldn't parse grpc address");
    let user_handler = UserHandler::default();
    let grpc_service = UserServiceServer::new(user_handler);
    tokio::join!(
        async { axum::serve(api_ltr, api).await.unwrap(); },
        async { Server::builder().add_service(grpc_service).serve(grpc_addr).await.expect("GRPC server failed"); }
    );
}

async fn hello() -> &'static str {
    "Hello, World!"
}