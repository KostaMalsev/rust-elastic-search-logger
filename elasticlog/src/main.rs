use serde_json::json;
use elasticsearch::{
    Elasticsearch, Error,
    http::transport::Transport,
    IndexParts,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get Elasticsearch host from environment variable or use a default
    let es_host = env::var("ELASTICSEARCH_HOST")
        .unwrap_or_else(|_| "http://localhost:9200".to_string());

    let es_user = env::var("ELASTICSEARCH_HOST")
        .unwrap_or_else(|_| "http://localhost:9200".to_string());

    println!("Connecting to Elasticsearch at: {}", es_host);

    // Create the transport with a more robust error handling
    let transport = Transport::single_node(&es_host)
        .map_err(|e| format!("Failed to create transport: {}", e))?;

    // Create the client
    let client = Elasticsearch::new(transport);

    // Prepare the document to be indexed
    let doc = json!({
        "id": 1,
        "user": "kimchy",
        "post_date": "2009-11-15T00:00:00Z",
        "message": "Trying out Elasticsearch, so far so good?"
    });

    // Perform the indexing operation
    let response = client
        .index(IndexParts::IndexId("tweets", "1"))
        .body(doc)
        .send()
        .await?;

    // Check if the operation was successful
    if response.status_code().is_success() {
        println!("Document indexed successfully");
    } else {
        eprintln!("Failed to index document. Status: {}", response.status_code());
        // You might want to log or handle the response body for more details
        println!("Response body: {:?}", response.text().await?);
    }

    Ok(())
}