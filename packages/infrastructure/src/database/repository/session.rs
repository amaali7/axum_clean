use crate::database::client::SurrealDBClient;

pub struct SurrealSessionRepository {
    client: SurrealDBClient,
}

impl SurrealSessionRepository {
    pub fn new(client: SurrealDBClient) -> Self {
        Self { client }
    }
}
