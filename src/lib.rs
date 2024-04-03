use std::pin::Pin;
use futures::stream;
use futures::stream::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json,Value};
use std::env;
use std::time::Duration;
use tokio::time::sleep;

mod graph;

use graph::{Graph, PaperID};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphResponse {
    pub status: GraphResponseStatuses,
    pub graph_json: Option<Graph>,
    pub progress: Option<f64>,
}

static SLEEP_TIME_BETWEEN_CHECKS: u64 = 1000;
static SLEEP_TIME_AFTER_ERROR: u64 = 5000;

fn is_end_status(status: &GraphResponseStatuses) -> bool {
    matches!(
        status,
        GraphResponseStatuses::BadId
            | GraphResponseStatuses::Error
            | GraphResponseStatuses::NotInDb
            | GraphResponseStatuses::FreshGraph
            | GraphResponseStatuses::BadToken
            | GraphResponseStatuses::BadRequest
            | GraphResponseStatuses::OutOfRequests
    )
}

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

    pub fn get_graph_async_iterator(
        &self,
        paper_id: String,
        initial_fresh_only: bool,
        loop_until_fresh: bool,
    ) -> Pin<Box<dyn Stream<Item = Result<GraphResponse, Box<dyn std::error::Error + Send + Sync>>>>> {

        // TODO: Clean up the borrowing mess
        let client  = self.client.clone();
        let server_addr = self.server_addr.clone();
        let access_token = self.access_token.clone();
        let paper_id = paper_id.clone();
        Box::pin(stream::unfold((1, None, None::<Box<dyn std::error::Error + Send + Sync>>, initial_fresh_only), move |(mut retry_counter, mut newest_graph, mut last_error, mut fresh_only)| {
            let client = client.clone();
            let paper_id_cloned = paper_id.clone();
            let server_addr_cloned = server_addr.clone();
            let access_token_cloned = access_token.clone();

            async move {
                while retry_counter > 0 {
                    let response = client
                        .get(format!("{}/papers-api/graph/{}/{}", server_addr_cloned, fresh_only as i32, paper_id_cloned))
                        .header("X-Api-Key", &access_token_cloned)
                        .send()
                        .await;

                    match response {
                        Ok(resp) if resp.status() == 200 => {
                            match resp.json::<Value>().await {
                                Ok(mut body) => {
                                    // Attempt to manually parse and modify the "status" field
                                    let status = body.get("status")
                                                        .and_then(|v| serde_json::from_value::<GraphResponseStatuses>(v.clone()).ok());

                                    if status.is_none() { body["status"] = json!("ERROR"); }
                                    fresh_only = true;
                                    match serde_json::from_value::<GraphResponse>(body) {
                                        Ok(mut data) => {
                                            if data.graph_json.is_some() {
                                                newest_graph = data.graph_json.clone();
                                            }

                                            if is_end_status(&data.status) || !loop_until_fresh {
                                                // Successful or made a wrong request
                                                return Some((Ok(data), (0, newest_graph, None, fresh_only)))
                                            }
                                            data.graph_json = newest_graph.clone();
                                            return Some((Ok(data), (retry_counter, newest_graph, None, fresh_only)));

                                        }
                                        Err(e) => last_error = Some(e.into())
                                    }
                                },
                                Err(e) => last_error = Some(e.into())
                            }
                        },
                        Ok(resp) => {
                            // Handle non-200 status codes without stopping the entire process
                            let error_message = format!("Bad response: {}", resp.status());
                            last_error = Some(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)));
                        },
                        Err(e) => last_error = Some(e.into()),
                    }
                    if last_error.is_some() {
                        retry_counter -= 1;

                        if retry_counter > 0 {
                            sleep(Duration::from_millis(SLEEP_TIME_AFTER_ERROR)).await; // Sleep after an error before retrying
                        }
                    }
                    else{
                        sleep(Duration::from_millis(SLEEP_TIME_BETWEEN_CHECKS)).await; // Sleep after an error before retrying
                    }
                }

                if let Some(error) = last_error {
                    // Return the last error if all retries have been exhausted
                    Some((Err(error), (0, newest_graph, None, true)))
                } else {
                    None // End of stream if no data could be fetched and retries are exhausted
                }
            }
        }))
    }

    pub async fn get_graph(
        &self,
        paper_id: String,
        fresh_only: bool,
    ) -> Result<GraphResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut result = GraphResponse {
            status: GraphResponseStatuses::Error,
            graph_json: None,
            progress: None,
        };

        let mut stream = self.get_graph_async_iterator(paper_id, fresh_only, true);

        // Consume the stream until completion, updating `result` with each received item
        while let Some(response) = stream.next().await {
            match response {
                Ok(data) => {
                    result = data;
                },
                Err(e) => return Err(e),
            }
        }
        Ok(result)
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
