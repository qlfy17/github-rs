use serde::{Deserialize, Serialize};

use crate::{
    client::{Client, IntoRequest},
    Result,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub id: i32,
    pub login: String,
    pub name: Option<String>,
}

impl IntoRequest for UserRequest {
    fn into_request(self, client: reqwest::Client) -> reqwest::RequestBuilder {
        client.get("/user")
    }
}

impl UserRequest {
    pub fn new() -> Self {
        UserRequestBuilder::default().build()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserRequestBuilder {}

impl UserRequestBuilder {
    pub fn build(&self) -> UserRequest {
        UserRequest {}
    }
}

impl Client {
    pub async fn current_user(&self, req: UserRequest) -> Result<UserResponse> {
        let req = self.new_request(req);
        Ok(self.send::<UserResponse>(req).await?)
    }
}

#[test]
async fn current_user_should_work() -> Result<()> {
    let client = Client::new(None);
    client
        .current_user(UserRequestBuilder::build(&self))
        .await?
}
