#[cfg(test)]

mod tests {
    use arangoq::*;
    use reqwest::{Body, Client};
    use arangoq::{Collection, CollectionType};
    use arangoq::arango_api::Insert;
    use serde_json;
    use serde::{Serialize, Deserialize};
    use Database;
    
    #[derive(ArangoBuilder, Serialize, Deserialize, Debug, PartialEq, Clone)]
    pub struct Person {
        name: String,
        age: u8,
    }
    
    #[actix_rt::test]
    async fn test_live_queries() {
        std::env::set_var("ARANGO_USER_NAME", "test_dev");
        std::env::set_var("ARANGO_PASSWORD", "test_dev_pw");
        let conn = ArangoConnection::new(String::from("http://localhost:8529/"), "test_db".to_string(), Client::default());
        
        let test_person_entry = Person {
            name: String::from("Testname"),
            age: 42
        };
        
        let db = Database {
            name: "asdasd".to_owned(),
            connection: ArangoConnection::new(String::from("http://localhost:8529/"), "test_db".to_string(), Client::default())
        };
        
        db.create_collection("testdocs", CollectionType::Document).await;
        
        let coll = Collection::new("testdocs", CollectionType::Document);
        let query = coll.insert(&test_person_entry);
        
        let collection_name = "People";
        
        let result = query.try_exec::<Person>(&conn).await;
        assert!(!result.unwrap().error);
    }
}