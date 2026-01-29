use tonic::{Request, Response, Status};
use crate::presentation::grpc::pb::health::{HealthRequest, HealthResponse};
use crate::presentation::grpc::pb::health::health_service_server::HealthService;
#[derive(Default)]
pub struct HealthHandler {}

#[tonic::async_trait]
impl HealthService for HealthHandler {
    async fn check_health(&self, _request: Request<HealthRequest>) -> Result<Response<HealthResponse>, Status> {
        Ok(Response::new(HealthResponse{
            status: "Heartbeat".to_string(),
        }))
    }
}