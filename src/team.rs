use oauth2::AccessToken;
use serde::Deserialize;

use crate::{client::Client, organization::Organization, Result};

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: i32,
    pub name: Option<String>,
    pub organization: Organization,
}

impl Client {
    pub async fn team_by_name(
        &self,
        org_name: &str,
        team_name: &str,
        auth: &AccessToken,
    ) -> Result<Team> {
        let url = format!("/orgs/{org_name}/teams/{team_name}");
        self.request(&url, auth).await
    }
}
