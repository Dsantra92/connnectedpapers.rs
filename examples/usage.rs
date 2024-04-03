use connectedpapers::ConnectedPapersClient;
use tokio;

const DEEPFRUITS_PAPER_ID: &str = "9397e7acd062245d37350f5c05faf56e9cfae0d6";

#[tokio::main]
async fn main() {
    // if you have the API key stored in .env file
    // dotenv().ok();

    let client = ConnectedPapersClient::new();
    let remaining_usages = client.get_remaining_usages().await.unwrap();
    println!("Remaining usage: {remaining_usages}");

    let free_access_papers = client.get_free_access_papers().await.unwrap();
    println!("Free access papers: {:?}", free_access_papers);

    let graph = client
        .get_graph(DEEPFRUITS_PAPER_ID.into(), true)
        .await
        .unwrap();
    assert!(graph.graph_json.unwrap().start_id == DEEPFRUITS_PAPER_ID);
}
