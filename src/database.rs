use crate::arango_api::{Collection, CollectionType};
use crate::arango_connection::ArangoConnection;

pub struct Database {
    pub name: String,
    pub connection: ArangoConnection,
}

impl Database {
    pub async fn create_collection(
        &self,
        local_name: &str,
        collection_type: CollectionType
    ) {
        let qualified_name = self.connection.context.collection_name(local_name);
        let coll_url = self.connection.collection();
        
        let data = serde_json::json!({
            "name": qualified_name,
            "type": collection_type as u8
        });
        log::debug!("{}", data.to_string());
        let res = self.connection
            .client
            .post(coll_url.as_str())
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .basic_auth(
                std::env::var("ARANGO_USER_NAME").unwrap_or_default(),
                std::env::var("ARANGO_PASSWORD").ok(),
            )
            .json(&data)
            .send()
            .await;
        log::debug!("{:#?}", res);
    }
    
    pub async fn list_collections(&self) -> Vec<Collection> {
        let coll_url = self.connection.collection();
        
        let client = reqwest::Client::new();
        let fut = async {
            let res = client
                .get(coll_url.as_str())
                .header("accept", "application/json")
                .header("content-type", "application/json")
                .basic_auth(
                    std::env::var("ARANGO_USER_NAME").unwrap_or_default(),
                    std::env::var("ARANGO_PASSWORD").ok(),
                )
                .send()
                .await;
            if let Ok(resp) = res {
                let data: serde_json::Value = resp.json().await.unwrap_or_default();
                let resutls: Vec<Collection> =
                    serde_json::from_value(data["result"].clone()).unwrap_or_default();
                return resutls;
            }
            return vec![];
        };
        fut.await
    }
}
