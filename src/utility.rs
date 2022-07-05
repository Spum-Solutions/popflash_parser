use select::document::Document;

/// This makes use of the function `get_body_from_url` function in the same module by converting the id to a valid url, this is just for quality of life reasons
pub async fn get_body_from_id(
    match_id: usize,
) -> Result<select::document::Document, Box<dyn std::error::Error>> {
    let url = format!("https://popflash.site/match/{}", match_id);
    let result = get_body_from_url(url.as_ref()).await?;

    Ok(result)
}

pub async fn get_body_from_url(
    match_url: &str,
) -> Result<select::document::Document, Box<dyn std::error::Error>> {
    let body = reqwest::get(match_url).await?.text().await?;

    let document = Document::from(body.as_ref());
    Ok(document)
}
