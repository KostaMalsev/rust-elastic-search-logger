use serde_json::json;
use elasticsearch::{
    Elasticsearch, Error,
    http::transport::Transport,IndexParts
};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let transport = Transport::single_node("http://host.docker.internal:9200")?;
    let client = Elasticsearch::new(transport);

    let response = client
    .index(IndexParts::IndexId("tweets", "1"))
    .body(json!({
        "id": 1,
        "user": "kimchy",
        "post_date": "2009-11-15T00:00:00Z",
        "message": "Trying out Elasticsearch, so far so good?"
    }))
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
