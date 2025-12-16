use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;
use ureq::Agent;
use url::Url;

#[derive(Clone, Deserialize, Debug)]
pub struct SimpleTorrent {
    pub filename: String,
    pub links: Vec<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct UnrestrictedLink {
    pub filename: String,
    pub download: String,
}

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

    pub fn get_torrents(&self) -> Result<Vec<SimpleTorrent>> {
        let request_url = Url::parse(&format!("{}/torrents", self.base_url))?;
        let response: Vec<SimpleTorrent> = self
            .agent
            .get(request_url.as_ref())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .call()?
            .body_mut()
            .read_json()?;

        Ok(response)
    }

    pub fn unrestrict_link(&self, link: &str) -> Result<UnrestrictedLink> {
        let form = [("link", link)];

        let request_url = Url::parse(&format!("{}/unrestrict/link", self.base_url))?;
        let response: UnrestrictedLink = self
            .agent
            .post(request_url.as_ref())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send_form(form)?
            .body_mut()
            .read_json()?;

        Ok(response)
    }
}
