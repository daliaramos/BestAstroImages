use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();


    //APOD
    let astroObj = client.get("https://api.nasa.gov/planetary/apod?api_key=OzpcTPWl9C57laK3tZT4bz8mL87oJXW2PfDkTS5f&count=6")
        .send()
        .await?;

    let body = astroObj.text().await?;
    println!("{}", body);




    //When a user liked an image save it to the image table
     let res = client
         .post("http://localhost:3000/image")
         .header("Content-Type", "application/json")
         .body("JSON BODY")
         .send().await?;

     let body = res.text().await?;
     println!("POST Response: {}", body);


    Ok(())
}
