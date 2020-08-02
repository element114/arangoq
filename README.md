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

## Similar crates
When [arangors](https://github.com/element114/oas_gen) was published on crates.io we already used `arangoq` internally at Reed Wolf Ltd. Last time we evaluated `arangors`, it was in an early stage, many of it's api were skeleton only.
Today the two crates share minimal functionalities. The focus of `aragors` is to be similar to the Python crate, whereas `arangoq` provides a more Rust like, high level query builder experience and was designed to be more **resilient to insertion attacks** right from the start.

We performed several manual tests to make sure it fits our use-case.
That said, ss always, **use at your own risk**.

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
