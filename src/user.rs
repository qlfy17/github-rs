use async_trait::async_trait;
use oauth2::AccessToken;
use serde::Deserialize;

use crate::{client::GithubClient, Result};

#[derive(Debug, Deserialize)]
pub struct User {
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub id: i32,
    pub login: String,
    pub name: Option<String>,
}

impl GithubClient {
    pub async fn current_user(&self, auth: &AccessToken) -> Result<User> {
        self.request("/user", auth).await
    }
}
