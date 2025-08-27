# Thiserror - 27th of August

If you want to create custom error handling - but without big boilerplate.
This error maps external errors to your own custom errors.

## Usage

```sh
cargo add thiserror
```

## Example

We have two different error sources: `std::io::Error` and `serde_json::Error`.
With `thiserror`, but will automatically transformed into your custom error `MyError`.

```rust
use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug)]
enum MyError {
    #[error("invalid input")]
    InvalidInput,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialize error: {0}")]
    Serialize(#[from] serde_json::Error),
}

#[derive(Serialize)]
struct MyStruct{
    field: String
}

fn main() -> Result<(), MyError> {
    // let create_file = std::fs::write("nonexistent_file.txt", "Hello, world!")?; //< uncomment, to make it work

    // This line will early return, as the file does not exist
    // ? will convert the io::Error to your `MyError::Io`
    let result = std::fs::read_to_string("nonexistent_file.txt")?;

    // to_string is fallible - if an error occurs, it will early return with your `MyError::Serialize`
    let serialized = serde_json::to_string(&MyStruct{field: "value".to_string()})?;
    println!("{}", serialized);

    Ok(())
}
```

If you never have seen the `?` operator before: It's a lot of syntactic sugar, basically doing this operation:

```rust
fn main() -> Result<(), MyError> {
    let result = std::fs::read_to_string("nonexistent_file.txt")?;

    // is the same as:
    let result = match std::fs::read_to_string("nonexistent_file.txt") {
        Ok(result) => result,
        Err(err) => return Err(MyError::Io(err)), // It early returns here
    };

    // If we are here, the `read_to_string` call was successful

    Ok(())
}
```
