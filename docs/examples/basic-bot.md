# Basic Bot Example

This example demonstrates how to create a simple Discord bot using rustycord.

## Prerequisites

- Rust 1.70 or later
- A Discord application and bot token

## Code

```rust
use rustycord::{Bot, Client, MessageHandler, logger};
use rustycord::models::Message;
use async_trait::async_trait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with info level
    logger::setup_logger("info".to_string())?;
    
    // Get bot token from environment variable
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Expected DISCORD_TOKEN environment variable");
    
    // Create a new bot instance
    let mut bot = Bot::new(None).await;
    
    // Login the bot
    let user_response = bot.login(token).await;
    println!("Bot logged in as: {:?}", user_response.username);
    
    // Register message handlers
    bot.register_message_handler(Box::new(PingHandler));
    bot.register_message_handler(Box::new(EchoHandler));
    
    // Start the bot (this will block until the bot is stopped)
    bot.start().await?;
    
    Ok(())
}

/// A simple ping-pong handler
struct PingHandler;

#[async_trait]
impl MessageHandler for PingHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if message.content == "!ping" {
            Ok(Some("Pong! 🏓".to_string()))
        } else {
            Ok(None)
        }
    }
}

/// An echo handler that repeats user input
struct EchoHandler;

#[async_trait]
impl MessageHandler for EchoHandler {
    async fn handle_message(&self, message: &Message) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        if message.content.starts_with("!echo ") {
            let response = message.content.strip_prefix("!echo ").unwrap_or("");
            Ok(Some(format!("🔊 {}", response)))
        } else {
            Ok(None)
        }
    }
}
```

## Environment Setup

Create a `.env` file in your project root:

```bash
DISCORD_TOKEN=your_bot_token_here
```

## Running the Bot

1. Set up your environment variable:
   ```bash
   export DISCORD_TOKEN="your_bot_token_here"
   ```

2. Run the bot:
   ```bash
   cargo run
   ```

## Features Demonstrated

- **Bot Initialization**: Creating and configuring a bot instance
- **Login**: Authenticating with Discord
- **Message Handlers**: Responding to specific commands
- **Logging**: Setting up structured logging
- **Async Operations**: Using Rust's async/await for non-blocking operations

## Commands

Once the bot is running, you can test these commands in your Discord server:

- `!ping` - Bot responds with "Pong! 🏓"
- `!echo <message>` - Bot echoes your message with a speaker emoji

## Next Steps

- [Message Handler Guide](../user-guide/prefix-commands.md) - Learn more about message handling
- [Embeds Example](embeds.md) - Add rich embed responses
- [Logging Example](logging.md) - Advanced logging configuration
