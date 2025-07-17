use rustycord::{bot::Bot, gateway::intents};
use tokio;

#[tokio::main]
async fn main() {
    // Initialize logging - Users can set this to "trace", "debug", "info", "warn", or "error"
    // "debug" will show detailed information about what rustycord is doing
    // "info" is recommended for production (shows important events only)
    // "trace" shows the most detailed logging including message contents

    let token = "Token".to_string();

    // Create bot with MESSAGE_CONTENT intent
    let intent = intents::ALL_INTENTS;

    let mut bot = Bot::builder(Some(intent)).await;

    // Start the bot with debug logging enabled
    // Change "debug" to "info" for production use
    bot.run(token, Some("debug".to_string())).await;
}
