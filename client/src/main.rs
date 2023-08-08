use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();


    //APOD
    let response = client.get("https://api.nasa.gov/planetary/apod?api_key=OzpcTPWl9C57laK3tZT4bz8mL87oJXW2PfDkTS5f&count=6")
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);


    let response = client.get("https://storage.googleapis.com/google-code-archive-downloads/v2/code.google.com/badwordslist/badwords.txt")
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}
