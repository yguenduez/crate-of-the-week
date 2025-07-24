# Serde - 23th of July

`cargo add serde -F derive`

Gives you a powerful way to serialize and deserialize data in Rust.

```rust
#[derive(Serialize, Deserialize)]
struct Hans {
    is_boss: bool,
    company: String,
}
```

And with helping crates like `serde_json`:

`cargo add serde_json`

You can use `serde_json` to serialize and deserialize JSON data.

```rust
fn serialize(hans: &Hans) -> Result<String, serde_json::Error> {
    serde_json::to_string(hans)
}
```

```rust
fn main() {
    let hans = Hans {
        is_boss: true,
        company: "EHDS".to_string(),
    };

    match serialize(&hans) {
        Ok(json) => println!("Serialized JSON: {}", json),
        Err(err) => println!("Serialization error: {}", err),
    }
}
```

You can also use `serde_json` to deserialize JSON data.

```rust
fn deserialize(json: &str) -> Result<Hans, serde_json::Error> {
    serde_json::from_str(json)
}
```

```rust
fn main() {
    let json = r#"{"is_boss": true, "company": "EHDS"}"#;
    match deserialize(json) {
        Ok(hans) => println!("Deserialized Hans: {:?}", hans),
        Err(err) => println!("Deserialization error: {}", err),
    }
}
```

## An example from live code

You can also use serde's traits to build more powerful data structures.
You can have a look at the `post` method. It takes a T, that implements the `serde::Serialize` trait.

So the post method takes any T, that is serializable.

```rust
pub trait HttpsClient {
    fn get(&self, request: GetRequest) -> Result<GetResponse, FailureType>;
    fn post<T: 'static + serde::Serialize>(
        &self,
        request: PostRequest,
        content: &T,
    ) -> Result<PostResponse, FailureType>;
    fn post_file(&self, request: PostFileRequest) -> Result<PostResponse, FailureType>;
    fn delete(&self, request: DeleteRequest) -> Result<DeleteResponse, FailureType>;
}
```
