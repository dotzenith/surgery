use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct SimpleTorrent {
    pub filename: String,
    pub links: Vec<String>,
}
