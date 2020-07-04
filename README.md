# arangoq
A quick arangodb query builder layer for rust.

```rust
   let url = || mockito::server_url();
   let conn = ArangoConnection::new(url(), "test_db".to_string(), Client::default());

   #[derive(ArangoBuilder, Serialize)]
   pub struct Person {
      name: &'static str,
      age: u8,
   }

   let collection_name = "People";
   let query = Person::query_builder(collection_name)
      .read()
      .filter()
      .name_eq(&"John Lennon")
      .or()
      .name_eq(&"George Harrison")
      .and()
      .age_gt(&42)
      .limit(10)
      .build();

   query.try_exec::<Person>(&conn).await;
```

## optional features
   * ["actors"] actix async actor implementation for queries

# Semver
This crate is in pre semver state, breaking changes increment minor.

# License
This project is licensed under either of
 - Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
 - MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
at your option.

[1](http://unhandledexpression.com/general/2018/11/27/foss-is-free-as-in-toilet.html)

## cargo
cargo install cargo-sort-ck

cargo-sort-ck

cargo clippy

## release
cargo install cargo-release

cargo release patch
