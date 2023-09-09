use actix_web::HttpRequest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GithubUserData {
    pub name: String,
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

// pub async fn verification(token: String) -> Result<GithubUserData> {
//     let token = match request.headers().get("Authorization") {
//         Some(token) => token.to_str().map_err(|_| anyhow!("Invalid header value"))?,
//         None => return Err(anyhow!("Authorization header not found")),
//     };

//     let url = format!("https://api.github.com/user?access_token={}", token);
//     let response = reqwest::get(&url).await?;
//     let response_text = response.text().await?;

//     let github_user_data: GithubUserData = serde_json::from_str(&response_text)?;
//     println!("GitHub User Data: {:?}", github_user_data);
//     Ok(github_user_data)
// }
