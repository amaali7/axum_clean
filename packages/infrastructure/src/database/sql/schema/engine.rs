#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseEngine {
    SurrealDb,
    Sqlx(SqlxEngine),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlxEngine {
    Postgres,
    Sqlite,
    MySql,
}
