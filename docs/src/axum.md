# Axum - 13th of August

Axum is a powerfull, yet easy to use web framework for Rust, that is built on top of the Tokio runtime.
It's quite modular. You can for example create middlewares with another great crate called `tower`.

What you usually need:

- The async runtime: `cargo add tokio -F full`
- JSON: `cargo add serde -F derive`

And axum itself: `cargo add axum`

## Howto

[Source](https://github.com/tokio-rs/axum?tab=readme-ov-file#usage-example)

The main entry point of axum is the router, where you define your endpoints.

```rust
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Here we have two endpoints:

- `/`: This endpoint to the root URL ("/"). It is mapped to the `root` function.
- `/users`: This endpoint is used to handle HTTP POST requests to the "/users" URL. It is mapped to the `create_user` function, which accepts JSON as input (will be extracted) and creates a JSON response from it.

The callback functions themselves are responsible for handling the incoming requests, where e.g. `Path` variables, `Query` variables or `Json` can be extracted. Those functions are responsible for generating the appropriate responses, that can be turned into a HTTP response.

For example the root function returns just a string:

```rust
async fn root() -> &'static str {
    "Hello, World!"
}
```

Whereas the `create_user` accepts a JSON input (extracts it from the request) and returns a JSON response:

```rust
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn create_user(
    Json(payload): Json<CreateUser>, // JSON-Extractor
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
```

If you run the example in `axum-crate` with `cargo run`. It will open up a web server at `http://localhost:3000`.
You can curl to test it:

```sh
curl -X POST -H "Content-Type: application/json" -d '{"username": "john"}' http://localhost:3000/users
```

or (to call the root endpoint)

```sh
curl -X GET http://localhost:3000
```

## Other examples

You can find more examples, like graphql, chats, or using ORMs etc in the [axum repository](https://github.com/tokio-rs/axum/tree/main/examples).
