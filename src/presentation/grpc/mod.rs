pub mod user_service_handler;
pub mod health_service_handler;

pub mod pb {
    pub mod user {
        tonic::include_proto!("user_service");
    }
    pub mod health {
        tonic::include_proto!("health_service");
    }
    pub mod commons {
        tonic::include_proto!("commons");
    }
}
