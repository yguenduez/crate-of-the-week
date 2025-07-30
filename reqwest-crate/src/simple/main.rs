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
