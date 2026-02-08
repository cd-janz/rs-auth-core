pub mod infrastructure;
pub mod presentation;
use crate::infrastructure::infra_state::InfraState;
use crate::presentation::grpc::pb::user::user_service_server::UserServiceServer;
use crate::presentation::grpc::user_service_handler::UserHandler;
use axum::{Router, routing::get};
use std::net::SocketAddr;
use tonic::transport::Server;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(hello))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // STATES
    let infra = InfraState::new().await;
    // AXUM REST Config
    let rest_addr: SocketAddr = "0.0.0.0:8080".parse().expect("couldn't parse rest address");
    let (_, api_spec) = OpenApiRouter::<InfraState>::with_openapi(ApiDoc::openapi())
        .routes(routes!(hello))
        .split_for_parts();
    let api = Router::new()
        .route("/api/v1", get(hello))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_spec))
        .with_state(infra);
    let api_ltr = tokio::net::TcpListener::bind(rest_addr)
        .await
        .expect("couldn't start axum tcp server");
    // Axum-Tonic GRPC Config
    let grpc_addr: SocketAddr = "0.0.0.0:50051"
        .parse()
        .expect("couldn't parse grpc address");
    let user_handler = UserHandler::default();
    let grpc_service = UserServiceServer::new(user_handler);
    tokio::join!(
        async {
            axum::serve(api_ltr, api).await.unwrap();
        },
        async {
            Server::builder()
                .add_service(grpc_service)
                .serve(grpc_addr)
                .await
                .expect("GRPC server failed");
        }
    );
}

#[utoipa::path(get, path="/api/v1", responses((status = 200, description = "Hello World", body = &str)))]
async fn hello() -> &'static str {
    "Hello, World!"
}
