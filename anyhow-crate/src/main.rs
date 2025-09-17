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
