use rustcord::{bot::Bot, gateway::intents};

use tokio;

#[tokio::main]
async fn main() {
    let token =
        "ODM1NzMzMDE2NDY2NDIzODY5.GBaGBM.9tF_H_7FhXO2jPFKDESMmwX1ytvJWYH8mCSbCM".to_string();

    // new bot implementation using Bot struct
    let intent = intents::ALL_INTENTS;

    Bot::builder(Some(intent))
            .await
            .run(token, Some("info".to_string()))
            .await;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
