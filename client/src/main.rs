use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKey {
    api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct APIres {
    //#[serde(flatten)]
    //copyright: String,
    date: String,
    explanation: String,
    hdurl: String,
    media_type: String,
    service_version: String,
    title: String,
    url: String
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    /*
    let client = Client::new();

    let res = client.get("https://api.nasa.gov/planetary/apod?api_key=OzpcTPWl9C57laK3tZT4bz8mL87oJXW2PfDkTS5f")
        .send()
        .await?
        .json::<APIres>()
        .await?;

    println!("{:?}", res);


*/


/*
    let body = res.text().await?;

    println!("{}", body);
*/





    Ok(())
}
>>>>>>> roles
