use futures::stream;
use futures::stream::Stream;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use tokio::time::sleep;

mod graph;
use graph::{Graph, PaperID};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GraphResponseStatuses {
    BadId,
    Error,
    NotInDb,
    OldGraph,
    FreshGraph,
    InProgress,
    Queued,
    BadToken,
    BadRequest,
    OutOfRequests,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphResponse {
    status: GraphResponseStatuses,
    graph_json: Option<Graph>,
    progress: Option<f64>,
}

static SLEEP_TIME_BETWEEN_CHECKS: u64 = 1000;
static SLEEP_TIME_AFTER_ERROR: u64 = 5000;

pub struct ConnectedPapersClient {
    access_token: String,
    server_addr: String,
    client: Client,
}

impl ConnectedPapersClient {
    pub fn build(access_token: impl Into<String>, server_addr: impl Into<String>) -> Self {
        ConnectedPapersClient {
            access_token: access_token.into(),
            server_addr: server_addr.into(),
            client: Client::new(),
        }
    }

    // TODO: Implementing a default would be a better choice here.

    pub fn new() -> Self {
        let server_addr = env::var("CONNECTED_PAPERS_REST_API")
            .unwrap_or_else(|_| "https://api.connectedpapers.com".to_string());

        let default_token = "TEST_TOKEN";
        let access_token =
            env::var("CONNECTED_PAPERS_API_KEY").unwrap_or_else(|_| default_token.to_string());

        ConnectedPapersClient {
            access_token,
            server_addr,
            client: Client::new(),
        }
    }

    pub async fn get_remaining_usages(
        &self,
    ) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .get(format!("{}/papers-api/remaining-usages", self.server_addr))
            .header("X-Api-Key", &self.access_token)
            .send()
            .await?;

        if response.status().is_success() {
            let data = response.json::<serde_json::Value>().await?;
            if let Some(remaining_uses) = data["remaining_uses"].as_i64() {
                Ok(remaining_uses as i32)
            } else {
                Err("The 'remaining_uses' field is missing or not an integer".into())
            }
        } else {
            Err(format!("Bad response: {}", response.status()).into())
        }
    }

    pub async fn get_free_access_papers(
        &self,
    ) -> Result<Vec<PaperID>, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .get(format!(
                "{}/papers-api/free-access-papers",
                self.server_addr
            ))
            .header("X-Api-Key", &self.access_token)
            .send()
            .await?;

        if response.status().is_success() {
            let data = response.json::<serde_json::Value>().await?;
            let paper_ids = data["papers"]
                .as_array()
                .ok_or("Expected an array for 'papers'")?
                .iter()
                .filter_map(|p| p.as_str().map(String::from))
                .collect::<Vec<PaperID>>();
            Ok(paper_ids)
        } else {
            Err(format!("Bad response: {}", response.status()).into())
        }
    }
}
