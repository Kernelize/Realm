use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

impl AppState {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}
