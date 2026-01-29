use std::net::SocketAddr;
use tonic::Request;
use tokio::time::{sleep, Duration};
use auth_core::presentation::grpc::pb::health::health_service_server::HealthServiceServer;
use auth_core::presentation::grpc::pb::health::health_service_client::HealthServiceClient;
use auth_core::presentation::grpc::health_service_handler::HealthHandler;
use auth_core::presentation::grpc::pb::health::HealthRequest;

#[tokio::test(flavor = "multi_thread")]
async fn test_grpc_connection() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tokio::task::spawn(async move {
        let service = HealthHandler::default();
        tonic::transport::Server::builder()
            .add_service(HealthServiceServer::new(service))
            .serve(addr).await.unwrap();
    });
    sleep(Duration::from_millis(100)).await;
    let mut client = HealthServiceClient::connect(format!("http://{}", addr))
        .await
        .expect("Error creating a new client");
    let request = Request::new(HealthRequest {});
    let response = client.check_health(request).await;
    match response {
        Ok(response) => {
            let inner = response.into_inner();
            assert_eq!(inner.status, "Heartbeat");
        },
        Err(error) => {
            panic!("GRPC server answered with the error: {:?}", error);
        }
    }
}