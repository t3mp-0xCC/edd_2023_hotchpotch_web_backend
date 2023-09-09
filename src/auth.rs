use actix_web::HttpRequest;
use reqwest::header;
use serde::Deserialize;
use anyhow::{Result, Ok};

#[derive(Debug, Deserialize)]
pub struct GithubUserData {
    pub login: String,
    pub avatar_url: String,
}

pub fn parse_token(req: HttpRequest) -> Option<String> {
  match req.headers().get("Authorization") {
    Some(auth_header) => {
      let auth_str = auth_header.to_str().unwrap();
      if auth_str.starts_with("Bearer ") {
        return Some(auth_str[7..].to_string())
      } else {
        return None
      }
    },
    None => return None
  };
}

pub async fn verification(token: String) -> Result<GithubUserData> {
  let url = "https://api.github.com/user";
  let mut custom_headers = header::HeaderMap::new();
  custom_headers.insert(header::USER_AGENT, "HotchPotch".parse()?);
  custom_headers.insert(header::AUTHORIZATION, format!("Bearer {}", token).parse()?);

  let client = reqwest::Client::new();
  let res = client.get(url)
    .headers(custom_headers)
    .send().await?
    .text().await?;
  
  let user_data = serde_json::from_str(&res)?;
  Ok(user_data)
}