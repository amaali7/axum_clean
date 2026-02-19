use async_trait::async_trait;
use application::{RequestContex, error::{AppResult, ApplicationError}, ports::{ SortBy, RoleRepository, role::RoleQueryResult}};
use domain::{Name, role::{Role, RoleId}};

use crate::{
    database::client::SurrealDBClient, error::InfrastructureError,
    serialization::{
        SerializedRoleId, role::{role::SerializedRole, SurrealRoleResponseExt}, value_objects::SerializedName,
        
    }
};

pub struct SurrealRoleRepository {
    client: SurrealDBClient,
}

impl SurrealRoleRepository {
    pub fn new(client: SurrealDBClient) -> Self {
        Self { client }
    }
}

// TODO: Permission must have logic for ranking it
#[async_trait]
impl RoleRepository for SurrealRoleRepository {
    async fn create(&self,ctx: RequestContex, role: Role) -> AppResult<Role>{
        let record: SerializedRole = role.try_into().map_err(|_| ApplicationError::Repository("Role Error ".to_string()))?;
        let result: Option<SerializedRole> =  self
            .client
            .db
            .query("LET $role_id = $uid;
                    INSERT INTO role CONTENT $role RETURN AFTER")
            .bind(("uid", ctx.user_id_as_str()))
            .bind(("role", record))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(role) => Ok(role.try_into()?),
            None => Err(ApplicationError::Repository("Role not created!".to_string())),
        }
        
    }
    async fn update(&self,ctx: RequestContex, role: Role) -> AppResult<Role>{
        let record: SerializedRole = role.try_into().map_err(|err: InfrastructureError| ApplicationError::Domain(err.into()))?;
        let result: Option<SerializedRole> =  self
            .client
            .db
            .query("LET $role_id = $uid;
                    UPDATE role:$id MERGE $role RETURN AFTER")
            .bind(("role", record.clone()))
            .bind(("uid", ctx.user_id_as_str()))
            .bind(("id", record.id()))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(role) => Ok(role.try_into()?),
            None => Err(ApplicationError::Repository("Role not Updated!".to_string())),
        }
    }
    async fn get_by_id(&self, _request_contex: RequestContex, id: RoleId) -> AppResult<Role>{
        let id: SerializedRoleId = id.into();
        let result: Option<SerializedRole> =  self
            .client
            .db
            .query("SELECT * FROM ONLY role:$id")
            .bind(("id", id))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(role) => Ok(role.try_into()?),
            None => Err(ApplicationError::Repository("Role not Updated!".to_string())),
        }
    }
    async fn delete(&self,ctx: RequestContex, id: RoleId) -> AppResult<bool>{
        let result: Option<SerializedRole> =  self
            .client
            .db
            .query("LET $role_id = $uid;
                    DELETE person:$id RETURN BEFORE")
            .bind(("uid", ctx.user_id_as_str()))
            .bind(("id", id.id()))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(_) => Ok(true),
            None => Err(ApplicationError::Repository("Role not deleted!".to_string())),
        }
    }
    async fn get_by_name(&self, _request_contex:RequestContex, name: Name) -> AppResult<Role>{
        let name: SerializedName = name.try_into()?;
        let result: Option<SerializedRole> =  self
            .client
            .db
            .query("SELECT * FROM role WHERE name = $name LIMIT 1")
            .bind(("name", name))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(role) => Ok(role.try_into()?),
            None => Err(ApplicationError::Repository("Role not Updated!".to_string())),
        }
    }
    
    // TODO: Add permissions field into requert contex to be prossesed as required take into account murge all roles permessions with the role ones
// TODO: Add permissions field into requert contex to be prossesed as required take into account murge all roles permessions with the user ones
    async fn get_roles_paginated(&self,ctx: RequestContex ,sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<Role>>{
        let mut order = String::new();
        
        for ord in sort_by{
            order.push_str(&format!("{},", ord.as_string()));
        }
        if !sort_by.is_empty(){
            order = format!("ORDER BY {}", order);
            order.truncate(order.len() -1);
        }
        
        let result: Vec<SerializedRole> =  self
            .client
            .db
            .query("SELECT * FROM role $order LIMIT $page_size START $start_at")
            .bind(("order", order))
            .bind(("page_size", page_size))
            .bind(("start_at", page*page_size))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0).map_err(|err| ApplicationError::Repository(err.to_string()))?;
        let mut roles: Vec<Role> = Vec::new();
        for role in result{
            roles.push(role.try_into().map_err(|err: InfrastructureError| ApplicationError::Repository(err.to_string()))?);
        }
        Ok(roles)
    }
    async fn raw_query(&self,ctx: RequestContex, query: String) -> AppResult<RoleQueryResult>{
        let response = self.client
            .db
            .query(query)
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?;
        // Using the extension trait
        Ok(response.into_role_result().await.map_err(|err| ApplicationError::Repository(err.to_string()))?.try_into()?)
    }
}



