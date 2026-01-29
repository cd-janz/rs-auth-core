use tonic::{ Response, Request, Status };

use crate::presentation::grpc::pb::user::user_service_server::UserService;
use crate::presentation::grpc::pb::commons::{BasicResponseUser, CreateUser};

#[derive(Default)]
pub struct UserHandler {}

#[tonic::async_trait]
impl UserService for UserHandler {
    async fn create_user(&self, _request: Request<CreateUser>) -> Result<Response<BasicResponseUser>, Status> {
        todo!()
    }
}