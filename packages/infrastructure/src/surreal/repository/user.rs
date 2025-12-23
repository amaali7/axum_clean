use async_trait::async_trait;
use surrealdb::sql::Thing;

use application::{error::{AppResult, ApplicationError}, ports::UserRepository};
use domain::user::{User, UserId};

use crate::{error::InfrastructureError, serialization::user::user::SerializedUser, surreal::client::SurrealDBClient};

pub struct SurrealUserRepository {
    client: SurrealDBClient,
}

impl SurrealUserRepository {
    pub fn new(client: SurrealDBClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl UserRepository for SurrealUserRepository {
    async fn save(&self, user: User) -> AppResult<()>{
        let record: SerializedUser = user.into();

        let thing = Thing::from(("user", record.id().clone()));
        self.client.db.update(thing).content(record).await?;
        Ok(())
    }
    async fn update(&self, user: User) -> AppResult<()>{Ok(())}
    async fn get_by_id(&self, id: UserId) -> AppResult<Option<User>>;
    async fn delete(&self, id: UserId) -> AppResult<()>;
    async fn get_by_email(&self, id: Email) -> AppResult<Option<User>>;
    async fn get_by_username(&self, id: Username) -> AppResult<Option<User>>;
    async fn get_events(&self, user_id: UserId) -> AppResult<Option<Vec<UserEvent>>>;
    async fn list(&self) -> AppResult<Vec<User>>;
    async fn assign_permission(&self, user_id: UserId, permission: Permission) -> AppResult<()>;
    async fn remove_permission(&self, user_id: UserId, permission: Permission) -> AppResult<()>;
    async fn assign_role(&self, user_id: UserId, role_id: RoleId) -> AppResult<()>;
    async fn remove_role(&self, user_id: UserId, role_id: RoleId) -> AppResult<()>;
    // async fn save(&self, user: User) -> Result<(), ApplicationError> {
    //     let record: UserRecord = user.into();

    //     let thing = Thing::from(("user", record.id.clone()));

    //     self.client
    //         .db
    //         .update(thing)
    //         .content(record)
    //         .await?;

    //     Ok(())
    // }

    // async fn get_by_id(&self, id: UserId) -> Result<Option<User>, ApplicationError> {
    //     let thing = Thing::from(("user", id.to_string()));

    //     let record: Option<UserRecord> = self.client
    //         .db
    //         .select(thing)
    //         .await?;

    //     match record {
    //         None => Ok(None),
    //         Some(rec) => Ok(Some(rec.try_into()?)),
    //     }
    // }
}
