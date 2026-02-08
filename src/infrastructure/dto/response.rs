use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub messages: Vec<String>,
    pub data: T,
}
