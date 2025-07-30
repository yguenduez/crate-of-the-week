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
    let pokemon: Pokemon = reqwest::get(url_with_endpoint)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("{pokemon:?}")
}
