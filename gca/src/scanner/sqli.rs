use reqwest::Client;

pub async fn check_sqli(target: &str) -> anyhow::Result<bool> {
    let payload = "' OR 1=1 --";
    let url = format!("{}?id={}", target, payload);
    let resp = Client::new().get(&url).send().await?;
    if resp.status().is_success() {
        let body = resp.text().await?;
        Ok(body.contains("syntax") || body.contains("error"))
    } else {
        Ok(false)
    }
}
