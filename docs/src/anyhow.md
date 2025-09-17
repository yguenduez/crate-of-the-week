# Anyhow - 17th of September

Anyhow is a rust crate, that gives you an easy way to convert any error type, that implements the `std::error::Error` to be transformed into `anyhow::Error`

## Usage

```sh
cargo add anyhow
```

## Example

If you just want to convert errors to `anyhow::Error` - you most of the time can use the `?` operator

```rust
use anyhow::Result;

fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}
```

## Another example

You also can put custom context to an anyhow result, that will get appended, if an error occurs (see the `anyhow-crate` example).

```rust
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Cluster {
    name: String,
    nodes: Vec<String>,
}

fn get_cluster_info(filepath: &str) -> Result<Vec<Cluster>> {
    let config = std::fs::read_to_string(filepath)?;
    let map: Vec<Cluster> = serde_json::from_str(&config)?;
    Ok(map)
}

fn main() -> Result<()> {
    get_cluster_info("cluster.json").context("This one should pass, right?")?;
    get_cluster_info("does_not_exist.json").context("Oh that did not go well")?;

    Ok(())
}
```

## Comparison with `thiserror`

[Source](https://github.com/dtolnay/anyhow?tab=readme-ov-file#comparison-to-thiserror): You use anyhow, if you are do not really care about the error type.
`thiserror` is used, if you library has to distinguish between different error types, that is important for your library's users.
