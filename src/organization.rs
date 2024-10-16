use async_trait::async_trait;
use oauth2::AccessToken;
use serde::Deserialize;

use crate::{client::GithubClient, Result};

#[derive(Debug, Deserialize)]
pub struct Organization {
    pub id: i32,
    pub avatar_url: Option<String>,
}

impl GithubClient {
    pub async fn org_by_name(&self, org_name: &str, auth: &AccessToken) -> Result<Organization> {
        let url = format!("orgs/{org_name}");
        self.request(&url, auth).await
    }
}
