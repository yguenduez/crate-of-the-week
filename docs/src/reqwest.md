# Reqwest - 30th of July

Needed to run async code:

`cargo add tokio -F full`

No worries. Tokio will be explained in the next weeks. For now you only need to know we need an async runtime, that runs our async code.

`cargo add reqwest`

Is an easy to use http client for Rust.

# How to use it

Below snippet will fetch the HTML from Google and print it to the console.

```rust
#[tokio::main]
async fn main() {
    let html_from_google = reqwest::get("https://www.google.com")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{html_from_google}")
}
```

The example lives in `reqwest-crate` directory. To run it, use

```sh
cargo run --bin simple
```

# How it's normally used

You can also combine serde with reqwest to deserialize JSON responses when fetching data,
or serialize data, if you want to send data to a server.

```sh
cargo add serde -F derive
cargo add reqwest -F json
```

## Getting data

```rust
use serde::Deserialize;

const URL: &str = "https://pokeapi.co/api/v2/pokemon";

#[derive(Deserialize, Debug)]
struct Pokemon {
    name: String,
    height: u32,
}

#[tokio::main]
async fn main() {
    let pokename = "bulbasaur";
    let url_with_endpoint = format!("{}/{}", URL, pokename);
    let pokemon: Pokemon = reqwest::get(url_with_endpoint) // << will get the response
        .await
        .unwrap() // << will crash, if we have an error fetching the data
        .json() // << will deserialize the response body into a Pokemon struct
        .await
        .unwrap(); // << will crash, if we have an error deserializing the data

    println!("{pokemon:?}")
}
```

You can run this example by running (from `reqwest-crate` directory)

```sh
cargo run --bin with_json
```

It should print something like this:

```text
Pokemon { name: "bulbasaur", height: 7 }
```

# More sophisticated things

You can also utilize reqwest's builders to create more complex requests.

```rust
// This will POST a body of `foo=bar&baz=quux`
let params = [("foo", "bar"), ("baz", "quux")];
let client = reqwest::Client::new();
let res = client.post("http://httpbin.org/post")
    .form(&params)
    .send()
    .await?;
```

This will send a form to the given URL.

See [reqwest](https://docs.rs/reqwest/latest/reqwest/) for more.
