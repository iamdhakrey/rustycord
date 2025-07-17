# Installation

!!! danger "Development Version Warning"
    **rustycord is currently in heavy development and is NOT ready for production use.**
    
    - 🚨 **Breaking changes occur frequently**
    - 🚨 **APIs are unstable and will change**
    - 🚨 **Features are incomplete**
    - 🚨 **Not suitable for production bots**
    
    **Only use for learning, experimentation, and development.** Do not deploy bots using this library to production environments.

!!! note "Development Status"
    This library is actively being developed. While basic functionality works, many features are missing or incomplete. Check the [GitHub repository](https://github.com/iamdhakrey/rustycord) for the latest status.

## Prerequisites

Before installing rustycord, make sure you have:

- **Rust 1.70.0 or later** - [Install Rust](https://rustup.rs/)
- **Git** - For cloning repositories
- **Discord Bot Token** - [Create a Discord Application](https://discord.com/developers/applications)

## Installing Rust

If you don't have Rust installed, you can install it using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Verify your installation:

```bash
rustc --version
cargo --version
```

## Creating a New Project

Create a new Rust project for your Discord bot:

```bash
cargo new my-discord-bot
cd my-discord-bot
```

## Adding rustycord Dependency

Add rustycord and required dependencies to your `Cargo.toml`:

```toml
[package]
name = "my-discord-bot"
version = "0.1.0"
edition = "2021"

[dependencies]
rustycord = "0.1.0"
tokio = { version = "1.40", features = ["full"] }
async-trait = "0.1"
log = "0.4"
```

## Setting Up Your Discord Bot

### 1. Create a Discord Application

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications)
2. Click "New Application"
3. Give your application a name
4. Navigate to the "Bot" section
5. Click "Add Bot"
6. Copy your bot token (keep this secret!)

### 2. Configure Bot Permissions

In the "Bot" section of your application:

1. Enable the following **Privileged Gateway Intents**:
   - Message Content Intent (if you need to read message content)
   - Server Members Intent (if you need member information)
   - Presence Intent (if you need presence information)

2. In the "OAuth2" > "URL Generator" section:
   - Select "bot" scope
   - Select required permissions:
     - Send Messages
     - Read Message History
     - Use Slash Commands (if needed)

3. Use the generated URL to invite your bot to a server

### 3. Set Up Environment Variables

Create a `.env` file in your project root:

```env
DISCORD_TOKEN=your_bot_token_here
```

Add `.env` to your `.gitignore`:

```gitignore
.env
target/
Cargo.lock
```

## Verifying Installation

Create a simple test bot in `src/main.rs`:

```rust
use rustycord::{Bot, Client, logger};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Initialize logging
    logger::setup_logger(logger::LogLevel::Info)?;
    
    // Get bot token
    let token = std::env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN environment variable not set");
    
    // Create client and bot
    println!("Creating Discord client...");
    let client = Client::new(&token).await?;
    
    println!("Starting bot...");
    let bot = Bot::new(client);
    bot.start().await?;
    
    Ok(())
}
```

Add the `dotenv` dependency to your `Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies
dotenv = "0.15"
```

Run your bot:

```bash
cargo run
```

You should see log output indicating your bot has connected to Discord!

## Next Steps

Now that you have rustycord installed and a basic bot running:

- [Create Your First Bot](first-bot.md) - Build a simple echo bot
- [Bot Configuration](configuration.md) - Learn about bot settings
- [Message Handlers](../user-guide/message-handlers.md) - Handle user messages

## Troubleshooting

### Common Issues

**Bot token is invalid:**
```
Error: HTTP error: 401 Unauthorized
```
- Verify your bot token is correct
- Make sure there are no extra spaces in your `.env` file

**Permission denied:**
```
Error: Missing Access
```
- Check that your bot has the required permissions in the Discord server
- Verify the bot is properly invited to the server

**Connection timeout:**
```
Error: Connection timeout
```
- Check your internet connection
- Verify Discord's status at [discordstatus.com](https://discordstatus.com)

### Getting Help

If you encounter issues:

1. Check the [troubleshooting guide](../troubleshooting.md)
2. Search [GitHub Issues](https://github.com/your-username/rustycord/issues)
3. Create a new issue with:
   - Your Rust version (`rustc --version`)
   - Your operating system
   - Complete error messages
   - Minimal reproduction code
