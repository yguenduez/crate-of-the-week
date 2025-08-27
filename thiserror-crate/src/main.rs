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
