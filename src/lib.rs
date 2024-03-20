use std::env;
use futures::stream;
use futures::StreamExt;
use futures::stream::BoxStream;
use reqwest::Client;
use serde::{Deserialize, Serialize};
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
        let access_token = env::var("CONNECTED_PAPERS_API_KEY")
                .unwrap_or_else(|_| default_token.to_string());

        ConnectedPapersClient {
            access_token,
            server_addr,
            client: Client::new(),
        }
    }

   pub fn get_graph_async_iterator(
        &self,
        paper_id: PaperID,
        fresh_only: bool,
        loop_until_fresh: bool,
    ) -> BoxStream<'static, Result<GraphResponse, Box<dyn std::error::Error + Send + Sync>>> {
        let client = self.client.clone();
        let access_token = self.access_token.clone();
        let server_addr = self.server_addr.clone();

        stream::unfold(0, move |retry_counter| {
            let client = client.clone();
            let access_token = access_token.clone();
            let server_addr = server_addr.clone();
            let paper_id = paper_id.clone();

            async move {
                if retry_counter >= 3 {
                    return None;
                }

                let response = client
                    .get(format!("{}/papers-api/graph/{}/{}", server_addr, fresh_only as u8, paper_id))
                    .header("X-Api-Key", &access_token)
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.status().is_success() => {
                        let graph_response: Result<GraphResponse, _> = resp.json().await;
                        match graph_response {
                            Ok(data) if data.status == GraphResponseStatuses::FreshGraph || !loop_until_fresh => {
                                Some((Ok(data), retry_counter + 1))
                            },
                            Ok(_) => {
                                sleep(Duration::from_millis(SLEEP_TIME_BETWEEN_CHECKS)).await;
                                Some((Err("Graph not fresh".into()), retry_counter + 1))
                            },
                            Err(e) => Some((Err(e.into()), retry_counter + 1)),
                        }
                    },
                    _ => {
                        sleep(Duration::from_millis(SLEEP_TIME_AFTER_ERROR)).await; // Sleep before retrying after error
                        Some((Err("Failed to fetch graph".into()), retry_counter + 1))
                    },
                }
            }
        }).boxed()
    }

    // pub async fn get_graph(
    //     &self,
    //     paper_id: PaperID,
    //     fresh_only: bool,
    //     loop_until_fresh: bool,
    // ) -> Result<GraphResponse, Box<dyn std::error::Error + Send + Sync>> {
    //     let mut graph_stream = self.get_graph_async_iterator(paper_id, fresh_only, loop_until_fresh);
    //     let mut last_response: GraphResponse;
    //
    //
    //     while let Some(result) = graph_stream.next().await {
    //         match result {
    //             Ok(graph_response) => {
    //                 last_response = graph_response;
    //             },
    //             Err(e) => Err(e)
    //         }
    //     }
    //
    //     // If the stream ends without a successful response, return an error
    //     Err("Failed to fetch a valid graph".into())
    // }

    pub async fn get_remaining_usages(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let response = self.client
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
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Bad response: {}", response.status()),
            )))
        }
    }

    pub async fn get_free_access_papers(&self) -> Result<Vec<PaperID>,  Box<dyn std::error::Error>> {
        let response = self.client
            .get(format!("{}/papers-api/free-access-papers", self.server_addr))
            .header("X-Api-Key", &self.access_token)
            .send()
            .await?;

        if response.status().is_success() {
            let data = response.json::<serde_json::Value>().await?;
            let paper_ids = data["papers"].as_array()
                .ok_or("Expected an array for 'papers'")?
                .iter()
                .filter_map(|p| p.as_str().map(String::from))
                .collect::<Vec<PaperID>>();
            Ok(paper_ids)

        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Bad response: {}", response.status()),
            )))
        }
    }

}

