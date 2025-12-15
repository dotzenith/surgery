use crate::data::User;
use anyhow::{Context, Result};
use std::env;
use ureq::Agent;
use url::Url;

pub struct RDClient {
    base_url: String,
    api_key: String,
    agent: Agent,
}

impl RDClient {
    pub fn new() -> Result<Self> {
        let base_url: String = env::var("RD_BASE_URL").context("RD Base URL not set")?;
        let api_key: String = env::var("RD_API_KEY").context("RD API Key not set")?;
        let agent: Agent = Agent::new_with_defaults();

        Ok(RDClient {
            base_url,
            api_key,
            agent,
        })
    }

    pub fn get_username(&self) -> Result<String> {
        let request_url = Url::parse(&format!("{}/user", self.base_url,))?;
        let response: User = self
            .agent
            .get(request_url.as_ref())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .call()?
            .body_mut()
            .read_json()?;

        Ok(response.username)
    }
}
