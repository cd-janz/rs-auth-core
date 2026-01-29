use std::sync::Arc;
use crate::infrastructure::configuration::database::SQLDatabase;
use crate::infrastructure::repositories::user_repository::{UserRepImpl, UserRepository};

#[derive(Clone)]
pub struct InfraState{
    pub user_repo: Arc<dyn UserRepository>
}

impl InfraState{
    pub async fn new() -> Self{
        let db = SQLDatabase::new().await;
        Self {
            user_repo: Arc::new(UserRepImpl::new(db))
        }
    }
}