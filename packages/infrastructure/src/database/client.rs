use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, Surreal};
use crate::error::InfrastructureResult;

#[derive(Clone)]
pub struct SurrealDBClient {
    pub db: Surreal<Client>,
}

impl SurrealDBClient {
    pub async fn new(url: &str, user: &str, pass: &str) -> InfrastructureResult<Self> {
        // 1. open one connection (or a pool of connections if you prefer)
        let db = Surreal::new::<Ws>(url).await?;

        db.signin(Root {
            username: user,
            password: pass,
        }).await?;
        db.use_ns("test").use_db("test").await?;

        Ok(Self { db })
    }
}
