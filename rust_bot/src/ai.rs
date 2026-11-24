use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone)]
pub struct AiClient {
    http: Client,
    endpoint: String,
    api_key: String,
    model: String,
}
#[derive(Serialize)]
struct ChatRequest<'a> {
 model: &'a str,
 messages: Vec<ChatMessage<'a>>,
 temperature: f32,
}
#[derive(Serialize)]
struct ChatMessage<'a> { role: &'a str, content: &'a str }
#[derive(Deserialize)]
struct ChatResponse { choices: Vec<Choice> }
#[derive(Deserialize)]
struct Choice { message: ResponseMessage }
#[derive(Deserialize)]
struct ResponseMessage { content: String }

impl AiClient {
 pub fn from_env() -> Result<Self, String> {
  let api_key=std::env::var("AI_API_KEY").or_else(|_|std::env::var("OPENAI_API_KEY")).map_err(|_|"AI_API_KEY or OPENAI_API_KEY is required".to_string())?;
  let endpoint=std::env::var("AI_API_URL").unwrap_or_else(|_|"https://api.openai.com/v1/chat/completions".to_string());
  let model=std::env::var("AI_MODEL").unwrap_or_else(|_|"gpt-4o-mini".to_string());
  let http=Client::builder().timeout(Duration::from_secs(45)).build().map_err(|e|format!("AI HTTP client setup: {e}"))?;
  Ok(Self{http,endpoint,api_key,model})
 }
 pub async fn ask(&self, system:&str, user:&str)->Result<String,String>{
  let request=ChatRequest{model:&self.model,messages:vec![ChatMessage{role:"system",content:system},ChatMessage{role:"user",content:user}],temperature:0.7};
  let response=self.http.post(&self.endpoint).bearer_auth(&self.api_key).json(&request).send().await.map_err(|e|format!("AI request failed: {e}"))?;
  let status=response.status();
  if !status.is_success(){let body=response.text().await.unwrap_or_default();return Err(format!("AI provider returned HTTP {status}: {}",body.chars().take(500).collect::<String>()));}
  let parsed:ChatResponse=response.json().await.map_err(|e|format!("AI response parsing failed: {e}"))?;
  parsed.choices.into_iter().next().map(|c|c.message.content).filter(|s|!s.trim().is_empty()).ok_or_else(||"AI provider returned no content".to_string())
 }
}
