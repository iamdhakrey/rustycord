use rustycord::{
    bot::Bot,
    client::Client,
    embeds::Embed,
    gateway::intents,
    handlers::message_handler::{MessageHandler, PingPongHandler},
    message::ChannelMessage,
};

use async_trait::async_trait;
use tokio;

// Custom message handler example
struct CustomHandler;

#[async_trait]
impl MessageHandler for CustomHandler {
    async fn on_message_create(
        &self,
        message: &ChannelMessage,
        client: &Client,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Don't respond to bot messages
        if message.author.bot.unwrap_or(false) {
            return Ok(());
        }

        // Handle different commands
        if message.content.starts_with("!hello") {
            client
                .send_text_message(&message.channel_id, "Hello there! 👋")
                .await?;
        } else if message.content.to_lowercase().contains("hello") {
            client
                .send_text_message(&message.channel_id, "Hello! 👋 Nice to meet you!")
                .await?;
        } else if message.content.starts_with("!echo ") {
            let echo_text = &message.content[6..]; // Remove "!echo " prefix
            let response = format!("Echo: {}", echo_text);
            client
                .send_text_message(&message.channel_id, &response)
                .await?;
        } else if message.content.starts_with("!embed") {
            let embed = Embed::new()
                .title("Example Embed")
                .description("This is an example embed message!")
                .color(0x00ff00);

            client
                .send_embed_message(&message.channel_id, vec![embed])
                .await?;
        } else if message.content.starts_with("!info") {
            let info_message = format!(
                "**Message Info:**\n• Author: {}\n• Channel: {}\n• Content: {}",
                message.author.name, message.channel_id, message.content
            );
            client
                .send_text_message(&message.channel_id, &info_message)
                .await?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // Initialize logger - you can change this to control logging verbosity:
    // "trace" - Most detailed, shows everything including message contents
    // "debug" - Detailed information about operations and events
    // "info"  - Important events only (recommended for production)
    // "warn"  - Only warnings and errors
    // "error" - Only error messages
    rustycord::logger::setup_logger("debug".to_string()).expect("Failed to initialize logger");

    // Replace with your actual bot token
    let token = "Token".to_string();

    // Create bot with MESSAGE_CONTENT intent
    let intent = intents::ALL_INTENTS;
    let mut bot = Bot::builder(Some(intent)).await;

    // Login to get the client
    let user_info = bot.login(token.clone()).await;
    println!("🚀 Logged in as: {}", user_info.username);

    // Get the client to access event dispatcher
    if let Some(client) = &bot.client {
        let event_dispatcher = client.get_event_dispatcher();
        let message_handlers = event_dispatcher.get_message_handlers();

        // Add different types of message handlers
        message_handlers.add_handler(PingPongHandler).await;
        // message_handlers.add_handler(EchoMessageHandler).await; // Removed to avoid duplicate responses
        message_handlers.add_handler(CustomHandler).await;

        println!("📝 Message handlers registered!");
        println!("🔧 Available commands:");
        println!("  • ping - Responds with pong");
        println!("  • hello - Responds with greeting");
        println!("  • !hello - Greets the user");
        println!("  • !echo <text> - Echoes the text back");
        println!("  • !embed - Sends an embed message");
        println!("  • !info - Shows message information");
        println!("  • Other messages will be ignored (no auto-echo)");
    }

    // Connect to gateway (without calling login again)
    bot.connect(bot.intents, Some(true)).await;

    // Keep the bot running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
