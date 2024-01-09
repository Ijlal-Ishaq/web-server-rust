use mongodb::{bson::Document, Client, Collection, Database};
use std::{env, sync::Arc};

// Define a struct to hold the MongoDB client
pub struct Mongo {
    pub client: Client,
}

impl Mongo {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        // Create a new MongoDB client
        let uri = env::var("MONGODB_URI").expect("set the MONGODB_URI environment var");

        let client = Client::with_uri_str(uri).await?;
        Ok(Self { client })
    }

    pub fn db(&self, name: &str) -> Database {
        self.client.database(name)
    }

    pub fn collection(&self, db_name: &str, coll_name: &str) -> Collection<Document> {
        self.db(db_name).collection(coll_name)
    }
}

// Function to initialize the MongoDB client using a Tokio runtime
pub async fn initialize_mongo() -> Arc<Mongo> {
    let mongo = Mongo::new().await.expect("Failed to initialize MongoDB");
    Arc::new(mongo)
}
