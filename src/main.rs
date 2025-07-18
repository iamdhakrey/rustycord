// main.rs or your entry point

use rustycord::{client::Client, logger::setup_logger};

#[tokio::main]
async fn main() {
    // Initialize logging (optional)
    let _ = setup_logger("debug".to_string());

    // Create a new client
    let mut client = Client::new();

    // Login to Discord
    let token = "Token".to_string();
    client.login(token).await;

    // Fetch gateway information
    let gateway_url = client.get_gateway().await;
    log::info!("Gateway URL: {}", gateway_url);

    // Determine the number of shards
    // You can fetch this from the gateway/bot endpoint or set it manually
    let total_shards = 2; // Example: 2 shards

    // Initialize shards
    client.initialize_shards(total_shards).await;

    // Start shards
    client.start_shards(Some(513), None).await;

    // Keep the main task alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
