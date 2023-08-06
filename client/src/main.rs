use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let response = client.get("http://localhost:3000/questions")
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

    //APOD
    let response = client.get("https://api.nasa.gov/planetary/apod?api_key=OzpcTPWl9C57laK3tZT4bz8mL87oJXW2PfDkTS5f&count=6")
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}
