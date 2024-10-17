use std::time::Duration;

use oauth2::AccessToken;
use reqwest::{header, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use tracing::info;

use crate::Result;

#[derive(Debug)]
pub struct Client {
    // base_url: String,
    client: reqwest::Client,
}

// #[async_trait]
// pub trait Client: Send + Sync {
//     async fn current_user(&self, auth: &AccessToken) -> Result<User>;
//     async fn org_by_name(&self, org_name: &str, auth: &AccessToken) -> Result<Organization>;
// }

pub trait IntoRequest {
    fn into_request(self, client: reqwest::Client) -> RequestBuilder;
}

impl Client {
    pub fn new(client: Option<reqwest::Client>) -> Self {
        match client {
            Some(client) => Self { client },
            None => Self {
                client: reqwest::Client::new(),
            },
        }
    }

    pub fn new_request(&self, req: impl IntoRequest) -> RequestBuilder {
        let req = req.into_request(self.client.clone());
        // req.bearer_auth(&self.token);
        req.header(header::ACCEPT, "application/vnd.github.v3+json")
            .header(header::USER_AGENT, "crates.io (https://crates.io)")
            .timeout(Duration::from_secs(10))
    }

    pub async fn send<T>(&self, req: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        req.send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await
            .map_err(Into::into)
    }

    async fn _request<T>(&self, url: &str, auth: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("https://api.github.com{url}");
        info!("GITHUB HTTP: {url}");

        self.client
            .get(url)
            .header(header::ACCEPT, "application/vnd.github.v3+json")
            .header(header::AUTHORIZATION, auth)
            .header(header::USER_AGENT, "crates.io (https://crates.io)")
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await
            .map_err(Into::into)
    }

    pub async fn request<T>(&self, url: &str, auth: &AccessToken) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self._request(url, &format!("Bearer {}", auth.secret()))
            .await
    }

    pub async fn request_basic<T>(&self, url: &str, username: &str, password: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self._request(url, &format!("basic {username}:{password}"))
            .await
    }
}
