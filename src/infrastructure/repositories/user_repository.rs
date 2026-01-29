use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder};
use tonic::async_trait;
use uuid::Uuid;
use crate::infrastructure::entities::users;
use crate::infrastructure::entities::prelude::Users;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn insert(&self, user: users::ActiveModel) -> Result<(), DbErr>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<users::Model>, DbErr>;
    async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr>;
    async fn find_by_username(&self, username: &str) -> Result<Option<users::Model>, DbErr>;
    async fn find_all(&self) -> Result<Vec<users::Model>, DbErr>;
    async fn update(&self, user: users::ActiveModel) -> Result<(), DbErr>;
    async fn delete(&self, id: Uuid) -> Result<(), DbErr>;
}

pub struct UserRepImpl{
    db: DatabaseConnection
}

impl UserRepImpl{
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepImpl {
    async fn insert(&self, user: users::ActiveModel) -> Result<(), DbErr> {
        Users::insert(user).exec_without_returning(&self.db).await?;
        Result::Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<users::Model>, DbErr> {
        Users::find_by_id(id).one(&self.db).await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr> {
        Users::find().filter(users::Column::Email.eq(email)).one(&self.db).await
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<users::Model>, DbErr> {
        Users::find().filter(users::Column::Username.eq(username)).one(&self.db).await
    }

    async fn find_all(&self) -> Result<Vec<users::Model>, DbErr> {
        Users::find().order_by_asc(users::Column::CreatedAt).all(&self.db).await
    }

    async fn update(&self, user: users::ActiveModel) -> Result<(), DbErr> {
        Users::update(user).exec(&self.db).await?;
        Result::Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DbErr> {
        Users::delete_by_id(id).exec(&self.db).await?;
        Result::Ok(())
    }
}