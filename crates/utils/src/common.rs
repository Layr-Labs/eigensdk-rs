use reqwest::{Client, Error};

pub async fn get_url_content(client: &Client, url: &str) -> Result<String, Error> {
    let resp = client.get(url).send().await?;
    resp.text().await
}
