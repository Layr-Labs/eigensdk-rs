use reqwest::Error;

pub async fn get_url_content(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    response.text().await
}
