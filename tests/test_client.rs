use connectedpapers::ConnectedPapersClient;
use connectedpapers::GraphResponseStatuses;

const DEEPFRUITS_PAPER_ID: &str = "9397e7acd062245d37350f5c05faf56e9cfae0d6";

#[tokio::test]
async fn test_get_paper() {
    let client = ConnectedPapersClient::new(); // Assuming a method to create a new client
    let response = client
        .get_graph(DEEPFRUITS_PAPER_ID.into(), true)
        .await
        .unwrap(); // Assuming get_graph is async and returns Result

    assert_eq!(response.status, GraphResponseStatuses::FreshGraph);
    assert!(response.graph_json.is_some());
    let graph = response.graph_json.unwrap();
    assert_eq!(graph.start_id, DEEPFRUITS_PAPER_ID.to_string());
}
