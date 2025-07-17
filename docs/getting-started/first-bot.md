# Your First Bot

This guide will walk you through creating your first Discord bot with RustCord, from setup to deployment.

## Prerequisites

Make sure you have completed the [installation guide](installation.md) before proceeding.

## Creating a Simple Echo Bot

Let's start with a basic bot that echoes messages back to users.

### Step 1: Set Up Your Project

```bash
cargo new my-first-bot
cd my-first-bot
```

### Step 2: Add Dependencies

Edit your `Cargo.toml`:

```toml
[package]
name = "my-first-bot"
version = "0.1.0"
edition = "2021"

[dependencies]
rustcord = "0.1.0"
tokio = { version = "1.40", features = ["full"] }
async-trait = "0.1"
dotenv = "0.15"
```

### Step 3: Create Your Bot

Replace the contents of `src/main.rs`:

```rust
use async_trait::async_trait;
use rustcord::{
    bot::BotBase,
    handlers::message_handler::MessageHandler,
    message::ChannelMessage,
    client::Client,
    logger
};

// Simple echo handler
struct EchoHandler;

#[async_trait]
impl MessageHandler for EchoHandler {
    async fn on_message_create(
        &self, 
        message: &ChannelMessage, 
        client: &Client
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Don't respond to bot messages (avoid infinite loops!)
        if message.author.bot.unwrap_or(false) {
            return Ok(());
        }
        
        // Only respond to messages that start with "!echo"
        if message.content.starts_with("!echo ") {
            let echo_text = &message.content[6..]; // Remove "!echo " prefix
            client.send_text_message(&message.channel_id, echo_text).await?;
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Initialize logging
    logger::setup_logger("info".to_string())?;
    
    // Get bot token
    let token = std::env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN environment variable not set");
    
    println!("🚀 Starting your first Discord bot!");
    
    // Create and login bot
    let mut bot = BotBase::new(None).await;
    let user_info = bot.login(token).await;
    println!("🔑 Logged in as: {}", user_info.username);
    
    // Register message handler
    if let Some(client) = &bot.client {
        let event_dispatcher = client.get_event_dispatcher();
        let message_handlers = event_dispatcher.get_message_handlers();
        
        message_handlers.add_handler(EchoHandler).await;
        println!("📝 Echo handler registered!");
    }
    
    println!("🤖 Bot is running! Try typing '!echo Hello World' in Discord");
    
    // Connect to Discord
    bot.connect(bot.intents, Some(true)).await;
    
    // Keep the bot running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
```

### Step 4: Set Up Environment Variables

Create a `.env` file:

```env
DISCORD_TOKEN=your_bot_token_here
```

Remember to replace `your_bot_token_here` with your actual bot token from the Discord Developer Portal.

### Step 5: Run Your Bot

```bash
cargo run
```

You should see output like:

```
🚀 Starting your first Discord bot!
🔑 Logged in as: YourBot#1234
📝 Echo handler registered!
🤖 Bot is running! Try typing '!echo Hello World' in Discord
```

### Step 6: Test Your Bot

1. Go to your Discord server where you invited the bot
2. Type: `!echo Hello World`
3. Your bot should respond with: `Hello World`

## Adding More Commands

Let's enhance our bot with multiple commands using the prefix system:

```rust
use async_trait::async_trait;
use rustcord::{
    bot::BotBase,
    handlers::message_handler::MessageHandler,
    prefix::{PrefixListener, PrefixCommand, HelpCommand, PingCommand},
    message::ChannelMessage,
    client::Client,
    logger
};
use std::sync::Arc;

// Custom greeting command
struct GreetCommand;

#[async_trait]
impl PrefixCommand for GreetCommand {
    async fn execute(&self, message: &ChannelMessage, args: Vec<&str>) 
        -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> 
    {
        let name = if args.is_empty() {
            &message.author.name
        } else {
            args[0]
        };
        
        Ok(Some(format!("Hello, {}! 👋 Welcome to the server!", name)))
    }
    
    fn description(&self) -> &str {
        "Greet someone (or yourself if no name provided)"
    }
}

// Message handler using prefix system
struct PrefixHandler {
    listener: Arc<PrefixListener>,
}

impl PrefixHandler {
    fn new(listener: Arc<PrefixListener>) -> Self {
        Self { listener }
    }
}

#[async_trait]
impl MessageHandler for PrefixHandler {
    async fn on_message_create(
        &self, 
        message: &ChannelMessage, 
        client: &Client
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Skip bot messages
        if message.author.bot.unwrap_or(false) {
            return Ok(());
        }
        
        // Handle prefix commands
        if let Some(response) = self.listener.handle_message(message).await? {
            client.send_text_message(&message.channel_id, &response).await?;
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    logger::setup_logger("info".to_string())?;
    
    let token = std::env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN environment variable not set");
    
    println!("🚀 Starting enhanced Discord bot!");
    
    // Create bot
    let mut bot = BotBase::new(None).await;
    let user_info = bot.login(token).await;
    println!("🔑 Logged in as: {}", user_info.username);
    
    // Set up prefix system
    let listener = Arc::new(PrefixListener::new("!"));
    
    // Register commands
    listener.register_command("help", Box::new(HelpCommand::new(listener.clone()))).await;
    listener.register_command("ping", Box::new(PingCommand)).await;
    listener.register_command("greet", Box::new(GreetCommand)).await;
    
    // Register message handler
    if let Some(client) = &bot.client {
        let event_dispatcher = client.get_event_dispatcher();
        let message_handlers = event_dispatcher.get_message_handlers();
        
        message_handlers.add_handler(PrefixHandler::new(listener)).await;
        println!("📝 Prefix handler registered!");
    }
    
    println!("🤖 Enhanced bot is running! Available commands:");
    println!("  • !help - Show all commands");
    println!("  • !ping - Test bot responsiveness");
    println!("  • !greet [name] - Greet someone");
    
    // Connect and run
    bot.connect(bot.intents, Some(true)).await;
    
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
```

Now your bot supports:
- `!help` - Lists all available commands
- `!ping` - Responds with "Pong! 🏓"
- `!greet` - Greets you
- `!greet Alice` - Greets Alice

## Common Issues and Solutions

### Bot Token Invalid
```
Error: HTTP error: 401 Unauthorized
```
**Solution:** Check your bot token in the `.env` file and ensure it's correct.

### Bot Not Responding
```
Bot connects but doesn't respond to commands
```
**Solutions:**
1. Ensure your bot has "Send Messages" permission
2. Check that Message Content Intent is enabled in Discord Developer Portal
3. Verify the bot is invited to the server

### Permission Denied
```
Error: Missing Access
```
**Solution:** Re-invite your bot with proper permissions using the OAuth2 URL generator.

## Next Steps

Now that you have a working bot:

1. **Add More Commands** - Follow the [prefix commands guide](../user-guide/prefix-commands.md)
2. **Add Rich Embeds** - Learn about [embed messages](../user-guide/embeds.md)
3. **Handle Events** - Explore [event handling](../user-guide/events.md)
4. **Deploy Your Bot** - Check out [deployment options](../deployment/hosting.md)

## Complete Example Repository

You can find complete working examples in the [examples directory](../examples/) of the RustCord repository.

## Getting Help

- Check the [troubleshooting guide](../troubleshooting.md)
- Browse [API documentation](../api/)
- Ask questions in [GitHub Discussions](https://github.com/your-username/rustcord/discussions)
