use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

pub mod error;
pub mod types;
pub mod schema;
pub mod migrations;

/// API for docker surrealdb service (deprecated - use DatabaseManager instead)
#[allow(dead_code)]
pub async fn connect() -> surrealdb::Result<Surreal<Client>> {
    // Create the WebSocket connection using the correct type
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Sign in with root credentials (or use your credentials)
    db.signin(surrealdb::opt::auth::Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Switch to the desired namespace and database
    db.use_ns("test").use_db("test").await?;

    // Your DB logic here
    println!("Connected to SurrealDB!");

    Ok(db)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[tokio::test]
    pub async fn test_connection() {
        let _ = connect().await.expect("Failed");
    }

    #[tokio::test]
    pub async fn test_locking() {
        use std::sync::LazyLock;
        use surrealdb::engine::remote::ws::Client;
        use surrealdb::Surreal;

        let db: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);
        db.connect::<Ws>("127.0.0.1:8080")
            .await
            .expect("failed to connect");
    }
}
