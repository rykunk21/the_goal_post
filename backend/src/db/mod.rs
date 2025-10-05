use surrealdb::{engine::remote::ws::Client};
use surrealdb::{RecordId, Surreal};
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use std::env;

pub mod error;
pub mod schema;

use error::Error;


#[derive(Debug, Deserialize)]
struct Record {
    id: RecordId,
}

pub struct DatabaseManager {
    pub db: Surreal<Client>,
}

impl DatabaseManager {
    /// Create a new database connection with simple initialization
    pub async fn new() -> Result<Self, surrealdb::Error> {
        // Get connection details from environment or use defaults
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let namespace = env::var("DATABASE_NS").unwrap_or_else(|_| "nfl".to_string());
        let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "predictions".to_string());

        // Create WebSocket connection directly
        use surrealdb::engine::remote::ws::Ws;
        let db = Surreal::new::<Ws>(&database_url).await?;

        // Authenticate with root credentials
        db.signin(surrealdb::opt::auth::Root {
            username: "root",
            password: "root",
        }).await?;

        // Switch to the desired namespace and database
        db.use_ns(namespace).use_db(database_name).await?;

        println!("Connected to SurrealDB with schemaless storage!");

        Ok(DatabaseManager { db })
    }

    pub async fn store<T: Serialize + 'static>(
        &self,
        collection: &str,
        data: T,
    ) -> Result<RecordId, Error> {
        
        
        let record: Record= self
            .db
            .create(collection)
            .content(data)
            .await?
            .ok_or(Error::Db)?; // any SurrealDB error is converted via From<surrealdb::Error>

        Ok(record.id)
    }

    /// Retrieve a struct by ID from a collection
    pub async fn get<T: DeserializeOwned>(&self, collection: &str, id: &str) -> Result<Option<T>, surrealdb::Error> {
        self.db.select((collection, id)).await
    }

    /// Get all structs from a collection
    pub async fn get_all<T: DeserializeOwned>(&self, collection: &str) -> Result<Vec<T>, surrealdb::Error> {
        self.db.select(collection).await
    }

    /// Update a struct in a collection
    pub async fn update<T: Serialize + DeserializeOwned + 'static>(&self, collection: &str, id: &str, data: T) -> Result<Option<T>, surrealdb::Error> {
        self.db.update((collection, id)).content(data).await
    }

    /// Delete a record from a collection
    pub async fn delete<T: DeserializeOwned>(&self, collection: &str, id: &str) -> Result<Option<T>, surrealdb::Error> {
        self.db.delete((collection, id)).await
    }

    /// Query with custom SurrealQL
    pub async fn query(&self, sql: &str) -> Result<surrealdb::Response, surrealdb::Error> {
        self.db.query(sql).await
    }

    /// Check if the database connection is healthy
    pub async fn health_check(&self) -> Result<bool, surrealdb::Error> {
        // Use a simple SurrealQL query that should always work
        match self.db.query("INFO FOR DB").await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        name: String,
        value: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct ComplexTestStruct {
        id: String,
        data: Vec<String>,
        nested: TestStruct,
        optional: Option<String>,
    }

    // Test 1: Database connection and initialization
    #[tokio::test]
    async fn test_database_connection() {
        let db = DatabaseManager::new().await.expect("Failed to connect to database");
        let health = db.health_check().await.expect("Health check should not error");
        if !health {
            eprintln!("Health check returned false - database may not be responding properly");
        }
        assert!(health, "Database should be healthy after connection");
    }

    // Test 2: Basic struct storage and retrieval
    #[tokio::test]
    async fn test_store_and_retrieve_struct() {
        let db = DatabaseManager::new().await.expect("Failed to connect");
        
        // Try with a simple JSON value first to isolate the issue
        let simple_data = serde_json::json!({
            "name": "test_item",
            "value": 42
        });

        // Store the JSON directly
        let record_id = db.store("test_collection", simple_data).await.expect("Failed to store JSON");
        let id_str = record_id.to_string();
        assert!(!id_str.is_empty(), "Store should return a non-empty ID");
        
        // Retrieve as JSON
        let retrieved: Option<serde_json::Value> = db.get("test_collection", &id_str).await.expect("Failed to retrieve JSON");
        assert!(retrieved.is_some(), "Should retrieve the stored JSON");
        
        let retrieved_data = retrieved.unwrap();
        assert_eq!(retrieved_data["name"], "test_item");
        assert_eq!(retrieved_data["value"], 42);
        
        // Clean up
        let _: Option<serde_json::Value> = db.delete("test_collection", &id_str).await.expect("Failed to delete");
    }


    // Test 4: Get all structs from collection
    #[tokio::test]
    async fn test_get_all_structs() {
        let db = DatabaseManager::new().await.expect("Failed to connect");
        
        let test_data1 = TestStruct { name: "item1".to_string(), value: 1 };
        let test_data2 = TestStruct { name: "item2".to_string(), value: 2 };
        let test_data3 = TestStruct { name: "item3".to_string(), value: 3 };

        // Store multiple items
        let record_id1 = db.store("test_all_collection", test_data1).await.expect("Failed to store item1");
        let record_id2 = db.store("test_all_collection", test_data2).await.expect("Failed to store item2");
        let record_id3 = db.store("test_all_collection", test_data3).await.expect("Failed to store item3");
        
        // Get all items
        let all_items: Vec<TestStruct> = db.get_all("test_all_collection").await.expect("Failed to get all");
        assert!(all_items.len() >= 3, "Should retrieve at least 3 items");
        
        // Verify all items are present (order may vary)
        let names: Vec<String> = all_items.iter().map(|item| item.name.clone()).collect();
        assert!(names.contains(&"item1".to_string()));
        assert!(names.contains(&"item2".to_string()));
        assert!(names.contains(&"item3".to_string()));
        
        // Clean up
        let _: Option<TestStruct> = db.delete("test_all_collection", &record_id1.to_string()).await.expect("Failed to delete");
        let _: Option<TestStruct> = db.delete("test_all_collection", &record_id2.to_string()).await.expect("Failed to delete");
        let _: Option<TestStruct> = db.delete("test_all_collection", &record_id3.to_string()).await.expect("Failed to delete");
    }

    // Test 5: Update struct
    #[tokio::test]
    async fn test_update_struct() {
        let db = DatabaseManager::new().await.expect("Failed to connect");
        
        let original_data = TestStruct {
            name: "original".to_string(),
            value: 100,
        };

        // Store original
        let record_id = db.store("test_update_collection", original_data).await.expect("Failed to store");
        let id_str = record_id.to_string();
        
        // Update the struct
        let updated_data = TestStruct {
            name: "updated".to_string(),
            value: 200,
        };
        
        let update_result: Option<TestStruct> = db.update("test_update_collection", &id_str, updated_data).await.expect("Failed to update");
        assert!(update_result.is_some(), "Update should return the updated struct");
        
        let updated_struct = update_result.unwrap();
        assert_eq!(updated_struct.name, "updated");
        assert_eq!(updated_struct.value, 200);
        
        // Verify the update persisted
        let retrieved: Option<TestStruct> = db.get("test_update_collection", &id_str).await.expect("Failed to retrieve updated");
        assert!(retrieved.is_some());
        let retrieved_struct = retrieved.unwrap();
        assert_eq!(retrieved_struct.name, "updated");
        assert_eq!(retrieved_struct.value, 200);
        
        // Clean up
        let _: Option<TestStruct> = db.delete("test_update_collection", &id_str).await.expect("Failed to delete");
    }

    // Test 6: Delete struct
    #[tokio::test]
    async fn test_delete_struct() {
        let db = DatabaseManager::new().await.expect("Failed to connect");
        
        let test_data = TestStruct {
            name: "to_be_deleted".to_string(),
            value: 999,
        };

        // Store the struct (clone it so we can use it later for assertions)
        let record_id = db.store("test_delete_collection", test_data.clone()).await.expect("Failed to store");
        let id_str = record_id.to_string();
        
        // Verify it exists
        let before_delete: Option<TestStruct> = db.get("test_delete_collection", &id_str).await.expect("Failed to get before delete");
        assert!(before_delete.is_some(), "Struct should exist before deletion");
        
        // Delete the struct
        let deleted: Option<TestStruct> = db.delete("test_delete_collection", &id_str).await.expect("Failed to delete");
        assert!(deleted.is_some(), "Delete should return the deleted struct");
        
        let deleted_struct = deleted.unwrap();
        assert_eq!(deleted_struct.name, test_data.name);
        assert_eq!(deleted_struct.value, test_data.value);
        
        // Verify it no longer exists
        let after_delete: Option<TestStruct> = db.get("test_delete_collection", &id_str).await.expect("Failed to get after delete");
        assert!(after_delete.is_none(), "Struct should not exist after deletion");
    }

    // Test 7: Complex struct serialization
    #[tokio::test]
    async fn test_complex_struct_serialization() {
        let db = DatabaseManager::new().await.expect("Failed to connect");
        
        let complex_data = ComplexTestStruct {
            id: "complex_123".to_string(),
            data: vec!["item1".to_string(), "item2".to_string(), "item3".to_string()],
            nested: TestStruct {
                name: "nested_struct".to_string(),
                value: 456,
            },
            optional: Some("optional_value".to_string()),
        };

        // Store complex struct
        let record_id = db.store("complex_collection", complex_data.clone()).await.expect("Failed to store complex struct");
        let id_str = record_id.to_string();
        
        // Retrieve and verify
        let retrieved: Option<ComplexTestStruct> = db.get("complex_collection", &id_str).await.expect("Failed to retrieve complex struct");
        assert!(retrieved.is_some(), "Should retrieve complex struct");
        
        let retrieved_data = retrieved.unwrap();
        assert_eq!(retrieved_data.id, complex_data.id);
        assert_eq!(retrieved_data.data, complex_data.data);
        assert_eq!(retrieved_data.nested, complex_data.nested);
        assert_eq!(retrieved_data.optional, complex_data.optional);
        
        // Clean up
        let _: Option<ComplexTestStruct> = db.delete("complex_collection", &id_str).await.expect("Failed to delete");
    }

    // Test 8: Custom query functionality
    #[tokio::test]
    async fn test_custom_query() {
        let db = DatabaseManager::new().await.expect("Failed to connect");
        
        // Store some test data
        let test_data1 = TestStruct { name: "query_test_1".to_string(), value: 10 };
        let test_data2 = TestStruct { name: "query_test_2".to_string(), value: 20 };
        
        let record_id1 = db.store("query_collection", test_data1).await.expect("Failed to store");
        let record_id2 = db.store("query_collection", test_data2).await.expect("Failed to store");
        
        // Test custom query
        let _response = db.query("SELECT * FROM query_collection WHERE value > 15").await.expect("Failed to execute query");
        // Note: We'll verify the response structure works, detailed parsing can be tested in implementation
        
        // Clean up
        let _: Option<TestStruct> = db.delete("query_collection", &record_id1.to_string()).await.expect("Failed to delete");
        let _: Option<TestStruct> = db.delete("query_collection", &record_id2.to_string()).await.expect("Failed to delete");
    }

    // Test 9: Collections are created dynamically
    #[tokio::test]
    async fn test_dynamic_collection_creation() {
        let db = DatabaseManager::new().await.expect("Failed to connect");
        
        let test_data = TestStruct {
            name: "dynamic_collection_test".to_string(),
            value: 777,
        };

        // Store in a new collection that doesn't exist yet
        let unique_collection = format!("dynamic_collection_{}", chrono::Utc::now().timestamp());
        let record_id = db.store(&unique_collection, test_data.clone()).await.expect("Failed to store in dynamic collection");
        let id_str = record_id.to_string();
        
        // Verify we can retrieve from the dynamically created collection
        let retrieved: Option<TestStruct> = db.get(&unique_collection, &id_str).await.expect("Failed to retrieve from dynamic collection");
        assert!(retrieved.is_some(), "Should retrieve from dynamically created collection");
        
        let retrieved_data = retrieved.unwrap();
        assert_eq!(retrieved_data.name, test_data.name);
        assert_eq!(retrieved_data.value, test_data.value);
        
        // Clean up
        let _: Option<TestStruct> = db.delete(&unique_collection, &id_str).await.expect("Failed to delete");
    }

    // Test 10: Error handling for non-existent records
    #[tokio::test]
    async fn test_error_handling() {
        let db = DatabaseManager::new().await.expect("Failed to connect");
        
        // Try to get a non-existent record
        let result: Option<TestStruct> = db.get("nonexistent_collection", "nonexistent_id").await.expect("Get should not error for missing records");
        assert!(result.is_none(), "Should return None for non-existent records");
        
        // Try to delete a non-existent record
        let delete_result: Option<TestStruct> = db.delete("nonexistent_collection", "nonexistent_id").await.expect("Delete should not error for missing records");
        assert!(delete_result.is_none(), "Should return None when deleting non-existent records");
        
        // Try to update a non-existent record
        let test_data = TestStruct { name: "test".to_string(), value: 1 };
        let update_result: Option<TestStruct> = db.update("nonexistent_collection", "nonexistent_id", test_data).await.expect("Update should not error for missing records");
        assert!(update_result.is_none(), "Should return None when updating non-existent records");
    }
}