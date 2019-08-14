#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
mod tests {
    use crate::*;

    fn test_collection() -> Collection {
        Collection::new("Beatles", CollectionType::Document)
    }

    // .unwrap() calls in the unit tests are safe to use.

    #[derive(Serialize, Deserialize)]
    struct TestUser {
        name: String
    }

    impl TestUser {
        fn new(name: &str) -> Self {
            Self { name: name.to_owned() }
        }
    }

    #[test]
    fn test_collection_insert() {
        let query = test_collection()
            .insert(&TestUser::new("Paul McCartney"));
        let expected = r#"{"query":"INSERT @value INTO @@collection","bindVars":{"@collection":"Beatles","value":{"name":"Paul McCartney"}}}"#;

        assert_eq!(expected, serde_json::to_string(&query).unwrap());
    }

    #[test]
    fn test_collection_get_all() {
        let query = test_collection().get_all();

        let expected =
            r#"{"query":"FOR item in @@collection RETURN item","bindVars":{"@collection":"Beatles"}}"#;

        assert_eq!(expected, serde_json::to_string(&query).unwrap());
    }

    #[test]
    fn test_collection_get_by_key() {
        let query = test_collection().get_by_key("Paul");
        let expected =
            r#"{"query":"RETURN DOCUMENT(@@collection, @key)","bindVars":{"@collection":"Beatles","key":"Paul"}}"#;

        assert_eq!(expected, serde_json::to_string(&query).unwrap());
    }

    #[test]
    fn test_collection_get_by_keys() {
        let query = test_collection().get_by_keys(&["Paul", "John", "Ringo", "George"]);
        let expected =
            r#"{"query":"RETURN DOCUMENT(@@collection, @keys)","bindVars":{"@collection":"Beatles","keys":["Paul","John","Ringo","George"]}}"#;
        assert_eq!(expected, serde_json::to_string(&query).unwrap());
    }

    #[test]
    fn test_collection_replace() {
        let query = test_collection().replace("Paul", &TestUser::new("John Lennon"));
        let expected =
            r#"{"query":"REPLACE @key WITH @elem IN @@collection","bindVars":{"@collection":"Beatles","elem":{"name":"John Lennon"},"key":"Paul"}}"#;
        assert_eq!(expected, serde_json::to_string(&query).unwrap());
    }

    #[test]
    fn test_collection_update() {
        #[derive(Serialize)]
        struct Instrument {
            instrument: String,
        }

        let query = test_collection()
            .update(
                "Paul",
                &Instrument {
                    instrument: String::from("bass"),
                },
            );
        let expected =
            r#"{"query":"UPDATE @key WITH @update IN @@collection","bindVars":{"@collection":"Beatles","key":"Paul","update":{"instrument":"bass"}}}"#;
        assert_eq!(expected, serde_json::to_string(&query).unwrap());
    }

    #[test]
    fn test_collection_remove() {
        let query = test_collection().remove("Paul");
        let expected =
            r#"{"query":"REMOVE @key IN @@collection","bindVars":{"@collection":"Beatles","key":"Paul"}}"#;
        assert_eq!(expected, serde_json::to_string(&query).unwrap());
    }

    #[test]
    fn test_collection_truncate() {
        let query = test_collection().truncate();
        let expected =
            r#"{"query":"FOR item IN @@collection REMOVE item IN @@collection","bindVars":{"@collection":"Beatles"}}"#;
        assert_eq!(expected, serde_json::to_string(&query).unwrap());
    }

    #[test]
    fn test_generated_arango_builder() {
        #[derive(ArangoBuilder, Serialize)]
        pub struct Person {
            name: &'static str,
            age: u8,
        }

        let collection_name = "People";

        let query1 = Person::query_builder(collection_name).read().build();
        let query2 = Person::query_builder(collection_name)
            .read()
            .filter()
            .name_eq(&"John Lennon")
            .build();
        let query3 = Person::query_builder(collection_name)
            .read()
            .filter()
            .name_eq(&"John Lennon")
            .filter()
            .age_gt(&42)
            .build();

        let query4 = Person::query_builder(collection_name)
            .read()
            .filter()
            .name_eq(&"John Lennon")
            .or()
            .name_eq(&"George Harrison")
            .and()
            .age_gt(&42)
            .limit(10)
            .build();

        let query5 = Person::query_builder(collection_name)
            .create(&Person {
                name: "Douglas Adams",
                age: 42,
            })
            .build();

        let query6 = Person::query_builder(collection_name).delete().build();

        let query7 = Person::query_builder(collection_name)
            .delete()
            .filter()
            .name_eq(&"John Lennon")
            .build();

        let query8 = Person::query_builder(collection_name)
            .update()
            .name(&"John Lennon")
            .build();

        let query9 = Person::query_builder(collection_name)
            .update()
            .filter()
            .name_eq(&"Paul McCartney")
            .age(&66)
            .build();

        let query10 = Person::query_builder(collection_name)
            .update()
            .filter()
            .name_eq(&"Paul McCartney")
            .replace_with(&Person {
                name: "Douglas Adams",
                age: 42,
            })
            .build();

        let query11 = Person::query_builder(collection_name)
            .read()
            .filter()
            .name_in(&["John", "Paul", "Ringo", "George"])
            .limit(10)
            .build();

        let query12 = Person::query_builder(collection_name)
            .read()
            .filter()
            .age_not_in(&[41, 42, 43, 44, 45])
            .limit(10)
            .build();

        let values = vec![
            (query1, r#"{"query":"FOR item IN @@collection LIMIT @limit RETURN item ","bindVars":{"@collection":"People","limit":100}}"#),
            (query2, r#"{"query":"FOR item IN @@collection FILTER item.name == @filterVar2 LIMIT @limit RETURN item ","bindVars":{"@collection":"People","filterVar2":"John Lennon","limit":100}}"#),
            (query3, r#"{"query":"FOR item IN @@collection FILTER item.name == @filterVar2 FILTER item.age > @filterVar3 LIMIT @limit RETURN item ","bindVars":{"@collection":"People","filterVar2":"John Lennon","filterVar3":42,"limit":100}}"#),
            (query4, r#"{"query":"FOR item IN @@collection FILTER item.name == @filterVar2 OR item.name == @filterVar3 AND item.age > @filterVar4 LIMIT @limit RETURN item ","bindVars":{"@collection":"People","filterVar2":"John Lennon","filterVar3":"George Harrison","filterVar4":42,"limit":10}}"#),
            (query5, r#"{"query":"INSERT @elem INTO @@collection ","bindVars":{"@collection":"People","elem":{"age":42,"name":"Douglas Adams"}}}"#),
            (query6, r#"{"query":"FOR item IN @@collection REMOVE item IN @@collection ","bindVars":{"@collection":"People"}}"#),
            (query7, r#"{"query":"FOR item IN @@collection FILTER item.name == @filterVar1 REMOVE item IN @@collection ","bindVars":{"@collection":"People","filterVar1":"John Lennon"}}"#),
            (query8, r#"{"query":"FOR item IN @@collection UPDATE item WITH { name: @withVar1 } IN @@collection ","bindVars":{"@collection":"People","withVar1":"John Lennon"}}"#),
            (query9, r#"{"query":"FOR item IN @@collection FILTER item.name == @filterVar1 UPDATE item WITH { age: @withVar2 } IN @@collection ","bindVars":{"@collection":"People","filterVar1":"Paul McCartney","withVar2":66}}"#),
            (query10, r#"{"query":"FOR item IN @@collection FILTER item.name == @filterVar1 UPDATE item WITH @withVar2 IN @@collection ","bindVars":{"@collection":"People","filterVar1":"Paul McCartney","withVar2":{"age":42,"name":"Douglas Adams"}}}"#),
            (query11, r#"{"query":"FOR item IN @@collection FILTER item.name IN @filterVar2 LIMIT @limit RETURN item ","bindVars":{"@collection":"People","filterVar2":["John","Paul","Ringo","George"],"limit":10}}"#),
            (query12, r#"{"query":"FOR item IN @@collection FILTER item.age NOT IN @filterVar2 LIMIT @limit RETURN item ","bindVars":{"@collection":"People","filterVar2":[41,42,43,44,45],"limit":10}}"#),
        ];

        for (query, expected) in values.into_iter() {
            assert_eq!(expected, serde_json::to_string(&query).unwrap());
        }
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct U {
        _key: String,
        _id: String,
        _rev: String,
        name: String,
    }

    impl U {
        pub fn new(_key: &str, _id: &str, _rev: &str, name: &str) -> Self {
            Self {
                _key: String::from(_key),
                _id: String::from(_id),
                _rev: String::from(_rev),
                name: String::from(name),
            }
        }
    }

    fn test_response() -> ArangoResponse<U> {
        ArangoResponse::new(
            vec![U::new("13221", "Characters/13221", "_ZEJkt1W---", "John Doe")],
            false,
            false,
            ArangoResponseExtra::new(0, 0, 0, 0, 0, 0, 3.654956817626953e-4, 2107, vec![]),
            false,
            201,
        )
    }

    fn test_response_json() -> &'static str {
        r#"{"result":[{"_key":"13221","_id":"Characters/13221","_rev":"_ZEJkt1W---","name":"John Doe"}],"hasMore":false,"cached":false,"extra":{"stats":{"writesExecuted":0,"writesIgnored":0,"scannedFull":0,"scannedIndex":0,"filtered":0,"httpRequests":0,"executionTime":0.0003654956817626953,"peakMemoryUsage":2107},"warnings":[]},"error":false,"code":201}"#
    }

    #[test]
    fn test_arango_response_ser_deser() {

        assert_eq!(
            test_response_json(),
            serde_json::to_string(&test_response()).unwrap()
        );

        assert_eq!(
            test_response(),
            serde_json::from_str(&serde_json::to_string(&test_response()).unwrap()).unwrap()
        );

        assert_eq!(
            test_response(),
            serde_json::from_slice(&serde_json::to_vec(&test_response()).unwrap()).unwrap()
        );
    }

    #[test]
    fn test_arango_mock() {
        let query = || Collection::new("Characters", CollectionType::Document).get_by_key("13221");
        let query_json = || serde_json::to_string(&query()).unwrap();
        let test_mock = ArangoMock::new(hashmap![query_json() => test_response_json().to_owned()]);
        assert_eq!(
            test_response_json(),
            &test_mock.execute_query(query())
        );
    }
}
